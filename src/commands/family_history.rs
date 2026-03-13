use rusqlite::Connection;
use colored::*;

/// Family history risk assessment.
/// Given a list of diseases in the family, identify diseases the user may be at increased risk for.
pub fn run(conn: &Connection, history: &str, json: bool) {
    let items: Vec<&str> = history
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if items.is_empty() {
        eprintln!("Please provide family history items, e.g. \"diabetes, heart attack, breast cancer\"");
        return;
    }

    // Map family history conditions to diseases with increased risk
    let mappings = get_family_risk_mappings();

    let mut at_risk: Vec<RiskEntry> = Vec::new();

    for input in &items {
        let input_lower = input.to_lowercase();
        for mapping in &mappings {
            let condition_match = mapping.triggers.iter().any(|t| {
                let t_lower = t.to_lowercase();
                input_lower.contains(&t_lower) || t_lower.contains(&input_lower)
            });
            if condition_match {
                for disease_name in &mapping.increases_risk_of {
                    // Verify disease exists in DB
                    let exists: bool = conn
                        .query_row(
                            "SELECT COUNT(*) FROM diseases WHERE name = ?1 COLLATE NOCASE",
                            [disease_name],
                            |r| r.get::<_, i64>(0),
                        )
                        .map(|c| c > 0)
                        .unwrap_or(false);

                    if exists && !at_risk.iter().any(|r| r.disease == *disease_name) {
                        at_risk.push(RiskEntry {
                            disease: disease_name.to_string(),
                            because_of: input.to_string(),
                            risk_level: mapping.risk_level.to_string(),
                            screening: mapping.screening.to_string(),
                        });
                    }
                }
            }
        }
    }

    if json {
        let json_data: Vec<serde_json::Value> = at_risk
            .iter()
            .map(|r| {
                serde_json::json!({
                    "disease": r.disease,
                    "family_condition": r.because_of,
                    "risk_level": r.risk_level,
                    "screening_recommendation": r.screening,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
        return;
    }

    println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║       👨‍👩‍👧‍👦  Family History Risk Assessment                ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    println!("{}: {}", "Family history".bold(), items.join(", "));
    println!();

    if at_risk.is_empty() {
        println!("{}", "No specific increased risks identified from the given family history.".yellow());
        println!("This doesn't mean zero risk — discuss with your healthcare provider.");
        return;
    }

    println!("{} disease(s) with increased risk identified:\n", at_risk.len());

    for entry in &at_risk {
        let emoji = match entry.risk_level.as_str() {
            "high" => "🔴",
            "moderate" => "🟡",
            _ => "🟢",
        };
        println!(
            "  {} {} {}",
            emoji,
            entry.disease.bold(),
            format!("(risk: {})", entry.risk_level).dimmed()
        );
        println!(
            "    Family link: {}",
            entry.because_of
        );
        println!(
            "    📋 Screening: {}",
            entry.screening
        );
        println!();
    }

    println!("{}", "⚠️  Family history increases risk but does not guarantee disease.".yellow());
    println!("{}", "   Discuss screening plans with your healthcare provider.".yellow());
}

struct RiskEntry {
    disease: String,
    because_of: String,
    risk_level: String,
    screening: String,
}

struct FamilyRiskMapping {
    triggers: Vec<&'static str>,
    increases_risk_of: Vec<&'static str>,
    risk_level: &'static str,
    screening: &'static str,
}

fn get_family_risk_mappings() -> Vec<FamilyRiskMapping> {
    vec![
        FamilyRiskMapping {
            triggers: vec!["breast cancer"],
            increases_risk_of: vec!["Breast Cancer", "Ovarian Cancer"],
            risk_level: "high",
            screening: "Mammogram starting 10 years before youngest family diagnosis (but not before 25). Consider genetic testing for BRCA1/2.",
        },
        FamilyRiskMapping {
            triggers: vec!["colon cancer", "colorectal cancer", "bowel cancer"],
            increases_risk_of: vec!["Colorectal Cancer"],
            risk_level: "high",
            screening: "Colonoscopy starting age 40 or 10 years before youngest family diagnosis. Every 5 years.",
        },
        FamilyRiskMapping {
            triggers: vec!["heart attack", "heart disease", "coronary artery disease"],
            increases_risk_of: vec!["Heart Attack", "Atrial Fibrillation", "Hypertension"],
            risk_level: "high",
            screening: "Lipid panel and cardiovascular risk assessment from age 20. Regular blood pressure checks. Consider calcium score CT.",
        },
        FamilyRiskMapping {
            triggers: vec!["diabetes", "type 2 diabetes"],
            increases_risk_of: vec!["Diabetes Type 2", "Prediabetes"],
            risk_level: "high",
            screening: "Fasting glucose or HbA1c from age 30 (or earlier if overweight). Annual screening if prediabetic.",
        },
        FamilyRiskMapping {
            triggers: vec!["stroke"],
            increases_risk_of: vec!["Stroke", "Hypertension", "Atrial Fibrillation"],
            risk_level: "moderate",
            screening: "Blood pressure monitoring. Lipid panel. Consider carotid ultrasound if multiple family members affected.",
        },
        FamilyRiskMapping {
            triggers: vec!["melanoma", "skin cancer"],
            increases_risk_of: vec!["Melanoma"],
            risk_level: "moderate",
            screening: "Annual full-body skin exam by dermatologist. Monthly self-exams. Strict sun protection.",
        },
        FamilyRiskMapping {
            triggers: vec!["lung cancer"],
            increases_risk_of: vec!["Lung Cancer"],
            risk_level: "moderate",
            screening: "Low-dose CT if smoker or ex-smoker with family history. Smoking cessation is the strongest intervention.",
        },
        FamilyRiskMapping {
            triggers: vec!["osteoporosis"],
            increases_risk_of: vec!["Osteoporosis"],
            risk_level: "moderate",
            screening: "DEXA scan at menopause or age 50 for men. Ensure adequate calcium and vitamin D. Weight-bearing exercise.",
        },
        FamilyRiskMapping {
            triggers: vec!["asthma"],
            increases_risk_of: vec!["Asthma"],
            risk_level: "moderate",
            screening: "Spirometry if symptoms develop. Avoid smoking and environmental triggers. Monitor respiratory health.",
        },
        FamilyRiskMapping {
            triggers: vec!["thyroid disease", "hypothyroidism", "hyperthyroidism", "graves", "hashimoto"],
            increases_risk_of: vec!["Graves' Disease", "Hashimoto's Thyroiditis", "Hypothyroidism", "Hyperthyroidism"],
            risk_level: "moderate",
            screening: "TSH screening every 5 years from age 35, more frequently if symptoms develop.",
        },
        FamilyRiskMapping {
            triggers: vec!["prostate cancer"],
            increases_risk_of: vec!["Prostate Cancer"],
            risk_level: "moderate",
            screening: "PSA testing discussion from age 40 (10 years earlier than general population). Annual if positive family history.",
        },
        FamilyRiskMapping {
            triggers: vec!["aortic aneurysm", "AAA"],
            increases_risk_of: vec!["Aortic Aneurysm", "Abdominal Aortic Aneurysm Rupture"],
            risk_level: "high",
            screening: "Abdominal ultrasound screening for first-degree relatives. Especially men over 50 who smoke.",
        },
        FamilyRiskMapping {
            triggers: vec!["epilepsy", "seizures"],
            increases_risk_of: vec!["Epilepsy"],
            risk_level: "low",
            screening: "No routine screening. EEG if suspicious symptoms develop. Genetic counseling for rare familial epilepsy syndromes.",
        },
        FamilyRiskMapping {
            triggers: vec!["parkinson", "parkinson's"],
            increases_risk_of: vec!["Parkinson's Disease"],
            risk_level: "moderate",
            screening: "No routine screening available. Be aware of early symptoms (tremor, slowness). Genetic counseling for early-onset family cases.",
        },
        FamilyRiskMapping {
            triggers: vec!["alzheimer", "dementia"],
            increases_risk_of: vec!["Alzheimer's Disease"],
            risk_level: "moderate",
            screening: "Cognitive screening if symptoms. Cardiovascular health, exercise, and social engagement reduce risk. Consider ApoE4 genetic testing.",
        },
        FamilyRiskMapping {
            triggers: vec!["celiac", "celiac disease"],
            increases_risk_of: vec!["Celiac Disease"],
            risk_level: "moderate",
            screening: "Serologic testing (tTG-IgA) if symptoms develop. First-degree relatives have ~10% lifetime risk.",
        },
        FamilyRiskMapping {
            triggers: vec!["sickle cell", "sickle cell disease"],
            increases_risk_of: vec!["Sickle Cell Disease"],
            risk_level: "high",
            screening: "Hemoglobin electrophoresis for carrier detection. Genetic counseling before pregnancy if both parents carriers.",
        },
        FamilyRiskMapping {
            triggers: vec!["hemophilia"],
            increases_risk_of: vec!["Hemophilia"],
            risk_level: "high",
            screening: "Coagulation studies for at-risk family members. Genetic counseling. Carrier testing for females in affected families.",
        },
        FamilyRiskMapping {
            triggers: vec!["marfan", "marfan syndrome"],
            increases_risk_of: vec!["Marfan Syndrome", "Aortic Dissection"],
            risk_level: "high",
            screening: "Clinical evaluation and echocardiogram for first-degree relatives. Genetic testing available. Annual aortic imaging.",
        },
        FamilyRiskMapping {
            triggers: vec!["depression", "bipolar"],
            increases_risk_of: vec!["Depression", "Bipolar Disorder"],
            risk_level: "moderate",
            screening: "Mental health awareness. PHQ-9 screening. Early intervention if mood changes. Lifestyle factors (sleep, exercise, social connection).",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_family_history_diabetes() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "diabetes", false);
    }

    #[test]
    fn test_family_history_multiple() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "breast cancer, heart attack, diabetes", false);
    }

    #[test]
    fn test_family_history_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "melanoma", true);
    }

    #[test]
    fn test_family_risk_mappings_not_empty() {
        let mappings = get_family_risk_mappings();
        assert!(mappings.len() >= 15, "Should have at least 15 risk mappings");
    }
}
