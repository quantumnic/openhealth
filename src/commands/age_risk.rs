use colored::Colorize;
use rusqlite::Connection;

struct AgeRisk {
    disease: &'static str,
    risk_note: &'static str,
}

pub fn run(conn: &Connection, age: u8, sex: Option<&str>, json: bool) {
    let risks = get_risks_for_age(age, sex);

    if json {
        let json_out: Vec<serde_json::Value> = risks
            .iter()
            .map(|r| {
                serde_json::json!({
                    "disease": r.disease,
                    "risk_note": r.risk_note,
                    "age": age,
                    "sex": sex.unwrap_or("unspecified"),
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_out).unwrap());
        return;
    }

    let age_group = match age {
        0 => "Neonate",
        1..=4 => "Toddler",
        5..=12 => "Child",
        13..=17 => "Adolescent",
        18..=39 => "Young Adult",
        40..=64 => "Middle-Aged Adult",
        65..=79 => "Senior",
        _ => "Elderly (80+)",
    };

    println!(
        "\n{}",
        "╔══════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║       🎂 Age-Specific Health Risk Profile        ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════╝".cyan()
    );

    print!("\n  👤 Age: {} ({})", age, age_group);
    if let Some(s) = sex {
        print!("  |  Sex: {}", s);
    }
    println!("\n");

    // Count diseases in DB for this age group
    let age_group_filter = match age {
        0 => "neonates",
        1..=17 => "children",
        _ => "adults",
    };

    let db_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM diseases WHERE age_group = ?1 OR age_group = 'all'",
            [age_group_filter],
            |r| r.get(0),
        )
        .unwrap_or(0);

    println!(
        "  📊 {} diseases in database relevant to your age group\n",
        db_count
    );

    if risks.is_empty() {
        println!("  No specific heightened risks identified for this age/sex.\n");
        return;
    }

    println!("  {} Key risks to watch for:\n", "⚠️".yellow());

    for (i, r) in risks.iter().enumerate() {
        println!("  {}. {}", i + 1, r.disease.bold());
        println!("     {}", r.risk_note);
        println!();
    }

    println!(
        "  {}",
        "💡 Discuss age-appropriate screening with your healthcare provider."
            .dimmed()
    );
    println!(
        "  {}",
        "📋 Use `openhealth screen` for detailed screening recommendations.\n"
            .dimmed()
    );
}

fn get_risks_for_age(age: u8, sex: Option<&str>) -> Vec<AgeRisk> {
    let mut risks = Vec::new();
    let is_male = sex.map(|s| s.to_lowercase()) == Some("male".to_string());
    let is_female = sex.map(|s| s.to_lowercase()) == Some("female".to_string());

    // Neonates
    if age == 0 {
        risks.push(AgeRisk { disease: "Neonatal Sepsis", risk_note: "Watch for poor feeding, lethargy, temperature instability. Seek care immediately." });
        risks.push(AgeRisk { disease: "Neonatal Jaundice", risk_note: "Common in first week. Monitor yellowing of skin/eyes. Phototherapy if bilirubin elevated." });
        risks.push(AgeRisk { disease: "Pyloric Stenosis", risk_note: "Projectile vomiting at 2-8 weeks. More common in firstborn males. Surgical correction." });
    }

    // Toddlers and young children
    if (1..=5).contains(&age) {
        risks.push(AgeRisk { disease: "Febrile Seizure", risk_note: "Most common age 6 months-5 years. Usually benign but frightening. Manage fever proactively." });
        risks.push(AgeRisk { disease: "Croup", risk_note: "Barking cough and stridor, typically at night. Steam/cool air helps. Seek care for stridor at rest." });
        risks.push(AgeRisk { disease: "Intussusception", risk_note: "Intermittent severe crying, currant jelly stool. Surgical emergency. Peak age 5-9 months." });
        risks.push(AgeRisk { disease: "Acute Rheumatic Fever", risk_note: "Following untreated strep throat. Can cause permanent heart damage. Prompt antibiotic treatment prevents it." });
    }

    // School-age children
    if (5..=12).contains(&age) {
        risks.push(AgeRisk { disease: "Acute Otitis Media", risk_note: "Very common ear infection. Antibiotics often needed. Watch for fever and ear pain." });
        risks.push(AgeRisk { disease: "Rickets", risk_note: "Vitamin D deficiency, especially with low sunlight exposure. Supplement recommended." });
        risks.push(AgeRisk { disease: "Impetigo", risk_note: "Highly contagious skin infection. Common in school settings. Treat with topical/oral antibiotics." });
    }

    // Adolescents
    if (13..=17).contains(&age) {
        risks.push(AgeRisk { disease: "Eating Disorders", risk_note: "Peak onset in adolescence. Watch for extreme dieting, body image distortion, weight changes." });
        risks.push(AgeRisk { disease: "Mental Health", risk_note: "Depression, anxiety, and self-harm increase. Open communication and screening essential." });
        if is_male {
            risks.push(AgeRisk { disease: "Testicular Torsion", risk_note: "Peak age 12-18. Sudden severe testicular pain = surgical emergency within 6 hours." });
        }
        if is_female {
            risks.push(AgeRisk { disease: "Polycystic Ovary Syndrome (PCOS)", risk_note: "Irregular periods, acne, excess hair. Early diagnosis improves long-term outcomes." });
        }
    }

    // Young adults
    if (18..=39).contains(&age) {
        risks.push(AgeRisk { disease: "STIs / HIV", risk_note: "Peak transmission age. Regular screening, safe sex practices, PrEP if at risk." });
        risks.push(AgeRisk { disease: "Mental Health", risk_note: "Peak onset for many psychiatric conditions (schizophrenia, bipolar). Seek help early." });
        if is_female {
            risks.push(AgeRisk { disease: "Preeclampsia", risk_note: "Monitor blood pressure during pregnancy. Early detection saves lives." });
            risks.push(AgeRisk { disease: "Endometriosis", risk_note: "Severe period pain is NOT normal. Average diagnostic delay is 7-10 years." });
        }
    }

    // Middle-aged
    if (40..=64).contains(&age) {
        risks.push(AgeRisk { disease: "Cardiovascular Disease", risk_note: "Leading cause of death. Screen blood pressure, cholesterol, glucose regularly." });
        risks.push(AgeRisk { disease: "Type 2 Diabetes", risk_note: "Risk increases with age. Screen fasting glucose/HbA1c every 3 years from age 45." });
        risks.push(AgeRisk { disease: "Colorectal Cancer", risk_note: "Screening recommended from age 45. Colonoscopy every 10 years or stool tests annually." });
        if is_female && age >= 50 {
            risks.push(AgeRisk { disease: "Breast Cancer", risk_note: "Mammography screening every 1-2 years. Self-examination monthly." });
            risks.push(AgeRisk { disease: "Osteoporosis", risk_note: "Bone density declines post-menopause. Calcium, vitamin D, weight-bearing exercise." });
        }
        if is_male && age >= 50 {
            risks.push(AgeRisk { disease: "Prostate Cancer", risk_note: "Discuss PSA screening with physician. Shared decision-making recommended." });
        }
    }

    // Seniors
    if age >= 65 {
        risks.push(AgeRisk { disease: "Falls and Fractures", risk_note: "Leading cause of injury. Home safety, exercise, vision checks, medication review." });
        risks.push(AgeRisk { disease: "Dementia / Alzheimer's", risk_note: "Risk doubles every 5 years after 65. Cognitive screening, social engagement, exercise help." });
        risks.push(AgeRisk { disease: "Pneumonia", risk_note: "Higher mortality in elderly. Annual flu + pneumococcal vaccination critical." });
        risks.push(AgeRisk { disease: "Atrial Fibrillation", risk_note: "Increases stroke risk 5x. Pulse check, ECG screening. Anticoagulation if detected." });
        risks.push(AgeRisk { disease: "Chronic Kidney Disease", risk_note: "Often asymptomatic until advanced. Annual creatinine/GFR screening." });
        if is_male {
            risks.push(AgeRisk { disease: "Benign Prostatic Hyperplasia", risk_note: "Very common after 60. Urinary symptoms treatable. See urologist if symptomatic." });
        }
    }

    risks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_age_risk_neonate() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 0, None, false);
    }

    #[test]
    fn test_age_risk_child() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 5, None, false);
    }

    #[test]
    fn test_age_risk_adolescent_male() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 15, Some("male"), false);
    }

    #[test]
    fn test_age_risk_adult_female() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 30, Some("female"), false);
    }

    #[test]
    fn test_age_risk_middle_aged() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 55, Some("male"), false);
    }

    #[test]
    fn test_age_risk_senior_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 70, Some("female"), true);
    }

    #[test]
    fn test_age_risk_elderly() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, 85, None, false);
    }

    #[test]
    fn test_get_risks_returns_entries() {
        let risks = get_risks_for_age(0, None);
        assert!(!risks.is_empty(), "Neonates should have age-specific risks");
        let risks = get_risks_for_age(70, Some("male"));
        assert!(risks.len() >= 4, "Seniors should have multiple risks");
    }
}
