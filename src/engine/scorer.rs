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

/// Bayesian-inspired symptom scorer.
/// For each disease, calculates a score based on:
/// - Weight of matched symptoms
/// - Whether primary symptoms are present
/// - Ratio of matched vs total disease symptoms
pub fn score_symptoms(conn: &Connection, input_symptoms: &[&str]) -> Vec<DiagnosisResult> {
    let normalized: Vec<String> = input_symptoms
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if normalized.is_empty() {
        return vec![];
    }

    let mut results = Vec::new();

    // Get all diseases
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

        for (sym_name, weight, is_primary) in &disease_symptoms {
            total_weight_sum += weight;
            if *is_primary {
                primary_total += 1;
            }

            let sym_lower = sym_name.to_lowercase();
            let is_match = normalized.iter().any(|input| {
                fuzzy_match(input, &sym_lower)
            });

            if is_match {
                matched.push(sym_name.clone());
                matched_weight_sum += weight;
                if *is_primary {
                    primary_matched += 1;
                }
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

        // Combined score: weight ratio (40%) + primary bonus (30%) + coverage (30%)
        let raw_score = weight_ratio * 0.4 + primary_bonus + coverage * 0.3;
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

/// Fuzzy matching: checks if input contains or is contained in symptom name,
/// or if individual words overlap significantly.
fn fuzzy_match(input: &str, symptom: &str) -> bool {
    if input == symptom || symptom.contains(input) || input.contains(symptom) {
        return true;
    }

    let input_words: Vec<&str> = input.split_whitespace().collect();
    let symptom_words: Vec<&str> = symptom.split_whitespace().collect();

    // Check if any significant word matches
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
        // Malaria should be near the top
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
        let results = score_symptoms(&conn, &["fever", "chills", "sweating", "headache", "nausea", "vomiting", "muscle pain", "fatigue"]);
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
}
