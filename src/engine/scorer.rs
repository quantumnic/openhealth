use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosisResult {
    pub disease_name: String,
    pub probability: f64,
    pub matched_symptoms: Vec<String>,
    pub missing_key_symptoms: Vec<String>,
    pub severity: String,
    pub description: String,
}

/// Bayesian-inspired symptom scorer with specificity weighting.
/// For each disease, calculates a score based on:
/// - Weight of matched symptoms
/// - Whether primary symptoms are present
/// - Ratio of matched vs total disease symptoms
/// - Symptom specificity (rare symptoms count more)
pub fn score_symptoms(conn: &Connection, input_symptoms: &[&str]) -> Vec<DiagnosisResult> {
    let normalized: Vec<String> = input_symptoms
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if normalized.is_empty() {
        return vec![];
    }

    // Pre-compute symptom specificity: how many diseases share each symptom
    let symptom_disease_counts = get_symptom_disease_counts(conn);

    let mut results = Vec::new();

    let mut stmt = conn
        .prepare("SELECT id, name, description, severity FROM diseases")
        .unwrap();

    let diseases: Vec<(i64, String, String, String)> = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let total_diseases = diseases.len().max(1) as f64;

    for (disease_id, disease_name, description, severity) in &diseases {
        let mut symptom_stmt = conn
            .prepare(
                "SELECT s.name, ds.weight, ds.is_primary 
                 FROM disease_symptoms ds 
                 JOIN symptoms s ON s.id = ds.symptom_id 
                 WHERE ds.disease_id = ?1",
            )
            .unwrap();

        let disease_symptoms: Vec<(String, f64, bool)> = symptom_stmt
            .query_map([disease_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get::<_, i32>(2)? != 0))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        if disease_symptoms.is_empty() {
            continue;
        }

        let mut matched = Vec::new();
        let mut matched_weight_sum = 0.0;
        let mut total_weight_sum = 0.0;
        let mut primary_matched = 0;
        let mut primary_total = 0;
        let mut missing_primary = Vec::new();
        let mut specificity_bonus = 0.0;

        for (sym_name, weight, is_primary) in &disease_symptoms {
            total_weight_sum += weight;
            if *is_primary {
                primary_total += 1;
            }

            let sym_lower = sym_name.to_lowercase();
            let is_match = normalized.iter().any(|input| fuzzy_match(input, &sym_lower));

            if is_match {
                matched.push(sym_name.clone());
                matched_weight_sum += weight;
                if *is_primary {
                    primary_matched += 1;
                }
                // Specificity: symptoms shared by fewer diseases are more informative
                let disease_count = symptom_disease_counts
                    .get(&sym_lower)
                    .copied()
                    .unwrap_or(1) as f64;
                specificity_bonus += (total_diseases / disease_count).ln().max(0.0) * 0.05;
            } else if *is_primary {
                missing_primary.push(sym_name.clone());
            }
        }

        if matched.is_empty() {
            continue;
        }

        // Calculate probability score
        let weight_ratio = matched_weight_sum / total_weight_sum;
        let primary_bonus = if primary_total > 0 {
            (primary_matched as f64 / primary_total as f64) * 0.3
        } else {
            0.0
        };
        let coverage = matched.len() as f64 / disease_symptoms.len() as f64;

        // Precision penalty: if user gave many symptoms but only few matched,
        // the disease is less likely (user has symptoms that don't fit this disease).
        let input_count = normalized.len() as f64;
        let match_precision = matched.len() as f64 / input_count;
        let precision_factor = 0.5 + 0.5 * match_precision; // range [0.5, 1.0]

        // Combined: weight ratio (35%) + primary bonus (30%) + coverage (20%) + specificity (10%) + precision (5%)
        let raw_score =
            (weight_ratio * 0.35 + primary_bonus + coverage * 0.20 + specificity_bonus.min(0.1))
                * precision_factor;
        let probability = (raw_score * 100.0).clamp(1.0, 95.0);

        results.push(DiagnosisResult {
            disease_name: disease_name.clone(),
            probability,
            matched_symptoms: matched,
            missing_key_symptoms: missing_primary,
            severity: severity.clone(),
            description: description.clone(),
        });
    }

    results.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
    results
}

/// Count how many diseases each symptom appears in (for specificity calculation).
fn get_symptom_disease_counts(
    conn: &Connection,
) -> std::collections::HashMap<String, usize> {
    let mut map = std::collections::HashMap::new();
    let mut stmt = conn
        .prepare(
            "SELECT LOWER(s.name), COUNT(DISTINCT ds.disease_id) 
             FROM disease_symptoms ds 
             JOIN symptoms s ON s.id = ds.symptom_id 
             GROUP BY LOWER(s.name)",
        )
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, usize>(1)?))
        })
        .unwrap();
    for row in rows.flatten() {
        map.insert(row.0, row.1);
    }
    map
}

/// Fuzzy matching: checks if input contains or is contained in symptom name,
/// or if individual words overlap significantly.
fn fuzzy_match(input: &str, symptom: &str) -> bool {
    if input == symptom || symptom.contains(input) || input.contains(symptom) {
        return true;
    }

    let input_words: Vec<&str> = input.split_whitespace().collect();
    let symptom_words: Vec<&str> = symptom.split_whitespace().collect();

    for iw in &input_words {
        if iw.len() < 3 {
            continue;
        }
        for sw in &symptom_words {
            if sw.len() < 3 {
                continue;
            }
            if iw == sw || sw.contains(iw) || iw.contains(sw) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_malaria_symptoms() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fever", "chills", "sweating", "headache"]);
        assert!(!results.is_empty());
        let malaria = results.iter().find(|r| r.disease_name == "Malaria");
        assert!(malaria.is_some(), "Malaria should appear in results");
        assert!(malaria.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_empty_symptoms() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &[]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_score_unknown_symptom() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["alien_probe_marks"]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_score_heart_attack() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chest pain", "left arm pain", "cold sweat"]);
        let ha = results.iter().find(|r| r.disease_name == "Heart Attack");
        assert!(ha.is_some(), "Heart Attack should appear");
    }

    #[test]
    fn test_score_cholera() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["watery diarrhea", "vomiting", "dehydration"]);
        let cholera = results.iter().find(|r| r.disease_name == "Cholera");
        assert!(cholera.is_some());
        assert!(cholera.unwrap().probability > 40.0);
    }

    #[test]
    fn test_score_covid19() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fever", "cough", "loss of taste", "loss of smell"]);
        let covid = results.iter().find(|r| r.disease_name == "COVID-19");
        assert!(covid.is_some(), "COVID-19 should appear in results");
        assert!(covid.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_lyme_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["erythema migrans rash", "fatigue", "joint pain"]);
        let lyme = results.iter().find(|r| r.disease_name == "Lyme Disease");
        assert!(lyme.is_some(), "Lyme Disease should appear in results");
    }

    #[test]
    fn test_fuzzy_match_partial() {
        assert!(fuzzy_match("fever", "high fever"));
        assert!(fuzzy_match("headache", "severe headache"));
        assert!(!fuzzy_match("xyz", "fever"));
    }

    #[test]
    fn test_results_sorted_by_probability() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fever", "headache"]);
        for w in results.windows(2) {
            assert!(w[0].probability >= w[1].probability);
        }
    }

    #[test]
    fn test_probability_max_95() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(
            &conn,
            &[
                "fever",
                "chills",
                "sweating",
                "headache",
                "nausea",
                "vomiting",
                "muscle pain",
                "fatigue",
            ],
        );
        for r in &results {
            assert!(r.probability <= 95.0, "Probability should cap at 95%");
        }
    }

    #[test]
    fn test_matched_symptoms_populated() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fever", "cough"]);
        for r in &results {
            assert!(!r.matched_symptoms.is_empty());
        }
    }

    #[test]
    fn test_specificity_boosts_unique_symptoms() {
        let conn = db::init_memory_database().unwrap();
        // "hydrophobia" is very specific to rabies
        let results = score_symptoms(&conn, &["hydrophobia", "fever"]);
        if let Some(rabies) = results.iter().find(|r| r.disease_name == "Rabies") {
            assert!(
                rabies.probability > 20.0,
                "Specific symptom should boost probability"
            );
        }
    }
}
