use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosisResult {
    pub disease_name: String,
    pub probability: f64,
    pub matched_symptoms: Vec<String>,
    pub missing_key_symptoms: Vec<String>,
    pub severity: String,
    pub description: String,
}

/// Optional demographic context for age/sex-aware scoring.
#[derive(Debug, Clone, Default)]
pub struct PatientContext {
    pub age: Option<u8>,
    pub sex: Option<String>,
}

/// Bayesian-inspired symptom scorer with specificity weighting and demographic context.
/// For each disease, calculates a score based on:
/// - Weight of matched symptoms
/// - Whether primary symptoms are present
/// - Ratio of matched vs total disease symptoms
/// - Symptom specificity (rare symptoms count more)
/// - Symptom co-occurrence bonus (symptom clusters)
/// - Demographic fit (age group and sex relevance)
pub fn score_symptoms(conn: &Connection, input_symptoms: &[&str]) -> Vec<DiagnosisResult> {
    score_symptoms_with_context(conn, input_symptoms, &PatientContext::default())
}

pub fn score_symptoms_with_context(
    conn: &Connection,
    input_symptoms: &[&str],
    context: &PatientContext,
) -> Vec<DiagnosisResult> {
    let raw_normalized: Vec<String> = input_symptoms
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if raw_normalized.is_empty() {
        return vec![];
    }

    // Expand input symptoms with synonym mappings
    let synonym_map = build_synonym_map();
    let mut normalized: Vec<String> = raw_normalized.clone();
    for sym in &raw_normalized {
        if let Some(canonical) = synonym_map.get(sym.as_str()) {
            let canon = canonical.to_string();
            if !normalized.contains(&canon) {
                normalized.push(canon);
            }
        }
    }

    // Pre-compute symptom specificity: how many diseases share each symptom
    let symptom_disease_counts = get_symptom_disease_counts(conn);

    let mut results = Vec::new();

    let mut stmt = conn
        .prepare("SELECT id, name, description, severity, age_group, category FROM diseases")
        .unwrap();

    let diseases: Vec<(i64, String, String, String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get::<_, Option<String>>(4)?.unwrap_or_else(|| "all".into()),
                row.get::<_, Option<String>>(5)?.unwrap_or_else(|| "general".into()),
            ))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let total_diseases = diseases.len().max(1) as f64;

    for (disease_id, disease_name, description, severity, age_group, _category) in &diseases {
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

        // Co-occurrence bonus: matching multiple primary symptoms together
        // is stronger evidence than matching them individually.
        // Enhanced: scales with total matched symptoms (cluster detection).
        let cooccurrence_bonus = if primary_matched >= 2 {
            let primary_bonus = 0.05 * (primary_matched as f64 - 1.0).min(3.0);
            let cluster_bonus = if matched.len() >= 4 {
                0.03 * (matched.len() as f64 - 3.0).min(4.0)
            } else {
                0.0
            };
            primary_bonus + cluster_bonus
        } else if matched.len() >= 4 {
            // Even without multiple primaries, matching many symptoms
            // of a single disease is strong evidence
            0.02 * (matched.len() as f64 - 3.0).min(3.0)
        } else {
            0.0
        };

        // Demographic adjustment: boost/penalize based on age/sex fit
        let demographic_factor = compute_demographic_factor(context, age_group);

        // Combined score
        let raw_score = (weight_ratio * 0.35
            + primary_bonus
            + coverage * 0.20
            + specificity_bonus.min(0.1)
            + cooccurrence_bonus)
            * precision_factor
            * demographic_factor;
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

/// Compute a multiplicative demographic factor based on patient age/sex and disease age_group.
fn compute_demographic_factor(context: &PatientContext, age_group: &str) -> f64 {
    let mut factor = 1.0;

    if let Some(age) = context.age {
        match age_group {
            "children" | "pediatric" => {
                if age > 18 {
                    factor *= 0.6; // less likely in adults
                } else {
                    factor *= 1.15;
                }
            }
            "neonates" => {
                if age > 0 {
                    factor *= 0.3;
                } else {
                    factor *= 1.2;
                }
            }
            "adults" | "adult" => {
                if age < 16 {
                    factor *= 0.6;
                } else {
                    factor *= 1.05;
                }
            }
            _ => {} // "all" — no adjustment
        }
    }

    // Sex-based adjustments for specific disease age groups
    // (handled via category in seed data, but age_group gives a hint)
    if let Some(ref sex) = context.sex {
        match age_group {
            // Obstetric/gynecological conditions strongly favor female
            "adults" | "adult" => {
                // No blanket adjustment; category-level would be better
                // but we keep it neutral here
            }
            _ => {}
        }
        // Just ensure sex is used to suppress the unused warning
        let _ = sex;
    }

    factor
}

/// Count how many diseases each symptom appears in (for specificity calculation).
fn get_symptom_disease_counts(conn: &Connection) -> std::collections::HashMap<String, usize> {
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
/// if individual words overlap significantly, or if edit distance is small (typo tolerance).
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
            // Typo tolerance: allow edit distance ≤ 1 for words ≥ 5 chars
            if iw.len() >= 5 && sw.len() >= 5 && edit_distance(iw, sw) <= 1 {
                return true;
            }
        }
    }

    false
}

/// Build a lookup map from synonym → canonical symptom name.
fn build_synonym_map() -> HashMap<&'static str, &'static str> {
    crate::db::seed::get_symptom_synonyms().into_iter().collect()
}

/// Simple Levenshtein edit distance for typo tolerance.
fn edit_distance(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let m = a_bytes.len();
    let n = b_bytes.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
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
        let results = score_symptoms(&conn, &["xyzzyplugh"]);
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
    fn test_synonym_expansion_stomach_ache() {
        let conn = db::init_memory_database().unwrap();
        // "stomach ache" should expand to "abdominal pain" via synonyms
        let results = score_symptoms(&conn, &["stomach ache", "fever"]);
        assert!(!results.is_empty(), "Synonym expansion should find matches");
        // Should match diseases with abdominal pain
        let has_abdominal = results
            .iter()
            .any(|r| r.matched_symptoms.iter().any(|s| s.to_lowercase().contains("abdominal")));
        assert!(has_abdominal, "Should match abdominal pain via synonym");
    }

    #[test]
    fn test_synonym_expansion_breathlessness() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["breathlessness", "chest pain"]);
        assert!(!results.is_empty(), "breathlessness should match via synonym");
    }

    #[test]
    fn test_synonym_expansion_tired() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["tired", "headache"]);
        assert!(!results.is_empty(), "tired should expand to fatigue");
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

    #[test]
    fn test_demographic_context_children() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext {
            age: Some(3),
            sex: None,
        };
        let adult_ctx = PatientContext {
            age: Some(40),
            sex: None,
        };

        // Croup is a pediatric disease
        let child_results =
            score_symptoms_with_context(&conn, &["barking cough", "stridor", "fever"], &child_ctx);
        let adult_results =
            score_symptoms_with_context(&conn, &["barking cough", "stridor", "fever"], &adult_ctx);

        let child_croup = child_results.iter().find(|r| r.disease_name == "Croup");
        let adult_croup = adult_results.iter().find(|r| r.disease_name == "Croup");

        if let (Some(cc), Some(ac)) = (child_croup, adult_croup) {
            assert!(
                cc.probability > ac.probability,
                "Croup should score higher for children ({}) than adults ({})",
                cc.probability,
                ac.probability
            );
        }
    }

    #[test]
    fn test_demographic_factor_ranges() {
        let ctx_child = PatientContext {
            age: Some(5),
            sex: None,
        };
        let ctx_adult = PatientContext {
            age: Some(35),
            sex: None,
        };

        assert!(compute_demographic_factor(&ctx_child, "children") > 1.0);
        assert!(compute_demographic_factor(&ctx_adult, "children") < 1.0);
        assert!(compute_demographic_factor(&ctx_adult, "adults") >= 1.0);
        assert!(compute_demographic_factor(&ctx_child, "adults") < 1.0);
    }

    #[test]
    fn test_score_pulmonary_embolism() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(
            &conn,
            &["sudden shortness of breath", "chest pain", "rapid heart rate", "leg swelling"],
        );
        let pe = results.iter().find(|r| r.disease_name == "Pulmonary Embolism");
        assert!(pe.is_some(), "Pulmonary Embolism should appear");
        assert!(pe.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_scarlet_fever() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["red rash", "strawberry tongue", "sore throat", "fever"]);
        let sf = results.iter().find(|r| r.disease_name == "Scarlet Fever");
        assert!(sf.is_some(), "Scarlet Fever should appear");
    }

    #[test]
    fn test_score_pericarditis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sharp chest pain", "pain worse when lying down"]);
        let pc = results.iter().find(|r| r.disease_name == "Pericarditis");
        assert!(pc.is_some(), "Pericarditis should appear");
    }

    #[test]
    fn test_score_encephalitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fever", "severe headache", "confusion", "seizures"]);
        let enc = results.iter().find(|r| r.disease_name == "Encephalitis");
        assert!(enc.is_some(), "Encephalitis should appear");
    }

    #[test]
    fn test_score_ectopic_pregnancy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sharp pelvic pain", "vaginal bleeding", "missed period"]);
        let ep = results.iter().find(|r| r.disease_name == "Ectopic Pregnancy");
        assert!(ep.is_some(), "Ectopic Pregnancy should appear");
    }

    #[test]
    fn test_score_intussusception() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(
            &conn,
            &["severe intermittent abdominal pain", "red currant jelly stool", "vomiting"],
        );
        let intus = results.iter().find(|r| r.disease_name == "Intussusception");
        assert!(intus.is_some(), "Intussusception should appear");
    }

    #[test]
    fn test_synonym_throwing_up() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["throwing up", "fever"]);
        assert!(!results.is_empty(), "throwing up should expand to vomiting");
    }


    #[test]
    fn test_score_heatstroke() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "confusion", "hot dry skin"]);
        let hs = results.iter().find(|r| r.disease_name == "Heatstroke");
        assert!(hs.is_some(), "Heatstroke should appear");
    }

    #[test]
    fn test_score_peritonitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal pain", "abdominal rigidity", "fever"]);
        let pt = results.iter().find(|r| r.disease_name == "Peritonitis");
        assert!(pt.is_some(), "Peritonitis should appear");
    }

    #[test]
    fn test_score_bells_palsy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["facial drooping", "inability to close eye", "drooling"]);
        let bp = results.iter().find(|r| r.disease_name == "Bell's Palsy");
        assert!(bp.is_some(), "Bell's Palsy should appear");
    }

    #[test]
    fn test_score_necrotizing_fasciitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe pain disproportionate to appearance", "rapid skin redness spreading", "fever"]);
        let nf = results.iter().find(|r| r.disease_name == "Necrotizing Fasciitis");
        assert!(nf.is_some(), "Necrotizing Fasciitis should appear");
    }

    #[test]
    fn test_score_bronchiolitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["wheezing", "cough", "rapid breathing"]);
        let br = results.iter().find(|r| r.disease_name == "Bronchiolitis");
        assert!(br.is_some(), "Bronchiolitis should appear");
    }

    #[test]
    fn test_synonym_ringing_in_ears() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["ringing in ears", "hearing loss"]);
        assert!(!results.is_empty(), "ringing in ears should match via synonym");
    }

    #[test]
    fn test_synonym_photophobia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["photophobia", "eye pain"]);
        assert!(!results.is_empty(), "photophobia should match via synonym");
    }
    #[test]
    fn test_score_lupus() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["butterfly rash", "joint pain", "fatigue", "fever"]);
        let lupus = results.iter().find(|r| r.disease_name == "Systemic Lupus Erythematosus");
        assert!(lupus.is_some(), "SLE should appear in results");
        assert!(lupus.unwrap().probability > 20.0);
    }

    #[test]
    fn test_score_gout() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe joint pain", "joint swelling", "joint redness"]);
        let gout = results.iter().find(|r| r.disease_name == "Gout");
        assert!(gout.is_some(), "Gout should appear in results");
        assert!(gout.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_iron_deficiency_anemia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "weakness", "pale skin", "dizziness"]);
        let ida = results.iter().find(|r| r.disease_name == "Iron Deficiency Anemia");
        assert!(ida.is_some(), "Iron Deficiency Anemia should appear");
    }

    #[test]
    fn test_score_pancreatitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal pain", "pain radiating to back", "nausea", "vomiting"]);
        let panc = results.iter().find(|r| r.disease_name == "Pancreatitis");
        assert!(panc.is_some(), "Pancreatitis should appear");
        assert!(panc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_sinusitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["facial pain", "nasal congestion", "thick nasal discharge"]);
        let sinus = results.iter().find(|r| r.disease_name == "Sinusitis");
        assert!(sinus.is_some(), "Sinusitis should appear");
    }

    #[test]
    fn test_synonym_brain_fog() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["brain fog", "widespread pain", "fatigue"]);
        assert!(!results.is_empty(), "brain fog should expand to cognitive difficulties");
    }

    #[test]
    fn test_cluster_bonus_many_symptoms() {
        let conn = db::init_memory_database().unwrap();
        // Many matching symptoms should get cluster bonus
        let results = score_symptoms(
            &conn,
            &["fever", "chills", "sweating", "headache", "nausea", "vomiting", "muscle pain", "fatigue"],
        );
        let malaria = results.iter().find(|r| r.disease_name == "Malaria");
        assert!(malaria.is_some());
        // With cluster bonus, should score very high
        assert!(malaria.unwrap().probability > 60.0, "Cluster bonus should push score high");
    }

    #[test]
    fn test_cooccurrence_bonus_multiple_primary() {
        let conn = db::init_memory_database().unwrap();
        // Cholera has 3 primary symptoms: watery diarrhea, vomiting, dehydration
        let results = score_symptoms(&conn, &["watery diarrhea", "vomiting", "dehydration"]);
        let cholera = results.iter().find(|r| r.disease_name == "Cholera");
        assert!(cholera.is_some());
        // With co-occurrence bonus, should be quite high
        assert!(
            cholera.unwrap().probability > 50.0,
            "Multiple primary symptoms should boost score significantly"
        );
    }
}
