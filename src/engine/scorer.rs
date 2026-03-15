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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_note: Option<String>,
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

        // Demographic adjustment: boost/penalize based on age/sex fit and category
        let demographic_factor = compute_demographic_factor(context, age_group)
            * compute_sex_factor(context, _category);

        // Negative evidence: check if patient has symptoms that argue against this disease
        let negative_map = get_negative_evidence();
        let neg_penalty = if let Some(neg_symptoms) = negative_map.get(disease_name.as_str()) {
            let neg_count = neg_symptoms.iter()
                .filter(|ns| normalized.iter().any(|input| fuzzy_match(input, &ns.to_lowercase())))
                .count();
            // Each contradicting symptom reduces score by 15%
            (0.85_f64).powi(neg_count as i32)
        } else {
            1.0
        };

        // Confidence note: indicate when missing key symptoms or negative evidence affects score
        let confidence_note = if !missing_primary.is_empty() && neg_penalty < 1.0 {
            Some("Missing key symptoms and contradicting evidence present".to_string())
        } else if neg_penalty < 1.0 {
            Some("Some symptoms argue against this diagnosis".to_string())
        } else if missing_primary.len() >= 2 {
            Some("Multiple key symptoms missing — lower confidence".to_string())
        } else {
            None
        };

        // Combined score
        let raw_score = (weight_ratio * 0.35
            + primary_bonus
            + coverage * 0.20
            + specificity_bonus.min(0.1)
            + cooccurrence_bonus)
            * precision_factor
            * demographic_factor
            * neg_penalty;
        let probability = (raw_score * 100.0).clamp(1.0, 95.0);

        results.push(DiagnosisResult {
            disease_name: disease_name.clone(),
            probability,
            matched_symptoms: matched,
            missing_key_symptoms: missing_primary,
            severity: severity.clone(),
            description: description.clone(),
            confidence_note,
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

    factor
}

/// Sex-based demographic adjustment for category-specific diseases.
fn compute_sex_factor(context: &PatientContext, category: &str) -> f64 {
    if let Some(ref sex) = context.sex {
        let sex_lower = sex.to_lowercase();
        match category {
            "gynecological" | "obstetric" => {
                if sex_lower == "male" {
                    0.05 // extremely unlikely in males
                } else {
                    1.1
                }
            }
            "urological" => {
                // Some urological conditions like testicular torsion are male-only
                if sex_lower == "female" {
                    0.7
                } else {
                    1.05
                }
            }
            _ => 1.0,
        }
    } else {
        1.0
    }
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
            if iw == sw {
                return true;
            }
            // Require minimum 4 chars for substring matching to avoid
            // false positives like "testicular" matching "tic"
            if (iw.len() >= 4 && sw.contains(iw)) || (sw.len() >= 4 && iw.contains(sw)) {
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

/// Negative evidence: symptoms that argue AGAINST a diagnosis.
/// If a patient has these symptoms, the disease is less likely.
fn get_negative_evidence() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    // Heart attack: typically NO fever, no rash
    map.insert("Heart Attack", vec!["rash", "high fever", "diarrhea"]);
    // Stroke: typically no fever early, no cough
    map.insert("Stroke", vec!["cough", "diarrhea", "rash"]);
    // Appendicitis: pain typically RIGHT side, no cough
    map.insert("Appendicitis", vec!["cough", "rash", "sore throat"]);
    // Common Cold: no high fever, no rash, no severe headache
    map.insert("Common Cold", vec!["high fever", "rash", "severe headache", "confusion"]);
    // Malaria: no cough, no sore throat typically
    map.insert("Malaria", vec!["cough", "sore throat", "rash"]);
    // Cholera: no fever typically, no rash
    map.insert("Cholera", vec!["high fever", "rash", "cough"]);
    // Migraine: no fever, no rash
    map.insert("Migraine", vec!["fever", "rash", "diarrhea"]);
    // Asthma: no fever (unless infection), no rash
    map.insert("Asthma", vec!["fever", "rash", "diarrhea"]);
    // Lactose Intolerance: no fever, no rash
    map.insert("Lactose Intolerance", vec!["fever", "rash", "fatigue", "weight loss"]);
    // Tension Headache: no fever, no vision changes, no rash
    map.insert("Tension Headache", vec!["fever", "rash", "vomiting", "vision changes"]);
    // Cluster Headache: no fever, no neck stiffness
    map.insert("Cluster Headache", vec!["fever", "neck stiffness", "rash"]);
    // Pneumonia: no rash typically
    map.insert("Pneumonia", vec!["rash", "joint pain", "diarrhea"]);
    // Anaphylaxis: typically rapid onset, no gradual fever
    map.insert("Anaphylaxis", vec!["gradual onset", "fever"]);
    // Peptic Ulcer: no fever unless perforated
    map.insert("Peptic Ulcer Disease", vec!["fever", "rash", "cough"]);
    // Gout: no bilateral, no fever usually (unless tophaceous)
    map.insert("Gout", vec!["cough", "rash", "diarrhea"]);
    // GERD: no fever, no weight loss
    map.insert("Irritable Bowel Syndrome", vec!["fever", "bloody stool", "weight loss"]);
    // Conversion Disorder: no organic signs
    map.insert("Conversion Disorder", vec!["fever", "rash", "weight loss"]);
    // Hyperkalemia: no rash, no cough
    map.insert("Hyperkalemia", vec!["rash", "cough", "fever"]);
    // DVT: typically unilateral, no rash
    map.insert("Deep Vein Thrombosis", vec!["rash", "cough", "fever"]);
    // v16 negative evidence
    map.insert("Graves' Disease", vec!["weight gain", "cold intolerance", "constipation"]);
    map.insert("Hashimoto's Thyroiditis", vec!["weight loss", "heat intolerance", "diarrhea"]);
    map.insert("Cataracts", vec!["eye pain", "eye redness", "headache"]);
    // v0.19.0 negative evidence
    map.insert("Panic Disorder", vec!["fever", "rash", "weight loss", "cough"]);
    map.insert("Sinusitis", vec!["chest pain", "rash", "diarrhea"]);
    map.insert("Plantar Fasciitis", vec!["fever", "rash", "swelling", "numbness"]);
    map.insert("Vertigo (BPPV)", vec!["fever", "hearing loss", "ear discharge"]);
    map.insert("Rosacea", vec!["fever", "joint pain", "fatigue", "weight loss"]);
    map.insert("Nephrolithiasis", vec!["rash", "cough", "sore throat"]);
    map.insert("Scurvy", vec!["fever", "cough", "diarrhea"]);
    map.insert("Frostbite", vec!["fever", "sweating", "rash"]);
    map.insert("Rabies", vec!["rash", "cough", "diarrhea"]);
    map.insert("Bipolar Disorder", vec!["fever", "rash", "cough", "weight loss"]);
    map.insert("Hypothermia", vec!["fever", "sweating", "rash"]);
    map.insert("Spontaneous Pneumothorax", vec!["fever", "productive cough", "rash"]);
    map.insert("Parkinson's Disease", vec!["fever", "rash", "diarrhea"]);
    map.insert("Alzheimer's Disease", vec!["fever", "acute pain", "rash"]);
    map.insert("Fibromyalgia", vec!["fever", "joint swelling", "rash"]);
    // v17 negative evidence
    map.insert("Carpal Tunnel Syndrome", vec!["fever", "rash", "cough"]);
    map.insert("Endometriosis", vec!["fever", "rash", "cough"]);
    map.insert("Celiac Disease", vec!["fever", "rash", "cough"]);
    map.insert("Bipolar Disorder", vec!["fever", "rash", "cough", "joint pain"]);
    map.insert("Plantar Fasciitis", vec!["fever", "rash", "cough"]);
    map.insert("Sciatica", vec!["fever", "rash", "cough"]);
    map.insert("Postpartum Depression", vec!["fever", "rash", "cough"]);
    // v18 negative evidence
    map.insert("Addison's Disease", vec!["weight gain", "moon face", "high blood pressure"]);
    map.insert("Cushing's Syndrome", vec!["weight loss", "hyperpigmentation", "low blood pressure"]);
    map.insert("Aortic Dissection", vec!["rash", "fever", "gradual onset"]);
    map.insert("Myocarditis", vec!["rash", "joint pain", "diarrhea"]);
    map.insert("Multiple Sclerosis", vec!["fever", "rash", "diarrhea"]);
    map.insert("Trigeminal Neuralgia", vec!["fever", "rash", "bilateral pain"]);
    map.insert("Toxic Shock Syndrome", vec!["gradual onset", "joint stiffness"]);
    map.insert("Sarcoidosis", vec!["high fever", "diarrhea", "vomiting"]);
    // v21 negative evidence
    map.insert("Achalasia", vec!["diarrhea", "fever", "rash"]);
    map.insert("Pheochromocytoma", vec!["rash", "diarrhea", "cough"]);
    map.insert("Polymyalgia Rheumatica", vec!["rash", "swollen joints", "muscle weakness"]);
    map.insert("Restless Legs Syndrome", vec!["fever", "rash", "joint swelling"]);
    map.insert("Normal Pressure Hydrocephalus", vec!["fever", "rash", "acute onset"]);
    map.insert("Interstitial Cystitis", vec!["fever", "blood in urine", "rash"]);
    map.insert("Peripheral Artery Disease", vec!["rash", "fever", "bilateral arm pain"]);
    map.insert("Thoracic Outlet Syndrome", vec!["fever", "rash", "bilateral symptoms"]);
    map.insert("Vocal Cord Dysfunction", vec!["fever", "rash", "wheezing on exhale"]);
    map.insert("Erythema Nodosum", vec!["blisters", "itching", "scaling"]);
    // v22 negative evidence
    map.insert("Chronic Obstructive Pulmonary Disease", vec!["rash", "joint swelling", "diarrhea"]);
    map.insert("Pulmonary Fibrosis", vec!["rash", "diarrhea", "joint swelling"]);
    map.insert("Schizophrenia", vec!["fever", "rash", "joint pain", "cough"]);
    map.insert("Obsessive-Compulsive Disorder", vec!["fever", "rash", "cough", "weight loss"]);
    map.insert("Post-Traumatic Stress Disorder", vec!["fever", "rash", "cough", "joint pain"]);
    map.insert("Otosclerosis", vec!["fever", "ear discharge", "ear pain"]);
    map.insert("Meniere's Disease", vec!["fever", "rash", "cough"]);
    map.insert("Rheumatoid Arthritis", vec!["rash", "fever", "diarrhea"]);
    map.insert("Ankylosing Spondylitis", vec!["rash", "diarrhea", "cough"]);
    map.insert("Epiglottitis", vec!["rash", "diarrhea", "gradual onset"]);
    map.insert("Pyelonephritis", vec!["rash", "cough", "joint pain"]);
    map.insert("Primary Biliary Cholangitis", vec!["fever", "diarrhea", "joint swelling"]);
    map.insert("Actinic Keratosis", vec!["fever", "joint pain", "cough"]);
    // v0.23.0 negative evidence
    map.insert("Atrial Fibrillation", vec!["rash", "fever", "cough", "diarrhea"]);
    map.insert("Allergic Rhinitis", vec!["fever", "chest pain", "weight loss"]);
    map.insert("Obstructive Sleep Apnea", vec!["fever", "rash", "weight loss", "diarrhea"]);
    map.insert("Osteoarthritis", vec!["fever", "rash", "weight loss"]);
    map.insert("Irritable Bowel Syndrome", vec!["fever", "bloody stool", "weight loss"]);
    map.insert("Peripheral Neuropathy", vec!["fever", "rash", "cough"]);
    map.insert("Polycystic Ovary Syndrome (PCOS)", vec!["fever", "rash", "cough", "diarrhea"]);
    map.insert("Psoriatic Arthritis", vec!["fever", "cough", "diarrhea"]);
    map.insert("Chronic Kidney Disease", vec!["rash", "cough", "fever"]);
    map.insert("Sepsis", vec!["rash", "chronic onset", "well-appearing"]);
    map.insert("Thyroid Storm", vec!["rash", "chronic onset", "weight gain"]);
    map.insert("Anaphylactic Shock", vec!["gradual onset", "fever", "chronic"]);
    map.insert("Preeclampsia", vec!["rash", "diarrhea", "cough"]);
    // v0.24.0 negative evidence
    map.insert("Heat Exhaustion", vec!["rash", "cough", "diarrhea"]);
    map.insert("Retinal Detachment", vec!["fever", "rash", "pain"]);
    map.insert("Thyroid Nodule", vec!["fever", "rash", "cough", "diarrhea"]);
    map.insert("Subdural Hematoma", vec!["rash", "cough", "diarrhea", "fever"]);
    map.insert("Cholesteatoma", vec!["rash", "cough", "joint pain"]);
    map.insert("Toxic Epidermal Necrolysis", vec!["cough", "diarrhea", "joint pain"]);
    map.insert("Organophosphate Poisoning", vec!["rash", "joint pain"]);
    // v0.25.0 negative evidence
    map.insert("Bruxism", vec!["fever", "rash", "weight loss"]);
    map.insert("Temporomandibular Joint Disorder", vec!["fever", "rash", "weight loss"]);
    map.insert("Generalized Anxiety Disorder", vec!["fever", "rash", "cough", "weight loss"]);
    map.insert("Eating Disorder (Anorexia Nervosa)", vec!["weight gain", "rash", "cough"]);
    map.insert("Rickets", vec!["rash", "cough", "fever"]);
    map.insert("Febrile Seizure", vec!["rash", "cough", "diarrhea"]);
    map.insert("Phenylketonuria (PKU)", vec!["fever", "rash", "acute onset"]);
    map.insert("Bulimia Nervosa", vec!["fever", "rash", "cough"]);
    map.insert("Impetigo", vec!["fever", "joint pain", "weight loss"]);
    map.insert("Tinea Corporis (Ringworm)", vec!["fever", "joint pain", "weight loss"]);
    map.insert("Metabolic Syndrome", vec!["rash", "cough", "fever"]);
    map.insert("Gallstones (Cholelithiasis)", vec!["rash", "cough", "fever"]);
    map.insert("Chronic Fatigue Syndrome", vec!["fever", "rash", "weight gain"]);
    map.insert("Peritonsillar Abscess", vec!["rash", "diarrhea", "joint pain"]);
    // v0.26.0 negative evidence
    map.insert("Acoustic Neuroma (Vestibular Schwannoma)", vec!["fever", "rash", "bilateral hearing loss"]);
    map.insert("Hemochromatosis", vec!["rash", "cough", "acute onset"]);
    map.insert("Pericarditis", vec!["rash", "diarrhea", "unilateral symptoms"]);
    map.insert("Polymyositis", vec!["rash", "cough", "sensory loss"]);
    map.insert("Pyloric Stenosis", vec!["diarrhea", "rash", "bloody stool"]);
    map.insert("Osteomyelitis", vec!["rash", "diarrhea", "headache"]);
    map.insert("Placenta Previa", vec!["painful bleeding", "rash", "fever"]);
    map.insert("Vocal Cord Polyps", vec!["fever", "rash", "difficulty breathing"]);
    map.insert("Testicular Torsion", vec!["rash", "cough", "gradual onset"]);
    map.insert("Henoch-Schönlein Purpura (IgA Vasculitis)", vec!["cough", "chest pain", "weight loss"]);
    map.insert("Aortic Stenosis", vec!["rash", "cough", "fever"]);
    map.insert("Necrotizing Fasciitis", vec!["gradual onset", "painless", "joint stiffness"]);
    map.insert("Sjogren's Syndrome", vec!["fever", "diarrhea", "weight gain"]);
    map.insert("Carbon Monoxide Poisoning", vec!["rash", "cough", "gradual onset over weeks"]);
    map.insert("Pilonidal Cyst", vec!["cough", "nausea", "headache"]);
    // v0.27.0 negative evidence
    map.insert("Peripartum Cardiomyopathy", vec!["rash", "joint pain", "cough"]);
    map.insert("Wernicke Encephalopathy", vec!["rash", "diarrhea", "joint pain"]);
    map.insert("Acute Compartment Syndrome", vec!["fever", "rash", "cough"]);
    map.insert("Lichen Planus", vec!["fever", "cough", "weight loss"]);
    map.insert("Adhesive Capsulitis (Frozen Shoulder)", vec!["fever", "rash", "cough"]);
    map.insert("Acromegaly", vec!["rash", "diarrhea", "weight loss"]);
    map.insert("Pellagra", vec!["joint swelling", "cough", "chest pain"]);
    map.insert("Toxic Megacolon", vec!["rash", "cough", "joint pain"]);
    map.insert("Mastitis", vec!["cough", "diarrhea", "joint pain"]);
    map.insert("Pelvic Inflammatory Disease", vec!["rash", "cough", "headache"]);
    map.insert("Placental Abruption", vec!["rash", "cough", "diarrhea"]);
    map.insert("Dengue Shock Syndrome", vec!["cough", "sore throat", "rash"]);
    // v0.28.0 negative evidence
    map.insert("Chikungunya", vec!["cough", "sore throat", "diarrhea"]);
    map.insert("Leishmaniasis (Visceral)", vec!["rash", "cough", "joint pain"]);
    map.insert("Schistosomiasis", vec!["cough", "chest pain", "joint pain"]);
    map.insert("Leptospirosis", vec!["chronic onset", "sore throat"]);
    map.insert("Anaphylaxis (Food Allergy)", vec!["gradual onset", "fever", "chronic"]);
    map.insert("Contact Dermatitis", vec!["fever", "joint pain", "weight loss"]);
    map.insert("Benign Paroxysmal Positional Vertigo", vec!["hearing loss", "ear discharge", "fever"]);
    map.insert("Chronic Hepatitis B", vec!["rash", "cough", "acute severe pain"]);
    map.insert("Irritable Bowel Syndrome", vec!["fever", "bloody stool", "weight loss"]);
    map.insert("Urinary Tract Infection", vec!["rash", "cough", "joint pain"]);
    map.insert("Herpes Zoster (Shingles)", vec!["bilateral rash", "cough", "diarrhea"]);
    map.insert("Optic Neuritis", vec!["bilateral vision loss", "rash", "joint swelling"]);
    map.insert("Hyperaldosteronism (Conn's Syndrome)", vec!["rash", "fever", "cough"]);
    map.insert("Mesenteric Ischemia (Acute)", vec!["rash", "cough", "chronic gradual onset"]);
    map.insert("Thoracic Aortic Aneurysm", vec!["rash", "fever", "diarrhea"]);
    // v0.29.0 negative evidence
    map.insert("Inguinal Hernia", vec!["fever", "rash", "diarrhea"]);
    map.insert("Hemorrhoids", vec!["fever", "weight loss", "abdominal mass"]);
    map.insert("Viral Conjunctivitis", vec!["vision loss", "severe eye pain", "fever"]);
    map.insert("Acute Bronchitis", vec!["high fever", "rash", "chest pain"]);
    map.insert("Viral Gastroenteritis", vec!["rash", "chest pain", "joint pain"]);
    map.insert("Tonsillitis", vec!["rash", "cough", "diarrhea"]);
    map.insert("Measles", vec!["diarrhea", "joint pain", "chest pain"]);
    map.insert("Chickenpox (Varicella)", vec!["joint pain", "cough", "diarrhea"]);
    map.insert("Mumps", vec!["rash", "cough", "diarrhea"]);
    map.insert("Chronic Hepatitis C", vec!["rash", "cough", "diarrhea"]);
    map.insert("Tetanus", vec!["rash", "diarrhea", "cough"]);
    map.insert("Yellow Fever", vec!["rash", "cough", "joint pain"]);
    map.insert("Chronic Urticaria", vec!["fever", "joint pain", "weight loss"]);
    map.insert("Varicose Veins", vec!["fever", "rash", "cough"]);
    map.insert("Benign Prostatic Hyperplasia", vec!["fever", "rash", "weight loss"]);
    // v30 negative evidence
    map.insert("Periapical Abscess", vec!["rash", "diarrhea", "cough"]);
    map.insert("Bruxism", vec!["fever", "rash", "diarrhea", "weight loss"]);
    map.insert("Peripheral Artery Disease", vec!["fever", "rash", "cough"]);
    map.insert("Neonatal Jaundice", vec!["rash", "cough", "diarrhea"]);
    map.insert("Neonatal Sepsis", vec!["rash", "chronic pain"]);
    map.insert("Turner Syndrome", vec!["fever", "cough", "diarrhea"]);
    map.insert("Marfan Syndrome", vec!["fever", "rash", "weight gain"]);
    map.insert("Crush Syndrome", vec!["rash", "cough", "sore throat"]);
    map.insert("Fat Embolism Syndrome", vec!["diarrhea", "sore throat", "joint pain"]);
    map.insert("Organophosphate Poisoning", vec!["mydriasis", "dry mouth", "constipation"]);
    map.insert("Carbon Monoxide Poisoning", vec!["rash", "diarrhea", "cough"]);
    map.insert("Ankylosing Spondylitis", vec!["rash", "diarrhea", "cough"]);
    map.insert("Heat Stroke", vec!["hypothermia", "shivering", "rash"]);
    // v0.31.0 negative evidence
    map.insert("Buruli Ulcer", vec!["fever", "cough", "diarrhea"]);
    map.insert("Dracunculiasis (Guinea Worm Disease)", vec!["cough", "rash", "headache"]);
    map.insert("Noma (Cancrum Oris)", vec!["cough", "diarrhea", "rash"]);
    map.insert("Ciguatera Fish Poisoning", vec!["fever", "rash", "cough"]);
    map.insert("Ascariasis", vec!["fever", "rash", "chest pain"]);
    map.insert("Kwashiorkor", vec!["cough", "rash", "chest pain"]);
    // v0.32.0 negative evidence
    map.insert("Hookworm Infection", vec!["cough", "rash", "chest pain"]);
    map.insert("Trachoma", vec!["cough", "diarrhea", "joint pain"]);
    map.insert("Onchocerciasis (River Blindness)", vec!["fever", "diarrhea", "cough"]);
    map.insert("Myocardial Bridge", vec!["fever", "rash", "cough"]);
    map.insert("Lymphatic Filariasis (Elephantiasis)", vec!["cough", "diarrhea", "rash"]);
    map.insert("Acute Rheumatic Fever", vec!["diarrhea", "rash", "cough"]);
    map.insert("African Trypanosomiasis (Sleeping Sickness)", vec!["rash", "cough", "diarrhea"]);
    map.insert("Interstitial Lung Disease", vec!["rash", "diarrhea", "joint swelling"]);
    map
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
    fn test_score_anemia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "weakness", "pale skin", "dizziness"]);
        let anemia = results.iter().find(|r| r.disease_name == "Anemia");
        assert!(anemia.is_some(), "Anemia should appear");
    }

    #[test]
    fn test_score_acute_pancreatitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal pain", "pain radiating to back", "nausea", "vomiting"]);
        let panc = results.iter().find(|r| r.disease_name == "Acute Pancreatitis");
        assert!(panc.is_some(), "Acute Pancreatitis should appear");
        assert!(panc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_common_cold() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["nasal congestion", "sore throat", "cough", "sneezing"]);
        let cold = results.iter().find(|r| r.disease_name == "Common Cold");
        assert!(cold.is_some(), "Common Cold should appear");
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

    // v11.0 tests for new diseases
    #[test]
    fn test_score_myasthenia_gravis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["muscle weakness", "drooping eyelids", "double vision"]);
        let mg = results.iter().find(|r| r.disease_name == "Myasthenia Gravis");
        assert!(mg.is_some(), "Myasthenia Gravis should appear");
        assert!(mg.unwrap().probability > 20.0);
    }

    #[test]
    fn test_score_guillain_barre() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["ascending muscle weakness", "tingling", "difficulty walking"]);
        let gbs = results.iter().find(|r| r.disease_name == "Guillain-Barré Syndrome");
        assert!(gbs.is_some(), "Guillain-Barré Syndrome should appear");
    }

    #[test]
    fn test_score_rhabdomyolysis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe muscle pain", "dark brown urine", "muscle weakness"]);
        let rhabdo = results.iter().find(|r| r.disease_name == "Rhabdomyolysis");
        assert!(rhabdo.is_some(), "Rhabdomyolysis should appear");
        assert!(rhabdo.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_tension_headache() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["bilateral headache", "pressing pain", "neck stiffness"]);
        let th = results.iter().find(|r| r.disease_name == "Tension Headache");
        assert!(th.is_some(), "Tension Headache should appear");
    }

    #[test]
    fn test_score_acute_glaucoma() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe eye pain", "blurred vision", "halos around lights", "nausea"]);
        let gl = results.iter().find(|r| r.disease_name == "Acute Angle-Closure Glaucoma");
        assert!(gl.is_some(), "Acute Angle-Closure Glaucoma should appear");
        assert!(gl.unwrap().probability > 30.0);
    }

    #[test]
    fn test_synonym_droopy_eyelid() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["droopy eyelid", "double vision"]);
        assert!(!results.is_empty(), "droopy eyelid should expand via synonym");
    }

    #[test]
    fn test_synonym_dark_urine() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["dark urine", "muscle pain"]);
        assert!(!results.is_empty(), "dark urine should expand via synonym");
    }

    // v12 tests for new diseases
    #[test]
    fn test_score_narcolepsy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["excessive daytime sleepiness", "cataplexy", "sleep paralysis"]);
        let narc = results.iter().find(|r| r.disease_name == "Narcolepsy");
        assert!(narc.is_some(), "Narcolepsy should appear");
        assert!(narc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_diverticulitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["left lower abdominal pain", "fever", "abdominal tenderness"]);
        let div = results.iter().find(|r| r.disease_name == "Diverticulitis");
        assert!(div.is_some(), "Diverticulitis should appear");
    }

    #[test]
    fn test_score_macular_degeneration() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["blurred central vision", "distorted vision", "dark spots in central vision"]);
        let md = results.iter().find(|r| r.disease_name == "Macular Degeneration");
        assert!(md.is_some(), "Macular Degeneration should appear");
        assert!(md.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_pemphigus() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["painful oral blisters", "skin blisters that rupture easily", "painful erosions"]);
        let pv = results.iter().find(|r| r.disease_name == "Pemphigus Vulgaris");
        assert!(pv.is_some(), "Pemphigus Vulgaris should appear");
    }

    #[test]
    fn test_score_takotsubo() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden chest pain", "shortness of breath", "palpitations"]);
        let tk = results.iter().find(|r| r.disease_name == "Takotsubo Cardiomyopathy");
        assert!(tk.is_some(), "Takotsubo Cardiomyopathy should appear");
    }

    #[test]
    fn test_synonym_sleepy_all_the_time() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sleepy all the time", "cataplexy"]);
        assert!(!results.is_empty(), "sleepy all the time should expand");
    }

    #[test]
    fn test_synonym_stuffy_nose() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["stuffy nose", "face pressure"]);
        assert!(!results.is_empty(), "stuffy nose + face pressure should match sinusitis");
    }

    #[test]
    fn test_score_marfan() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["tall stature", "long limbs and fingers", "lens dislocation"]);
        let mf = results.iter().find(|r| r.disease_name == "Marfan Syndrome");
        assert!(mf.is_some(), "Marfan Syndrome should appear");
    }

    #[test]
    fn test_score_hemophilia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["prolonged bleeding", "easy bruising", "joint bleeding"]);
        let hm = results.iter().find(|r| r.disease_name == "Hemophilia");
        assert!(hm.is_some(), "Hemophilia should appear");
    }

    // v14 tests
    #[test]
    fn test_negative_evidence_reduces_score() {
        let conn = db::init_memory_database().unwrap();
        // Malaria has negative evidence for "cough" — adding cough should reduce malaria score
        let without_cough = score_symptoms(&conn, &["fever", "chills", "headache"]);
        let with_cough = score_symptoms(&conn, &["fever", "chills", "headache", "cough"]);
        let mal_without = without_cough.iter().find(|r| r.disease_name == "Malaria");
        let mal_with = with_cough.iter().find(|r| r.disease_name == "Malaria");
        if let (Some(mw), Some(mc)) = (mal_without, mal_with) {
            assert!(
                mw.probability >= mc.probability,
                "Malaria should score same or lower with contradicting symptom 'cough'"
            );
        }
    }

    #[test]
    fn test_confidence_note_present() {
        let conn = db::init_memory_database().unwrap();
        // Malaria with cough (negative evidence) should get a confidence note
        let results = score_symptoms(&conn, &["fever", "chills", "cough"]);
        let malaria = results.iter().find(|r| r.disease_name == "Malaria");
        if let Some(m) = malaria {
            assert!(
                m.confidence_note.is_some(),
                "Should have confidence note when negative evidence present"
            );
        }
    }

    #[test]
    fn test_score_cdiff() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["watery diarrhea", "abdominal pain", "fever"]);
        let cdiff = results.iter().find(|r| r.disease_name == "Clostridioides difficile Infection");
        assert!(cdiff.is_some(), "C. diff should appear");
    }

    #[test]
    fn test_score_carbon_monoxide() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["headache", "dizziness", "confusion", "nausea"]);
        let co = results.iter().find(|r| r.disease_name == "Carbon Monoxide Poisoning");
        assert!(co.is_some(), "Carbon Monoxide Poisoning should appear");
    }

    #[test]
    fn test_score_dka() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["excessive thirst", "frequent urination", "fruity breath odor", "nausea"]);
        let dka = results.iter().find(|r| r.disease_name == "Diabetic Ketoacidosis");
        assert!(dka.is_some(), "DKA should appear");
        assert!(dka.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_botulism() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["descending paralysis", "double vision", "difficulty swallowing"]);
        let bot = results.iter().find(|r| r.disease_name == "Botulism");
        assert!(bot.is_some(), "Botulism should appear");
    }

    #[test]
    fn test_score_cauda_equina() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["saddle anesthesia", "urinary retention", "back pain"]);
        let ce = results.iter().find(|r| r.disease_name == "Cauda Equina Syndrome");
        assert!(ce.is_some(), "Cauda Equina Syndrome should appear");
    }

    #[test]
    fn test_score_ludwig_angina() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["floor of mouth swelling", "difficulty swallowing", "drooling", "fever"]);
        let la = results.iter().find(|r| r.disease_name == "Ludwig Angina");
        assert!(la.is_some(), "Ludwig Angina should appear");
    }

    // v16 disease tests
    #[test]
    fn test_score_graves_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["weight loss", "rapid heartbeat", "bulging eyes", "heat intolerance"]);
        let gd = results.iter().find(|r| r.disease_name == "Graves' Disease");
        assert!(gd.is_some(), "Graves' Disease should appear");
        assert!(gd.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_hashimotos() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "weight gain", "cold intolerance", "dry skin"]);
        let ht = results.iter().find(|r| r.disease_name == "Hashimoto's Thyroiditis");
        assert!(ht.is_some(), "Hashimoto's Thyroiditis should appear");
    }

    #[test]
    fn test_score_septic_arthritis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["acute joint pain", "joint swelling", "fever", "inability to move joint"]);
        let sa = results.iter().find(|r| r.disease_name == "Septic Arthritis");
        assert!(sa.is_some(), "Septic Arthritis should appear");
        assert!(sa.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_hypothermia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["shivering", "confusion", "loss of coordination", "cold pale skin"]);
        let hypo = results.iter().find(|r| r.disease_name == "Hypothermia");
        assert!(hypo.is_some(), "Hypothermia should appear");
    }

    #[test]
    fn test_score_spontaneous_pneumothorax() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden chest pain", "shortness of breath", "decreased breath sounds"]);
        let sp = results.iter().find(|r| r.disease_name == "Spontaneous Pneumothorax");
        assert!(sp.is_some(), "Spontaneous Pneumothorax should appear");
    }

    #[test]
    fn test_score_chronic_migraine() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["frequent headaches", "throbbing head pain", "light sensitivity", "nausea"]);
        let cm = results.iter().find(|r| r.disease_name == "Chronic Migraine");
        assert!(cm.is_some(), "Chronic Migraine should appear");
        assert!(cm.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_acute_porphyria() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal pain", "dark red urine", "confusion", "rapid heart rate"]);
        let aip = results.iter().find(|r| r.disease_name == "Acute Intermittent Porphyria");
        assert!(aip.is_some(), "Acute Intermittent Porphyria should appear");
    }

    #[test]
    fn test_score_itp() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["easy bruising", "petechiae", "purpura", "nosebleeds"]);
        let itp = results.iter().find(|r| r.disease_name == "Idiopathic Thrombocytopenic Purpura");
        assert!(itp.is_some(), "ITP should appear");
    }

    // v17 disease tests
    #[test]
    fn test_score_wilsons_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["jaundice", "tremor", "Kayser-Fleischer rings"]);
        let wd = results.iter().find(|r| r.disease_name == "Wilson's Disease");
        assert!(wd.is_some(), "Wilson's Disease should appear");
        assert!(wd.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_cystic_fibrosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["persistent cough with thick mucus", "recurrent lung infections", "poor weight gain"]);
        let cf = results.iter().find(|r| r.disease_name == "Cystic Fibrosis");
        assert!(cf.is_some(), "Cystic Fibrosis should appear");
    }

    #[test]
    fn test_score_carpal_tunnel() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["hand numbness", "hand tingling", "wrist pain"]);
        let ct = results.iter().find(|r| r.disease_name == "Carpal Tunnel Syndrome");
        assert!(ct.is_some(), "Carpal Tunnel Syndrome should appear");
        assert!(ct.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_endometriosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe menstrual cramps", "chronic pelvic pain", "pain during intercourse"]);
        let endo = results.iter().find(|r| r.disease_name == "Endometriosis");
        assert!(endo.is_some(), "Endometriosis should appear");
    }

    #[test]
    fn test_score_testicular_torsion() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden severe scrotal pain", "scrotal swelling", "nausea"]);
        let tt = results.iter().find(|r| r.disease_name == "Testicular Torsion");
        assert!(tt.is_some(), "Testicular Torsion should appear");
    }

    #[test]
    fn test_score_sickle_cell_crisis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe bone pain", "chest pain", "fever"]);
        let scc = results.iter().find(|r| r.disease_name == "Sickle Cell Crisis");
        assert!(scc.is_some(), "Sickle Cell Crisis should appear");
    }

    #[test]
    fn test_score_celiac() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chronic diarrhea", "bloating", "weight loss"]);
        let cel = results.iter().find(|r| r.disease_name == "Celiac Disease");
        assert!(cel.is_some(), "Celiac Disease should appear");
    }

    #[test]
    fn test_sex_factor_gynecological() {
        let male_ctx = PatientContext { age: Some(30), sex: Some("male".to_string()) };
        let female_ctx = PatientContext { age: Some(30), sex: Some("female".to_string()) };
        let male_f = compute_sex_factor(&male_ctx, "gynecological");
        let female_f = compute_sex_factor(&female_ctx, "gynecological");
        assert!(male_f < female_f, "Gynecological should penalize males: m={} f={}", male_f, female_f);
    }

    #[test]
    fn test_sex_factor_neutral() {
        let ctx = PatientContext { age: Some(30), sex: Some("male".to_string()) };
        assert_eq!(compute_sex_factor(&ctx, "infectious"), 1.0);
    }

    #[test]
    fn test_synonym_period_pain() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["period pain", "pelvic pain"]);
        assert!(!results.is_empty(), "period pain should expand via synonym");
    }

    #[test]
    fn test_synonym_testicle_pain() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["testicle pain", "nausea"]);
        assert!(!results.is_empty(), "testicle pain should expand via synonym");
    }

    // v18 disease tests
    #[test]
    fn test_score_addisons_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "weight loss", "hyperpigmentation", "low blood pressure"]);
        let ad = results.iter().find(|r| r.disease_name == "Addison's Disease");
        assert!(ad.is_some(), "Addison's Disease should appear");
        assert!(ad.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_cushings_syndrome() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["weight gain", "moon face", "purple stretch marks", "easy bruising"]);
        let cs = results.iter().find(|r| r.disease_name == "Cushing's Syndrome");
        assert!(cs.is_some(), "Cushing's Syndrome should appear");
        assert!(cs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_aortic_dissection() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden tearing chest pain", "pain radiating to back", "weak pulse"]);
        let ad = results.iter().find(|r| r.disease_name == "Aortic Dissection");
        assert!(ad.is_some(), "Aortic Dissection should appear");
    }

    #[test]
    fn test_score_kawasaki() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(3), sex: None };
        let results = score_symptoms_with_context(&conn, &["high fever", "red eyes", "strawberry tongue", "rash"], &child_ctx);
        let kd = results.iter().find(|r| r.disease_name == "Kawasaki Disease");
        assert!(kd.is_some(), "Kawasaki Disease should appear");
    }

    #[test]
    fn test_score_multiple_sclerosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["vision problems", "numbness", "tingling", "fatigue"]);
        let ms = results.iter().find(|r| r.disease_name == "Multiple Sclerosis");
        assert!(ms.is_some(), "Multiple Sclerosis should appear");
    }

    #[test]
    fn test_score_als() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["muscle weakness", "muscle twitching", "difficulty speaking"]);
        let als = results.iter().find(|r| r.disease_name == "Amyotrophic Lateral Sclerosis");
        assert!(als.is_some(), "ALS should appear");
    }

    #[test]
    fn test_score_toxic_shock() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "low blood pressure", "diffuse red rash", "confusion"]);
        let tss = results.iter().find(|r| r.disease_name == "Toxic Shock Syndrome");
        assert!(tss.is_some(), "Toxic Shock Syndrome should appear");
    }

    #[test]
    fn test_score_myocarditis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chest pain", "shortness of breath", "rapid heartbeat", "fever"]);
        let mc = results.iter().find(|r| r.disease_name == "Myocarditis");
        assert!(mc.is_some(), "Myocarditis should appear");
    }

    #[test]
    fn test_score_hepatitis_a() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["jaundice", "dark urine", "nausea", "fatigue"]);
        let ha = results.iter().find(|r| r.disease_name == "Hepatitis A");
        assert!(ha.is_some(), "Hepatitis A should appear");
    }

    #[test]
    fn test_negative_evidence_addisons_vs_cushings() {
        let conn = db::init_memory_database().unwrap();
        // Weight gain is negative for Addison's; should favor Cushing's
        let results = score_symptoms(&conn, &["fatigue", "weight gain", "moon face"]);
        let addison = results.iter().find(|r| r.disease_name == "Addison's Disease");
        let cushing = results.iter().find(|r| r.disease_name == "Cushing's Syndrome");
        if let (Some(a), Some(c)) = (addison, cushing) {
            assert!(
                c.probability >= a.probability,
                "Cushing's should score >= Addison's with weight gain: C={} A={}",
                c.probability, a.probability
            );
        }
    }

    #[test]
    fn test_synonym_trouble_swallowing() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["trouble swallowing", "chest pain"]);
        assert!(!results.is_empty(), "trouble swallowing should expand via synonym");
    }

    #[test]
    fn test_synonym_dark_skin_patches() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["dark skin patches", "fatigue", "weight loss"]);
        assert!(!results.is_empty(), "dark skin patches should match via synonym");
    }

    #[test]
    fn test_negative_evidence_graves_vs_hashimoto() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "goiter", "weight gain"]);
        let graves = results.iter().find(|r| r.disease_name == "Graves' Disease");
        let hashimoto = results.iter().find(|r| r.disease_name == "Hashimoto's Thyroiditis");
        if let (Some(g), Some(h)) = (graves, hashimoto) {
            assert!(
                h.probability >= g.probability,
                "Hashimoto's should score >= Graves' with weight gain (neg evidence): H={} G={}",
                h.probability, g.probability
            );
        }
    }

    // ── v0.19.0 scorer tests ──

    #[test]
    fn test_score_sinusitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["facial pain", "nasal congestion", "headache"]);
        assert!(results.iter().any(|r| r.disease_name == "Sinusitis"), "Sinusitis should appear for facial pain + nasal congestion");
    }

    #[test]
    fn test_score_rabies() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["hydrophobia", "agitation", "fever"]);
        assert!(results.iter().any(|r| r.disease_name == "Rabies"), "Rabies should appear for hydrophobia");
    }

    #[test]
    fn test_score_nephrolithiasis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe flank pain", "blood in urine", "nausea"]);
        assert!(results.iter().any(|r| r.disease_name == "Nephrolithiasis"), "Kidney stones should appear");
    }

    #[test]
    fn test_score_panic_disorder() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["palpitations", "chest tightness", "shortness of breath", "fear of dying"]);
        assert!(results.iter().any(|r| r.disease_name == "Panic Disorder"), "Panic Disorder should appear");
    }

    #[test]
    fn test_score_acute_otitis_media() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["ear pain", "fever", "hearing loss"]);
        assert!(results.iter().any(|r| r.disease_name == "Acute Otitis Media"), "Otitis Media should appear");
    }

    #[test]
    fn test_score_dental_abscess() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe toothache", "facial swelling", "fever"]);
        assert!(results.iter().any(|r| r.disease_name == "Dental Abscess"), "Dental Abscess should appear");
    }

    #[test]
    fn test_score_scurvy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["bleeding gums", "bruising easily", "fatigue"]);
        assert!(results.iter().any(|r| r.disease_name == "Scurvy"), "Scurvy should appear for bleeding gums + bruising");
    }

    #[test]
    fn test_score_bppv() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["vertigo", "dizziness", "nausea"]);
        assert!(results.iter().any(|r| r.disease_name == "Vertigo (BPPV)"), "BPPV should appear for vertigo + dizziness");
    }

    #[test]
    fn test_score_rosacea() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["facial redness", "visible blood vessels", "burning sensation on face"]);
        assert!(results.iter().any(|r| r.disease_name == "Rosacea"), "Rosacea should appear");
    }

    #[test]
    fn test_score_plantar_fasciitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["heel pain", "pain worse in morning"]);
        assert!(results.iter().any(|r| r.disease_name == "Plantar Fasciitis"), "Plantar Fasciitis should appear");
    }

    #[test]
    fn test_synonym_room_spinning() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["room spinning", "nausea"]);
        assert!(results.iter().any(|r| r.matched_symptoms.iter().any(|s| s.to_lowercase().contains("vertigo"))),
            "room spinning should expand to vertigo");
    }

    #[test]
    fn test_synonym_blood_in_pee() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["kidney pain", "blood in pee"]);
        assert!(!results.is_empty(), "kidney pain + blood in pee should match via synonyms");
    }

    #[test]
    fn test_negative_evidence_panic_no_fever() {
        let conn = db::init_memory_database().unwrap();
        let with_fever = score_symptoms(&conn, &["palpitations", "chest tightness", "fever"]);
        let without_fever = score_symptoms(&conn, &["palpitations", "chest tightness"]);
        let panic_with = with_fever.iter().find(|r| r.disease_name == "Panic Disorder");
        let panic_without = without_fever.iter().find(|r| r.disease_name == "Panic Disorder");
        if let (Some(pw), Some(pwo)) = (panic_with, panic_without) {
            assert!(pwo.probability >= pw.probability,
                "Panic Disorder should score lower with fever (negative evidence)");
        }
    }
}

// ── v0.21.0 scorer tests ──

#[cfg(test)]
mod tests_v21 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_achalasia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["difficulty swallowing", "regurgitation", "chest pain"]);
        let ac = results.iter().find(|r| r.disease_name == "Achalasia");
        assert!(ac.is_some(), "Achalasia should appear");
        assert!(ac.unwrap().probability > 20.0);
    }

    #[test]
    fn test_score_pheochromocytoma() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["episodic hypertension", "severe headache", "excessive sweating", "rapid heartbeat"]);
        let pheo = results.iter().find(|r| r.disease_name == "Pheochromocytoma");
        assert!(pheo.is_some(), "Pheochromocytoma should appear");
        assert!(pheo.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_polymyalgia_rheumatica() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["shoulder pain and stiffness", "hip pain and stiffness", "fatigue"]);
        let pmr = results.iter().find(|r| r.disease_name == "Polymyalgia Rheumatica");
        assert!(pmr.is_some(), "Polymyalgia Rheumatica should appear");
    }

    #[test]
    fn test_score_cholangitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fever", "jaundice", "right upper quadrant pain"]);
        let ch = results.iter().find(|r| r.disease_name == "Cholangitis");
        assert!(ch.is_some(), "Cholangitis should appear");
        assert!(ch.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_interstitial_cystitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["bladder pressure", "urinary urgency", "chronic pelvic pain"]);
        let ic = results.iter().find(|r| r.disease_name == "Interstitial Cystitis");
        assert!(ic.is_some(), "Interstitial Cystitis should appear");
    }

    #[test]
    fn test_score_hemolytic_uremic_syndrome() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(3), sex: None };
        let results = score_symptoms_with_context(&conn, &["bloody diarrhea", "decreased urination", "pallor"], &child_ctx);
        let hus = results.iter().find(|r| r.disease_name == "Hemolytic Uremic Syndrome");
        assert!(hus.is_some(), "HUS should appear");
    }

    #[test]
    fn test_score_restless_legs() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["urge to move legs", "uncomfortable leg sensations", "insomnia"]);
        let rls = results.iter().find(|r| r.disease_name == "Restless Legs Syndrome");
        assert!(rls.is_some(), "RLS should appear");
    }

    #[test]
    fn test_score_orbital_cellulitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["eye swelling", "eye pain", "proptosis", "fever"]);
        let oc = results.iter().find(|r| r.disease_name == "Orbital Cellulitis");
        assert!(oc.is_some(), "Orbital Cellulitis should appear");
        assert!(oc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_normal_pressure_hydrocephalus() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["gait disturbance", "urinary incontinence", "cognitive decline"]);
        let nph = results.iter().find(|r| r.disease_name == "Normal Pressure Hydrocephalus");
        assert!(nph.is_some(), "NPH should appear");
        assert!(nph.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_mastoiditis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["swelling behind ear", "ear pain", "fever"]);
        let mast = results.iter().find(|r| r.disease_name == "Mastoiditis");
        assert!(mast.is_some(), "Mastoiditis should appear");
    }

    #[test]
    fn test_score_peripheral_artery_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["leg pain when walking", "leg cramping", "cold feet"]);
        let pad = results.iter().find(|r| r.disease_name == "Peripheral Artery Disease");
        assert!(pad.is_some(), "PAD should appear");
    }

    #[test]
    fn test_score_hyperemesis_gravidarum() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe persistent vomiting", "nausea", "weight loss", "dehydration"]);
        let hg = results.iter().find(|r| r.disease_name == "Hyperemesis Gravidarum");
        assert!(hg.is_some(), "Hyperemesis Gravidarum should appear");
    }

    #[test]
    fn test_synonym_restless_legs() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["restless legs", "insomnia"]);
        assert!(!results.is_empty(), "restless legs should expand via synonym");
    }

    #[test]
    fn test_synonym_claudication() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["claudication", "cold feet"]);
        assert!(!results.is_empty(), "claudication should expand via synonym");
    }

    #[test]
    fn test_synonym_cant_swallow() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["can't swallow", "chest pain"]);
        assert!(!results.is_empty(), "can't swallow should expand to difficulty swallowing");
    }

    #[test]
    fn test_synonym_wobbly_walking() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["wobbly walking", "memory problems"]);
        assert!(!results.is_empty(), "wobbly walking should expand to gait disturbance");
    }
}

// ── v0.22.0 scorer tests ──

#[cfg(test)]
mod tests_v22 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_copd() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chronic cough", "shortness of breath", "wheezing"]);
        let copd = results.iter().find(|r| r.disease_name == "Chronic Obstructive Pulmonary Disease");
        assert!(copd.is_some(), "COPD should appear");
        assert!(copd.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_pulmonary_fibrosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["progressive shortness of breath", "dry cough", "clubbing of fingers"]);
        let pf = results.iter().find(|r| r.disease_name == "Pulmonary Fibrosis");
        assert!(pf.is_some(), "Pulmonary Fibrosis should appear");
    }

    #[test]
    fn test_score_schizophrenia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["auditory hallucinations", "delusions", "social withdrawal"]);
        let sz = results.iter().find(|r| r.disease_name == "Schizophrenia");
        assert!(sz.is_some(), "Schizophrenia should appear");
        assert!(sz.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_ocd() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["intrusive thoughts", "compulsive behaviors", "anxiety"]);
        let ocd = results.iter().find(|r| r.disease_name == "Obsessive-Compulsive Disorder");
        assert!(ocd.is_some(), "OCD should appear");
    }

    #[test]
    fn test_score_ptsd() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["flashbacks", "nightmares", "hypervigilance"]);
        let ptsd = results.iter().find(|r| r.disease_name == "Post-Traumatic Stress Disorder");
        assert!(ptsd.is_some(), "PTSD should appear");
        assert!(ptsd.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_menieres() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["episodic vertigo", "tinnitus", "ear fullness"]);
        let md = results.iter().find(|r| r.disease_name == "Meniere's Disease");
        assert!(md.is_some(), "Meniere's Disease should appear");
    }

    #[test]
    fn test_score_rheumatoid_arthritis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["symmetric joint pain", "morning stiffness lasting over 1 hour", "joint swelling"]);
        let ra = results.iter().find(|r| r.disease_name == "Rheumatoid Arthritis");
        assert!(ra.is_some(), "Rheumatoid Arthritis should appear");
        assert!(ra.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_ankylosing_spondylitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chronic low back pain", "morning stiffness improving with exercise", "reduced spinal mobility"]);
        let as_ = results.iter().find(|r| r.disease_name == "Ankylosing Spondylitis");
        assert!(as_.is_some(), "Ankylosing Spondylitis should appear");
    }

    #[test]
    fn test_score_epiglottitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe sore throat", "difficulty swallowing", "drooling", "high fever"]);
        let ep = results.iter().find(|r| r.disease_name == "Epiglottitis");
        assert!(ep.is_some(), "Epiglottitis should appear");
        assert!(ep.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_pyelonephritis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["flank pain", "high fever", "chills", "painful urination"]);
        let pyelo = results.iter().find(|r| r.disease_name == "Pyelonephritis");
        assert!(pyelo.is_some(), "Pyelonephritis should appear");
        assert!(pyelo.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_actinic_keratosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["rough scaly skin patches", "sandpaper-like texture"]);
        let ak = results.iter().find(|r| r.disease_name == "Actinic Keratosis");
        assert!(ak.is_some(), "Actinic Keratosis should appear");
    }

    #[test]
    fn test_synonym_hearing_voices() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["hearing voices", "paranoid"]);
        assert!(!results.is_empty(), "hearing voices should expand via synonym");
    }

    #[test]
    fn test_synonym_cant_breathe() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["can't breathe", "wheezing"]);
        assert!(!results.is_empty(), "can't breathe should expand to shortness of breath");
    }

    #[test]
    fn test_synonym_lower_back_pain() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["lower back pain", "stiff joints"]);
        assert!(!results.is_empty(), "lower back pain should expand");
    }

    #[test]
    fn test_synonym_itchy_skin() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["itchy skin", "jaundice"]);
        assert!(!results.is_empty(), "itchy skin should expand to pruritus");
    }

    #[test]
    fn test_negative_evidence_copd() {
        let conn = db::init_memory_database().unwrap();
        let with_rash = score_symptoms(&conn, &["chronic cough", "shortness of breath", "rash"]);
        let without_rash = score_symptoms(&conn, &["chronic cough", "shortness of breath"]);
        let copd_with = with_rash.iter().find(|r| r.disease_name == "Chronic Obstructive Pulmonary Disease");
        let copd_without = without_rash.iter().find(|r| r.disease_name == "Chronic Obstructive Pulmonary Disease");
        if let (Some(cw), Some(cwo)) = (copd_with, copd_without) {
            assert!(cwo.probability >= cw.probability,
                "COPD should score same or lower with rash (negative evidence)");
        }
    }

    // ── v0.23.0 tests ──────────────────────────────────────────────────

    #[test]
    fn test_score_sepsis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "rapid heart rate", "confusion", "rapid breathing"]);
        let sepsis = results.iter().find(|r| r.disease_name == "Sepsis");
        assert!(sepsis.is_some(), "Sepsis should appear for fever+tachycardia+confusion+tachypnea");
        assert!(sepsis.unwrap().probability > 0.3);
    }

    #[test]
    fn test_score_atrial_fibrillation() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["irregular heartbeat", "palpitations", "fatigue"]);
        let afib = results.iter().find(|r| r.disease_name == "Atrial Fibrillation");
        assert!(afib.is_some(), "AFib should appear for irregular heartbeat+palpitations");
        assert!(afib.unwrap().probability > 0.2);
    }

    #[test]
    fn test_score_whooping_cough() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe coughing fits", "whoop sound on inspiration", "vomiting"]);
        let pertussis = results.iter().find(|r| r.disease_name == "Whooping Cough (Pertussis)");
        assert!(pertussis.is_some(), "Pertussis should match coughing fits + whoop");
    }

    #[test]
    fn test_score_allergic_rhinitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sneezing", "runny nose", "itchy eyes", "nasal congestion"]);
        let rhinitis = results.iter().find(|r| r.disease_name == "Allergic Rhinitis");
        assert!(rhinitis.is_some(), "Allergic rhinitis should match sneezing+runny nose+itchy eyes");
    }

    #[test]
    fn test_score_osteoarthritis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["joint pain", "joint stiffness", "crepitus"]);
        let oa = results.iter().find(|r| r.disease_name == "Osteoarthritis");
        assert!(oa.is_some(), "OA should appear for joint pain + stiffness + crepitus");
    }

    #[test]
    fn test_score_sleep_apnea() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["loud snoring", "excessive daytime sleepiness", "gasping during sleep"]);
        let osa = results.iter().find(|r| r.disease_name == "Obstructive Sleep Apnea");
        assert!(osa.is_some(), "OSA should match snoring+sleepiness+gasping");
    }

    #[test]
    fn test_score_peripheral_neuropathy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["numbness in hands and feet", "tingling", "burning pain"]);
        let pn = results.iter().find(|r| r.disease_name == "Peripheral Neuropathy");
        assert!(pn.is_some(), "Peripheral neuropathy should match numbness+tingling+burning");
    }

    #[test]
    fn test_score_thyroid_storm() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "rapid heart rate", "agitation", "tremor", "sweating"]);
        let ts = results.iter().find(|r| r.disease_name == "Thyroid Storm");
        assert!(ts.is_some(), "Thyroid storm should match fever+tachycardia+agitation");
    }

    #[test]
    fn test_score_pcos() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["irregular periods", "excess hair growth", "acne", "weight gain"]);
        let pcos = results.iter().find(|r| r.disease_name == "Polycystic Ovary Syndrome (PCOS)");
        assert!(pcos.is_some(), "PCOS should match irregular periods+hirsutism+acne");
    }

    #[test]
    fn test_score_stemi() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe chest pain", "chest pain radiating to left arm", "sweating", "shortness of breath"]);
        let stemi = results.iter().find(|r| r.disease_name == "Myocardial Infarction (STEMI)");
        assert!(stemi.is_some(), "STEMI should match severe chest pain + radiation + diaphoresis");
        assert!(stemi.unwrap().probability > 0.3);
    }

    #[test]
    fn test_score_preeclampsia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high blood pressure", "proteinuria", "headache", "visual disturbances"]);
        let pe = results.iter().find(|r| r.disease_name == "Preeclampsia");
        assert!(pe.is_some(), "Preeclampsia should match hypertension+proteinuria+headache");
    }

    #[test]
    fn test_score_chronic_kidney_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "decreased urine output", "swelling in legs", "nausea"]);
        let ckd = results.iter().find(|r| r.disease_name == "Chronic Kidney Disease");
        assert!(ckd.is_some(), "CKD should match fatigue+oliguria+edema");
    }

    #[test]
    fn test_synonym_snoring() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["snoring", "always sleepy"]);
        assert!(!results.is_empty(), "snoring synonym should expand");
    }

    #[test]
    fn test_synonym_numb_hands() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["numb hands", "burning feet"]);
        assert!(!results.is_empty(), "numb hands synonym should expand");
    }

    #[test]
    fn test_negative_evidence_afib() {
        let conn = db::init_memory_database().unwrap();
        let with_fever = score_symptoms(&conn, &["irregular heartbeat", "palpitations", "fever"]);
        let without_fever = score_symptoms(&conn, &["irregular heartbeat", "palpitations"]);
        let afib_with = with_fever.iter().find(|r| r.disease_name == "Atrial Fibrillation");
        let afib_without = without_fever.iter().find(|r| r.disease_name == "Atrial Fibrillation");
        if let (Some(aw), Some(awo)) = (afib_with, afib_without) {
            assert!(awo.probability >= aw.probability,
                "AFib should score same or lower with fever (negative evidence)");
        }
    }
}

// ── v0.24.0 scorer tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests_v24 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_toxic_epidermal_necrolysis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["widespread skin peeling", "painful red skin", "fever"]);
        let ten = results.iter().find(|r| r.disease_name == "Toxic Epidermal Necrolysis");
        assert!(ten.is_some(), "TEN should appear");
        assert!(ten.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_heat_exhaustion() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["heavy sweating", "weakness", "dizziness", "nausea"]);
        let he = results.iter().find(|r| r.disease_name == "Heat Exhaustion");
        assert!(he.is_some(), "Heat Exhaustion should appear");
    }

    #[test]
    fn test_score_organophosphate_poisoning() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["excessive salivation", "miosis", "diarrhea", "muscle twitching"]);
        let op = results.iter().find(|r| r.disease_name == "Organophosphate Poisoning");
        assert!(op.is_some(), "Organophosphate Poisoning should appear");
        assert!(op.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_infantile_spasms() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(0), sex: None };
        let results = score_symptoms_with_context(&conn, &["sudden body flexion spasms", "developmental regression"], &child_ctx);
        let is = results.iter().find(|r| r.disease_name == "Infantile Spasms");
        assert!(is.is_some(), "Infantile Spasms should appear");
    }

    #[test]
    fn test_score_retinal_detachment() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden floaters", "flashes of light", "shadow or curtain over vision"]);
        let rd = results.iter().find(|r| r.disease_name == "Retinal Detachment");
        assert!(rd.is_some(), "Retinal Detachment should appear");
        assert!(rd.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_periorbital_cellulitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["eyelid swelling", "eyelid redness", "fever"]);
        let pc = results.iter().find(|r| r.disease_name == "Periorbital Cellulitis");
        assert!(pc.is_some(), "Periorbital Cellulitis should appear");
    }

    #[test]
    fn test_score_henoch_schonlein_purpura() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(6), sex: None };
        let results = score_symptoms_with_context(&conn, &["palpable purpura on legs and buttocks", "joint pain", "abdominal pain"], &child_ctx);
        let hsp = results.iter().find(|r| r.disease_name == "Henoch-Schönlein Purpura");
        assert!(hsp.is_some(), "HSP should appear");
        assert!(hsp.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_subdural_hematoma() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["headache", "confusion", "weakness on one side"]);
        let sdh = results.iter().find(|r| r.disease_name == "Subdural Hematoma");
        assert!(sdh.is_some(), "Subdural Hematoma should appear");
    }

    #[test]
    fn test_score_thyroid_nodule() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["neck lump", "difficulty swallowing"]);
        let tn = results.iter().find(|r| r.disease_name == "Thyroid Nodule");
        assert!(tn.is_some(), "Thyroid Nodule should appear");
    }

    #[test]
    fn test_score_cholesteatoma() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["foul-smelling ear discharge", "hearing loss"]);
        let ch = results.iter().find(|r| r.disease_name == "Cholesteatoma");
        assert!(ch.is_some(), "Cholesteatoma should appear");
    }

    #[test]
    fn test_synonym_eye_floaters() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["eye floaters", "seeing flashes"]);
        assert!(!results.is_empty(), "eye floaters + seeing flashes should match via synonyms");
    }

    #[test]
    fn test_synonym_lump_in_neck() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["lump in neck"]);
        assert!(!results.is_empty(), "lump in neck should match via synonym");
    }

    #[test]
    fn test_synonym_smelly_ear() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["smelly ear discharge", "hearing loss"]);
        assert!(!results.is_empty(), "smelly ear discharge should expand via synonym");
    }

    #[test]
    fn test_negative_evidence_heat_exhaustion() {
        let conn = db::init_memory_database().unwrap();
        let with_cough = score_symptoms(&conn, &["heavy sweating", "weakness", "cough"]);
        let without_cough = score_symptoms(&conn, &["heavy sweating", "weakness"]);
        let he_with = with_cough.iter().find(|r| r.disease_name == "Heat Exhaustion");
        let he_without = without_cough.iter().find(|r| r.disease_name == "Heat Exhaustion");
        if let (Some(hw), Some(hwo)) = (he_with, he_without) {
            assert!(hwo.probability >= hw.probability,
                "Heat Exhaustion should score same or lower with cough (negative evidence)");
        }
    }
}

// ── v0.25.0 scorer tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests_v25 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_bruxism() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["teeth grinding", "jaw pain", "headache"]);
        let br = results.iter().find(|r| r.disease_name == "Bruxism");
        assert!(br.is_some(), "Bruxism should appear");
        assert!(br.unwrap().probability > 20.0);
    }

    #[test]
    fn test_score_tmj() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["jaw pain", "clicking or popping jaw", "difficulty chewing"]);
        let tmj = results.iter().find(|r| r.disease_name == "Temporomandibular Joint Disorder");
        assert!(tmj.is_some(), "TMJ should appear");
        assert!(tmj.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_gad() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["persistent worry", "restlessness", "insomnia", "muscle tension"]);
        let gad = results.iter().find(|r| r.disease_name == "Generalized Anxiety Disorder");
        assert!(gad.is_some(), "GAD should appear");
        assert!(gad.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_anorexia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["extreme weight loss", "fear of gaining weight", "restricted food intake"]);
        let an = results.iter().find(|r| r.disease_name == "Eating Disorder (Anorexia Nervosa)");
        assert!(an.is_some(), "Anorexia should appear");
        assert!(an.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_rickets() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(3), sex: None };
        let results = score_symptoms_with_context(&conn, &["bowed legs", "bone pain", "delayed growth"], &child_ctx);
        let rick = results.iter().find(|r| r.disease_name == "Rickets");
        assert!(rick.is_some(), "Rickets should appear");
    }

    #[test]
    fn test_score_febrile_seizure() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(2), sex: None };
        let results = score_symptoms_with_context(&conn, &["seizure with fever", "high fever", "loss of consciousness"], &child_ctx);
        let fs = results.iter().find(|r| r.disease_name == "Febrile Seizure");
        assert!(fs.is_some(), "Febrile Seizure should appear");
    }

    #[test]
    fn test_score_pku() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["intellectual disability", "musty body odor", "seizures"]);
        let pku = results.iter().find(|r| r.disease_name == "Phenylketonuria (PKU)");
        assert!(pku.is_some(), "PKU should appear");
    }

    #[test]
    fn test_score_bulimia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["binge eating episodes", "self-induced vomiting", "dental erosion"]);
        let bul = results.iter().find(|r| r.disease_name == "Bulimia Nervosa");
        assert!(bul.is_some(), "Bulimia should appear");
        assert!(bul.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_impetigo() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["honey-colored crusted sores", "red sores around mouth and nose", "itching"]);
        let imp = results.iter().find(|r| r.disease_name == "Impetigo");
        assert!(imp.is_some(), "Impetigo should appear");
        assert!(imp.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_ringworm() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["ring-shaped rash", "itchy red patches", "scaly skin"]);
        let rw = results.iter().find(|r| r.disease_name == "Tinea Corporis (Ringworm)");
        assert!(rw.is_some(), "Ringworm should appear");
        assert!(rw.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_metabolic_syndrome() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["large waist circumference", "high blood pressure", "high blood sugar"]);
        let ms = results.iter().find(|r| r.disease_name == "Metabolic Syndrome");
        assert!(ms.is_some(), "Metabolic Syndrome should appear");
        assert!(ms.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_gallstones() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden right upper abdominal pain", "pain after fatty meals", "nausea"]);
        let gs = results.iter().find(|r| r.disease_name == "Gallstones (Cholelithiasis)");
        assert!(gs.is_some(), "Gallstones should appear");
        assert!(gs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_chronic_fatigue() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe persistent fatigue", "post-exertional malaise", "unrefreshing sleep"]);
        let cfs = results.iter().find(|r| r.disease_name == "Chronic Fatigue Syndrome");
        assert!(cfs.is_some(), "CFS should appear");
        assert!(cfs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_peritonsillar_abscess() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe sore throat usually one-sided", "difficulty swallowing", "trismus", "high fever"]);
        let pta = results.iter().find(|r| r.disease_name == "Peritonsillar Abscess");
        assert!(pta.is_some(), "Peritonsillar Abscess should appear");
        assert!(pta.unwrap().probability > 30.0);
    }

    #[test]
    fn test_synonym_grinding_teeth() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["grinding teeth", "jaw pain"]);
        assert!(!results.is_empty(), "grinding teeth should expand via synonym");
    }

    #[test]
    fn test_synonym_anxious() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["anxious", "insomnia", "restlessness"]);
        assert!(!results.is_empty(), "anxious should expand via synonym");
    }

    #[test]
    fn test_synonym_always_tired() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["always tired", "crash after activity"]);
        assert!(!results.is_empty(), "always tired + crash after activity should match CFS via synonyms");
    }

    #[test]
    fn test_synonym_circular_rash() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["circular rash", "itchy"]);
        assert!(!results.is_empty(), "circular rash should expand to ring-shaped rash");
    }

    #[test]
    fn test_negative_evidence_bruxism() {
        let conn = db::init_memory_database().unwrap();
        let with_fever = score_symptoms(&conn, &["teeth grinding", "jaw pain", "fever"]);
        let without_fever = score_symptoms(&conn, &["teeth grinding", "jaw pain"]);
        let br_with = with_fever.iter().find(|r| r.disease_name == "Bruxism");
        let br_without = without_fever.iter().find(|r| r.disease_name == "Bruxism");
        if let (Some(bw), Some(bwo)) = (br_with, br_without) {
            assert!(bwo.probability >= bw.probability,
                "Bruxism should score same or lower with fever (negative evidence)");
        }
    }

    #[test]
    fn test_score_acoustic_neuroma() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["unilateral hearing loss", "tinnitus", "balance problems"]);
        let an = results.iter().find(|r| r.disease_name == "Acoustic Neuroma (Vestibular Schwannoma)");
        assert!(an.is_some(), "Acoustic Neuroma should appear");
        assert!(an.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_pericarditis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sharp chest pain worse with breathing", "chest pain improves leaning forward", "fever"]);
        let pc = results.iter().find(|r| r.disease_name == "Pericarditis");
        assert!(pc.is_some(), "Pericarditis should appear");
        assert!(pc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_testicular_torsion() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sudden severe testicular pain", "testicular swelling", "nausea"]);
        let tt = results.iter().find(|r| r.disease_name == "Testicular Torsion");
        assert!(tt.is_some(), "Testicular Torsion should appear");
        assert!(tt.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_necrotizing_fasciitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["pain out of proportion to exam findings", "rapidly spreading erythema", "crepitus"]);
        let nf = results.iter().find(|r| r.disease_name == "Necrotizing Fasciitis");
        assert!(nf.is_some(), "Necrotizing Fasciitis should appear");
        assert!(nf.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_carbon_monoxide() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["headache", "dizziness", "confusion", "cherry red skin"]);
        let co = results.iter().find(|r| r.disease_name == "Carbon Monoxide Poisoning");
        assert!(co.is_some(), "CO Poisoning should appear");
        assert!(co.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_aortic_stenosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["exertional dyspnea", "syncope with exertion", "systolic ejection murmur"]);
        let as_result = results.iter().find(|r| r.disease_name == "Aortic Stenosis");
        assert!(as_result.is_some(), "Aortic Stenosis should appear");
        assert!(as_result.unwrap().probability > 30.0);
    }

    #[test]
    fn test_negative_evidence_gad() {
        let conn = db::init_memory_database().unwrap();
        let with_cough = score_symptoms(&conn, &["persistent worry", "restlessness", "cough"]);
        let without_cough = score_symptoms(&conn, &["persistent worry", "restlessness"]);
        let gad_with = with_cough.iter().find(|r| r.disease_name == "Generalized Anxiety Disorder");
        let gad_without = without_cough.iter().find(|r| r.disease_name == "Generalized Anxiety Disorder");
        if let (Some(gw), Some(gwo)) = (gad_with, gad_without) {
            assert!(gwo.probability >= gw.probability,
                "GAD should score same or lower with cough (negative evidence)");
        }
    }
}

// ── v0.27.0 scorer tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests_v27 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_wernicke_encephalopathy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["confusion", "balance problems", "nystagmus"]);
        let we = results.iter().find(|r| r.disease_name == "Wernicke Encephalopathy");
        assert!(we.is_some(), "Wernicke Encephalopathy should appear");
        assert!(we.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_compartment_syndrome() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe pain disproportionate to injury", "pain with passive stretch", "tense compartment"]);
        let cs = results.iter().find(|r| r.disease_name == "Acute Compartment Syndrome");
        assert!(cs.is_some(), "Acute Compartment Syndrome should appear");
        assert!(cs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_frozen_shoulder() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["shoulder pain", "shoulder stiffness", "limited range of motion"]);
        let fs = results.iter().find(|r| r.disease_name == "Adhesive Capsulitis (Frozen Shoulder)");
        assert!(fs.is_some(), "Frozen Shoulder should appear");
        assert!(fs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_pellagra() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["skin rash in sun-exposed areas", "diarrhea", "confusion"]);
        let pl = results.iter().find(|r| r.disease_name == "Pellagra");
        assert!(pl.is_some(), "Pellagra should appear");
        assert!(pl.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_acromegaly() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["enlarged hands", "enlarged feet", "coarsened facial features"]);
        let ac = results.iter().find(|r| r.disease_name == "Acromegaly");
        assert!(ac.is_some(), "Acromegaly should appear");
        assert!(ac.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_pelvic_inflammatory_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["lower abdominal pain", "abnormal vaginal discharge", "painful intercourse"]);
        let pid = results.iter().find(|r| r.disease_name == "Pelvic Inflammatory Disease");
        assert!(pid.is_some(), "PID should appear");
    }

    #[test]
    fn test_score_toxic_megacolon() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal distension", "abdominal pain", "fever"]);
        let tm = results.iter().find(|r| r.disease_name == "Toxic Megacolon");
        assert!(tm.is_some(), "Toxic Megacolon should appear");
    }

    #[test]
    fn test_score_mastitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["breast pain", "breast redness", "breast swelling"]);
        let mast = results.iter().find(|r| r.disease_name == "Mastitis");
        assert!(mast.is_some(), "Mastitis should appear");
        assert!(mast.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_lichen_planus() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["purple flat-topped bumps", "itchy skin", "white lacy patches in mouth"]);
        let lp = results.iter().find(|r| r.disease_name == "Lichen Planus");
        assert!(lp.is_some(), "Lichen Planus should appear");
        assert!(lp.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_dengue_shock() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal pain", "persistent vomiting", "cold clammy skin", "rapid weak pulse"]);
        let ds = results.iter().find(|r| r.disease_name == "Dengue Shock Syndrome");
        assert!(ds.is_some(), "Dengue Shock Syndrome should appear");
        assert!(ds.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_placental_abruption() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["vaginal bleeding", "severe abdominal pain", "uterine tenderness"]);
        let pa = results.iter().find(|r| r.disease_name == "Placental Abruption");
        assert!(pa.is_some(), "Placental Abruption should appear");
    }

    #[test]
    fn test_synonym_frozen_shoulder() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["frozen shoulder", "pain worse at night"]);
        assert!(!results.is_empty(), "frozen shoulder should match via synonym");
    }

    #[test]
    fn test_synonym_big_hands() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["big hands", "big feet", "headache"]);
        assert!(!results.is_empty(), "big hands/feet should expand via synonym");
    }

    #[test]
    fn test_synonym_breast_infection() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["breast infection", "fever"]);
        assert!(!results.is_empty(), "breast infection should expand via synonym");
    }

    #[test]
    fn test_negative_evidence_wernicke() {
        let conn = db::init_memory_database().unwrap();
        let with_rash = score_symptoms(&conn, &["confusion", "balance problems", "rash"]);
        let without_rash = score_symptoms(&conn, &["confusion", "balance problems"]);
        let we_with = with_rash.iter().find(|r| r.disease_name == "Wernicke Encephalopathy");
        let we_without = without_rash.iter().find(|r| r.disease_name == "Wernicke Encephalopathy");
        if let (Some(ww), Some(wwo)) = (we_with, we_without) {
            assert!(wwo.probability >= ww.probability,
                "Wernicke should score same or lower with rash (negative evidence)");
        }
    }

    #[test]
    fn test_score_peripartum_cardiomyopathy() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["shortness of breath", "fatigue", "swelling in legs", "difficulty breathing when lying down"]);
        let pc = results.iter().find(|r| r.disease_name == "Peripartum Cardiomyopathy");
        assert!(pc.is_some(), "Peripartum Cardiomyopathy should appear");
    }
}

// ── v0.28.0 scorer tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests_v28 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_chikungunya() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "severe joint pain", "joint swelling", "rash"]);
        let ck = results.iter().find(|r| r.disease_name == "Chikungunya");
        assert!(ck.is_some(), "Chikungunya should appear");
        assert!(ck.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_visceral_leishmaniasis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["prolonged fever", "weight loss", "enlarged spleen"]);
        let vl = results.iter().find(|r| r.disease_name == "Leishmaniasis (Visceral)");
        assert!(vl.is_some(), "Visceral Leishmaniasis should appear");
        assert!(vl.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_schistosomiasis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["bloody urine", "abdominal pain", "blood in stool"]);
        let sc = results.iter().find(|r| r.disease_name == "Schistosomiasis");
        assert!(sc.is_some(), "Schistosomiasis should appear");
        assert!(sc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_leptospirosis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "muscle pain", "jaundice", "red eyes"]);
        let lp = results.iter().find(|r| r.disease_name == "Leptospirosis");
        assert!(lp.is_some(), "Leptospirosis should appear");
        assert!(lp.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_food_anaphylaxis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["throat swelling", "difficulty breathing", "widespread hives"]);
        let fa = results.iter().find(|r| r.disease_name == "Anaphylaxis (Food Allergy)");
        assert!(fa.is_some(), "Food Anaphylaxis should appear");
        assert!(fa.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_contact_dermatitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["itchy rash", "red skin", "dry cracked skin"]);
        let cd = results.iter().find(|r| r.disease_name == "Contact Dermatitis");
        assert!(cd.is_some(), "Contact Dermatitis should appear");
    }

    #[test]
    fn test_score_uti() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["painful urination", "frequent urination", "urgency to urinate"]);
        let uti = results.iter().find(|r| r.disease_name == "Urinary Tract Infection");
        assert!(uti.is_some(), "UTI should appear");
        assert!(uti.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_shingles() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["painful unilateral rash", "vesicular blisters in band pattern", "burning or tingling pain before rash"]);
        let hz = results.iter().find(|r| r.disease_name == "Herpes Zoster (Shingles)");
        assert!(hz.is_some(), "Shingles should appear");
        assert!(hz.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_optic_neuritis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["vision loss in one eye", "pain with eye movement", "color vision impairment"]);
        let on = results.iter().find(|r| r.disease_name == "Optic Neuritis");
        assert!(on.is_some(), "Optic Neuritis should appear");
        assert!(on.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_mesenteric_ischemia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe abdominal pain out of proportion to exam", "nausea", "vomiting"]);
        let mi = results.iter().find(|r| r.disease_name == "Mesenteric Ischemia (Acute)");
        assert!(mi.is_some(), "Mesenteric Ischemia should appear");
    }

    #[test]
    fn test_score_hyperaldosteronism() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["resistant hypertension", "muscle weakness", "muscle cramps"]);
        let ha = results.iter().find(|r| r.disease_name == "Hyperaldosteronism (Conn's Syndrome)");
        assert!(ha.is_some(), "Conn's Syndrome should appear");
    }

    #[test]
    fn test_score_thoracic_aneurysm() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["back pain between shoulder blades", "chest pain", "shortness of breath"]);
        let ta = results.iter().find(|r| r.disease_name == "Thoracic Aortic Aneurysm");
        assert!(ta.is_some(), "Thoracic Aortic Aneurysm should appear");
    }

    #[test]
    fn test_score_chronic_hep_b() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["fatigue", "right upper quadrant pain", "jaundice"]);
        let hb = results.iter().find(|r| r.disease_name == "Chronic Hepatitis B");
        assert!(hb.is_some(), "Chronic Hepatitis B should appear");
    }

    #[test]
    fn test_score_ibs() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["recurrent abdominal pain", "bloating", "altered bowel habits"]);
        let ibs = results.iter().find(|r| r.disease_name == "Irritable Bowel Syndrome");
        assert!(ibs.is_some(), "IBS should appear");
        assert!(ibs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_synonym_burning_pee() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["burning pee", "peeing a lot"]);
        assert!(!results.is_empty(), "burning pee + peeing a lot should expand via synonyms");
    }

    #[test]
    fn test_synonym_shingles() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["shingles", "band of blisters"]);
        assert!(!results.is_empty(), "shingles synonym should expand");
    }

    #[test]
    fn test_synonym_throat_closing() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["throat closing", "hives"]);
        assert!(!results.is_empty(), "throat closing should expand via synonym");
    }

    #[test]
    fn test_negative_evidence_chikungunya() {
        let conn = db::init_memory_database().unwrap();
        let with_cough = score_symptoms(&conn, &["high fever", "severe joint pain", "cough"]);
        let without_cough = score_symptoms(&conn, &["high fever", "severe joint pain"]);
        let ck_with = with_cough.iter().find(|r| r.disease_name == "Chikungunya");
        let ck_without = without_cough.iter().find(|r| r.disease_name == "Chikungunya");
        if let (Some(cw), Some(cwo)) = (ck_with, ck_without) {
            assert!(cwo.probability >= cw.probability,
                "Chikungunya should score same or lower with cough (negative evidence)");
        }
    }

    #[test]
    fn test_score_inguinal_hernia() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["groin bulge", "groin pain", "pain with coughing or straining"]);
        let ih = results.iter().find(|r| r.disease_name == "Inguinal Hernia");
        assert!(ih.is_some(), "Inguinal Hernia should appear");
        assert!(ih.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_hemorrhoids() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["rectal bleeding", "anal itching", "pain during bowel movements"]);
        let hm = results.iter().find(|r| r.disease_name == "Hemorrhoids");
        assert!(hm.is_some(), "Hemorrhoids should appear");
        assert!(hm.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_viral_conjunctivitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["red eye", "watery eye discharge", "gritty feeling in eye"]);
        let vc = results.iter().find(|r| r.disease_name == "Viral Conjunctivitis");
        assert!(vc.is_some(), "Viral Conjunctivitis should appear");
        assert!(vc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_acute_bronchitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["persistent cough", "mucus production", "chest discomfort"]);
        let ab = results.iter().find(|r| r.disease_name == "Acute Bronchitis");
        assert!(ab.is_some(), "Acute Bronchitis should appear");
        assert!(ab.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_viral_gastroenteritis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["watery diarrhea", "nausea", "vomiting", "abdominal cramps"]);
        let vg = results.iter().find(|r| r.disease_name == "Viral Gastroenteritis");
        assert!(vg.is_some(), "Viral Gastroenteritis should appear");
        assert!(vg.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_tonsillitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["sore throat", "difficulty swallowing", "swollen tonsils", "fever"]);
        let tn = results.iter().find(|r| r.disease_name == "Tonsillitis");
        assert!(tn.is_some(), "Tonsillitis should appear");
        assert!(tn.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_measles() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(5), sex: None };
        let results = score_symptoms_with_context(&conn, &["high fever", "maculopapular rash spreading head to toe", "Koplik spots"], &child_ctx);
        let ms = results.iter().find(|r| r.disease_name == "Measles");
        assert!(ms.is_some(), "Measles should appear");
        assert!(ms.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_chickenpox() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["itchy rash progressing to blisters", "vesicular rash in different stages", "fever"]);
        let cp = results.iter().find(|r| r.disease_name == "Chickenpox (Varicella)");
        assert!(cp.is_some(), "Chickenpox should appear");
        assert!(cp.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_mumps() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["parotid gland swelling", "jaw pain", "fever"]);
        let mp = results.iter().find(|r| r.disease_name == "Mumps");
        assert!(mp.is_some(), "Mumps should appear");
        assert!(mp.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_tetanus() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["jaw stiffness (lockjaw)", "muscle spasms", "difficulty swallowing"]);
        let tt = results.iter().find(|r| r.disease_name == "Tetanus");
        assert!(tt.is_some(), "Tetanus should appear");
        assert!(tt.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_yellow_fever() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["high fever", "jaundice", "bleeding from gums or nose"]);
        let yf = results.iter().find(|r| r.disease_name == "Yellow Fever");
        assert!(yf.is_some(), "Yellow Fever should appear");
        assert!(yf.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_chronic_urticaria() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["recurrent raised itchy welts", "hives lasting less than 24 hours each"]);
        let cu = results.iter().find(|r| r.disease_name == "Chronic Urticaria");
        assert!(cu.is_some(), "Chronic Urticaria should appear");
        assert!(cu.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_varicose_veins() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["visible enlarged twisted veins in legs", "leg heaviness", "leg aching"]);
        let vv = results.iter().find(|r| r.disease_name == "Varicose Veins");
        assert!(vv.is_some(), "Varicose Veins should appear");
        assert!(vv.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_bph() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["frequent urination", "weak urine stream", "nocturia"]);
        let bph = results.iter().find(|r| r.disease_name == "Benign Prostatic Hyperplasia");
        assert!(bph.is_some(), "BPH should appear");
        assert!(bph.unwrap().probability > 30.0);
    }

    #[test]
    fn test_synonym_pink_eye() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["pink eye", "watery eyes"]);
        assert!(!results.is_empty(), "pink eye should expand via synonym");
    }

    #[test]
    fn test_synonym_stomach_bug() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["stomach bug", "vomiting"]);
        assert!(!results.is_empty(), "stomach bug should expand via synonym");
    }

    #[test]
    fn test_synonym_lockjaw() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["lockjaw", "muscle spasms"]);
        assert!(!results.is_empty(), "lockjaw should expand via synonym");
    }

    #[test]
    fn test_synonym_hives() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["hives", "itchy"]);
        assert!(!results.is_empty(), "hives should expand via synonym");
    }

    #[test]
    fn test_synonym_heavy_legs() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["heavy legs", "bulging veins"]);
        assert!(!results.is_empty(), "heavy legs + bulging veins should expand via synonyms");
    }

    #[test]
    fn test_synonym_weak_pee_stream() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["weak pee stream", "peeing a lot at night"]);
        assert!(!results.is_empty(), "BPH synonyms should expand");
    }

    #[test]
    fn test_negative_evidence_hemorrhoids() {
        let conn = db::init_memory_database().unwrap();
        let with_fever = score_symptoms(&conn, &["rectal bleeding", "anal itching", "fever"]);
        let without_fever = score_symptoms(&conn, &["rectal bleeding", "anal itching"]);
        let hm_with = with_fever.iter().find(|r| r.disease_name == "Hemorrhoids");
        let hm_without = without_fever.iter().find(|r| r.disease_name == "Hemorrhoids");
        if let (Some(hw), Some(hwo)) = (hm_with, hm_without) {
            assert!(hwo.probability >= hw.probability,
                "Hemorrhoids should score same or lower with fever (negative evidence)");
        }
    }

    #[test]
    fn test_negative_evidence_conjunctivitis() {
        let conn = db::init_memory_database().unwrap();
        let with_vision_loss = score_symptoms(&conn, &["red eye", "watery eye discharge", "vision loss"]);
        let without_vision_loss = score_symptoms(&conn, &["red eye", "watery eye discharge"]);
        let vc_with = with_vision_loss.iter().find(|r| r.disease_name == "Viral Conjunctivitis");
        let vc_without = without_vision_loss.iter().find(|r| r.disease_name == "Viral Conjunctivitis");
        if let (Some(vw), Some(vwo)) = (vc_with, vc_without) {
            assert!(vwo.probability >= vw.probability,
                "Conjunctivitis should score same or lower with vision loss (negative evidence)");
        }
    }

    #[test]
    fn test_negative_evidence_contact_dermatitis() {
        let conn = db::init_memory_database().unwrap();
        let with_fever = score_symptoms(&conn, &["itchy rash", "red skin", "fever"]);
        let without_fever = score_symptoms(&conn, &["itchy rash", "red skin"]);
        let cd_with = with_fever.iter().find(|r| r.disease_name == "Contact Dermatitis");
        let cd_without = without_fever.iter().find(|r| r.disease_name == "Contact Dermatitis");
        if let (Some(cw), Some(cwo)) = (cd_with, cd_without) {
            assert!(cwo.probability >= cw.probability,
                "Contact Dermatitis should score same or lower with fever (negative evidence)");
        }
    }

    // v30 scorer tests
    #[test]
    fn test_score_periapical_abscess() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe toothache", "facial swelling", "foul taste in mouth"]);
        let pa = results.iter().find(|r| r.disease_name == "Periapical Abscess");
        assert!(pa.is_some(), "Periapical Abscess should appear");
        assert!(pa.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_dvt() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["leg swelling", "calf pain", "leg warmth"]);
        let dvt = results.iter().find(|r| r.disease_name == "Deep Vein Thrombosis");
        assert!(dvt.is_some(), "DVT should appear");
        assert!(dvt.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_organophosphate() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["excessive salivation", "miosis", "muscle fasciculations", "lacrimation"]);
        let op = results.iter().find(|r| r.disease_name == "Organophosphate Poisoning");
        assert!(op.is_some(), "Organophosphate Poisoning should appear");
        assert!(op.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_heat_stroke() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["core temperature > 40C", "altered mental status", "hot dry skin"]);
        let hs = results.iter().find(|r| r.disease_name == "Heat Stroke");
        assert!(hs.is_some(), "Heat Stroke should appear");
        assert!(hs.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_ankylosing_spondylitis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chronic lower back pain", "morning stiffness lasting > 30 min", "reduced spinal mobility"]);
        let as_ = results.iter().find(|r| r.disease_name == "Ankylosing Spondylitis");
        assert!(as_.is_some(), "Ankylosing Spondylitis should appear");
        assert!(as_.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_carbon_monoxide() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["headache", "dizziness", "confusion", "cherry-red skin"]);
        let co = results.iter().find(|r| r.disease_name == "Carbon Monoxide Poisoning");
        assert!(co.is_some(), "Carbon Monoxide Poisoning should appear");
        assert!(co.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_crush_syndrome() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["dark brown urine", "muscle weakness after compression", "swollen extremity"]);
        let cs = results.iter().find(|r| r.disease_name == "Crush Syndrome");
        assert!(cs.is_some(), "Crush Syndrome should appear");
        assert!(cs.unwrap().probability > 30.0);
    }
}

// ── v0.31.0 scorer tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests_v31 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_buruli_ulcer() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["painless skin nodule", "large painless ulcer with undermined edges"]);
        let bu = results.iter().find(|r| r.disease_name == "Buruli Ulcer");
        assert!(bu.is_some(), "Buruli Ulcer should appear");
        assert!(bu.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_guinea_worm() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["painful blister on lower limb", "worm emerging from skin"]);
        let gw = results.iter().find(|r| r.disease_name == "Dracunculiasis (Guinea Worm Disease)");
        assert!(gw.is_some(), "Guinea Worm should appear");
        assert!(gw.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_noma() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(4), sex: None };
        let results = score_symptoms_with_context(&conn, &["gum ulceration", "tissue necrosis of face", "fever"], &child_ctx);
        let nm = results.iter().find(|r| r.disease_name == "Noma (Cancrum Oris)");
        assert!(nm.is_some(), "Noma should appear");
        assert!(nm.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_ciguatera() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["nausea", "diarrhea", "temperature reversal (cold feels hot)"]);
        let cig = results.iter().find(|r| r.disease_name == "Ciguatera Fish Poisoning");
        assert!(cig.is_some(), "Ciguatera should appear");
        assert!(cig.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_ascariasis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["abdominal pain", "visible worms in stool", "abdominal distension"]);
        let asc = results.iter().find(|r| r.disease_name == "Ascariasis");
        assert!(asc.is_some(), "Ascariasis should appear");
        assert!(asc.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_kwashiorkor() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(3), sex: None };
        let results = score_symptoms_with_context(&conn, &["bilateral pedal edema", "distended abdomen", "hair discoloration (reddish-orange)"], &child_ctx);
        let kw = results.iter().find(|r| r.disease_name == "Kwashiorkor");
        assert!(kw.is_some(), "Kwashiorkor should appear");
        assert!(kw.unwrap().probability > 30.0);
    }

    #[test]
    fn test_synonym_worms_in_poop() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["worms in poop", "belly pain"]);
        assert!(!results.is_empty(), "worms in poop + belly pain should match via synonyms");
    }

    #[test]
    fn test_synonym_swollen_leg() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["swollen leg", "thickened skin"]);
        assert!(!results.is_empty(), "swollen leg should expand via synonym");
    }

    #[test]
    fn test_synonym_swollen_belly() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["swollen belly", "edema"]);
        assert!(!results.is_empty(), "swollen belly should expand via synonym");
    }

    #[test]
    fn test_negative_evidence_ciguatera() {
        let conn = db::init_memory_database().unwrap();
        let with_fever = score_symptoms(&conn, &["nausea", "diarrhea", "temperature reversal (cold feels hot)", "fever"]);
        let without_fever = score_symptoms(&conn, &["nausea", "diarrhea", "temperature reversal (cold feels hot)"]);
        let cig_with = with_fever.iter().find(|r| r.disease_name == "Ciguatera Fish Poisoning");
        let cig_without = without_fever.iter().find(|r| r.disease_name == "Ciguatera Fish Poisoning");
        if let (Some(cw), Some(cwo)) = (cig_with, cig_without) {
            assert!(cwo.probability >= cw.probability,
                "Ciguatera should score same or lower with fever (negative evidence)");
        }
    }
}

// ── v0.32.0 scorer tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests_v32 {
    use super::*;
    use crate::db;

    #[test]
    fn test_score_hookworm() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["abdominal pain", "fatigue", "iron-deficiency anemia"]);
        let hw = results.iter().find(|r| r.disease_name == "Hookworm Infection");
        assert!(hw.is_some(), "Hookworm Infection should appear");
        assert!(hw.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_trachoma() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["eye itching and irritation", "eye discharge", "progressive vision loss"]);
        let tr = results.iter().find(|r| r.disease_name == "Trachoma");
        assert!(tr.is_some(), "Trachoma should appear");
        assert!(tr.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_onchocerciasis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["severe itching", "subcutaneous nodules (onchocercomas)", "visual impairment"]);
        let oncho = results.iter().find(|r| r.disease_name == "Onchocerciasis (River Blindness)");
        assert!(oncho.is_some(), "Onchocerciasis should appear");
        assert!(oncho.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_myocardial_bridge() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chest pain during exercise", "chest pain relieved by rest", "palpitations"]);
        let mb = results.iter().find(|r| r.disease_name == "Myocardial Bridge");
        assert!(mb.is_some(), "Myocardial Bridge should appear");
        assert!(mb.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_lymphatic_filariasis() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["chronic limb swelling (lymphedema)", "thickened skin on limbs"]);
        let lf = results.iter().find(|r| r.disease_name == "Lymphatic Filariasis (Elephantiasis)");
        assert!(lf.is_some(), "Lymphatic Filariasis should appear");
        assert!(lf.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_acute_rheumatic_fever() {
        let conn = db::init_memory_database().unwrap();
        let child_ctx = PatientContext { age: Some(8), sex: None };
        let results = score_symptoms_with_context(&conn, &["migratory joint pain (polyarthritis)", "fever", "heart murmur"], &child_ctx);
        let arf = results.iter().find(|r| r.disease_name == "Acute Rheumatic Fever");
        assert!(arf.is_some(), "Acute Rheumatic Fever should appear");
        assert!(arf.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_sleeping_sickness() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["intermittent fever", "sleep disturbance (day sleeping, night insomnia)", "confusion"]);
        let ss = results.iter().find(|r| r.disease_name == "African Trypanosomiasis (Sleeping Sickness)");
        assert!(ss.is_some(), "African Trypanosomiasis should appear");
        assert!(ss.unwrap().probability > 30.0);
    }

    #[test]
    fn test_score_interstitial_lung_disease() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["progressive shortness of breath", "dry persistent cough", "clubbing of fingers and toes"]);
        let ild = results.iter().find(|r| r.disease_name == "Interstitial Lung Disease");
        assert!(ild.is_some(), "Interstitial Lung Disease should appear");
        assert!(ild.unwrap().probability > 30.0);
    }

    #[test]
    fn test_synonym_night_sweats() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["night sweats", "fever", "joint pain"]);
        assert!(!results.is_empty(), "night sweats should match via synonym");
    }

    #[test]
    fn test_synonym_elephant_leg() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["elephant leg", "thick skin on legs"]);
        assert!(!results.is_empty(), "elephant leg should expand via synonym");
    }

    #[test]
    fn test_synonym_exercise_chest_pain() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["exercise chest pain", "palpitations"]);
        assert!(!results.is_empty(), "exercise chest pain should expand via synonym");
    }

    #[test]
    fn test_synonym_wandering_joint_pain() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["wandering joint pain", "fever"]);
        assert!(!results.is_empty(), "wandering joint pain should expand via synonym");
    }

    #[test]
    fn test_synonym_finger_clubbing() {
        let conn = db::init_memory_database().unwrap();
        let results = score_symptoms(&conn, &["finger clubbing", "cough"]);
        assert!(!results.is_empty(), "finger clubbing should expand via synonym");
    }

    #[test]
    fn test_negative_evidence_hookworm() {
        let conn = db::init_memory_database().unwrap();
        let with_cough = score_symptoms(&conn, &["abdominal pain", "fatigue", "cough"]);
        let without_cough = score_symptoms(&conn, &["abdominal pain", "fatigue"]);
        let hw_with = with_cough.iter().find(|r| r.disease_name == "Hookworm Infection");
        let hw_without = without_cough.iter().find(|r| r.disease_name == "Hookworm Infection");
        if let (Some(hww), Some(hwo)) = (hw_with, hw_without) {
            assert!(hwo.probability >= hww.probability,
                "Hookworm should score same or lower with cough (negative evidence)");
        }
    }

    #[test]
    fn test_negative_evidence_sleeping_sickness() {
        let conn = db::init_memory_database().unwrap();
        let with_rash = score_symptoms(&conn, &["intermittent fever", "confusion", "rash"]);
        let without_rash = score_symptoms(&conn, &["intermittent fever", "confusion"]);
        let ss_with = with_rash.iter().find(|r| r.disease_name == "African Trypanosomiasis (Sleeping Sickness)");
        let ss_without = without_rash.iter().find(|r| r.disease_name == "African Trypanosomiasis (Sleeping Sickness)");
        if let (Some(sw), Some(swo)) = (ss_with, ss_without) {
            assert!(swo.probability >= sw.probability,
                "Sleeping Sickness should score same or lower with rash (negative evidence)");
        }
    }
}
