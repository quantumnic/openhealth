use colored::Colorize;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnsetSpeed {
    /// Seconds to minutes (anaphylaxis, stroke, pneumothorax)
    Hyperacute,
    /// Hours to a day (appendicitis, MI, meningitis)
    Acute,
    /// Days to weeks (infections, autoimmune flares)
    Subacute,
    /// Weeks to months (chronic diseases, cancers)
    Chronic,
}

impl OnsetSpeed {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hyperacute" | "sudden" | "seconds" | "minutes" => Some(Self::Hyperacute),
            "acute" | "hours" | "rapid" => Some(Self::Acute),
            "subacute" | "days" | "weeks" => Some(Self::Subacute),
            "chronic" | "months" | "gradual" | "slow" => Some(Self::Chronic),
            _ => None,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Hyperacute => "Hyperacute (seconds–minutes)",
            Self::Acute => "Acute (hours–1 day)",
            Self::Subacute => "Subacute (days–weeks)",
            Self::Chronic => "Chronic (weeks–months)",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Self::Hyperacute => "⚡",
            Self::Acute => "🔥",
            Self::Subacute => "📅",
            Self::Chronic => "🐢",
        }
    }
}

/// Maps diseases to their typical onset speed.
fn get_onset_map() -> Vec<(&'static str, OnsetSpeed)> {
    vec![
        // Hyperacute
        ("Anaphylaxis", OnsetSpeed::Hyperacute),
        ("Anaphylaxis (Food Allergy)", OnsetSpeed::Hyperacute),
        ("Anaphylactic Shock", OnsetSpeed::Hyperacute),
        ("Stroke", OnsetSpeed::Hyperacute),
        ("Heart Attack", OnsetSpeed::Hyperacute),
        ("Myocardial Infarction (STEMI)", OnsetSpeed::Hyperacute),
        ("Spontaneous Pneumothorax", OnsetSpeed::Hyperacute),
        ("Pulmonary Embolism", OnsetSpeed::Hyperacute),
        ("Aortic Dissection", OnsetSpeed::Hyperacute),
        ("Cardiac Arrest", OnsetSpeed::Hyperacute),
        ("Retinal Detachment", OnsetSpeed::Hyperacute),
        ("Testicular Torsion", OnsetSpeed::Hyperacute),
        ("Acute Compartment Syndrome", OnsetSpeed::Hyperacute),
        ("Mesenteric Ischemia (Acute)", OnsetSpeed::Hyperacute),
        // Acute
        ("Appendicitis", OnsetSpeed::Acute),
        ("Meningitis", OnsetSpeed::Acute),
        ("Cholera", OnsetSpeed::Acute),
        ("Diabetic Ketoacidosis", OnsetSpeed::Acute),
        ("Acute Pancreatitis", OnsetSpeed::Acute),
        ("Pneumonia", OnsetSpeed::Acute),
        ("Peritonitis", OnsetSpeed::Acute),
        ("Epiglottitis", OnsetSpeed::Acute),
        ("Ludwig Angina", OnsetSpeed::Acute),
        ("Sepsis", OnsetSpeed::Acute),
        ("Toxic Shock Syndrome", OnsetSpeed::Acute),
        ("Necrotizing Fasciitis", OnsetSpeed::Acute),
        ("Carbon Monoxide Poisoning", OnsetSpeed::Acute),
        ("Organophosphate Poisoning", OnsetSpeed::Acute),
        ("Botulism", OnsetSpeed::Acute),
        ("Heatstroke", OnsetSpeed::Acute),
        ("Hypothermia", OnsetSpeed::Acute),
        ("Pyelonephritis", OnsetSpeed::Acute),
        ("Cholangitis", OnsetSpeed::Acute),
        ("Urinary Tract Infection", OnsetSpeed::Acute),
        ("Leptospirosis", OnsetSpeed::Acute),
        ("Herpes Zoster (Shingles)", OnsetSpeed::Acute),
        ("Optic Neuritis", OnsetSpeed::Acute),
        // Subacute
        ("Malaria", OnsetSpeed::Subacute),
        ("Dengue", OnsetSpeed::Subacute),
        ("Chikungunya", OnsetSpeed::Subacute),
        ("Typhoid Fever", OnsetSpeed::Subacute),
        ("Tuberculosis", OnsetSpeed::Subacute),
        ("Mononucleosis", OnsetSpeed::Subacute),
        ("COVID-19", OnsetSpeed::Subacute),
        ("Influenza", OnsetSpeed::Subacute),
        ("Common Cold", OnsetSpeed::Subacute),
        ("Schistosomiasis", OnsetSpeed::Subacute),
        ("Kawasaki Disease", OnsetSpeed::Subacute),
        ("Myocarditis", OnsetSpeed::Subacute),
        ("Guillain-Barré Syndrome", OnsetSpeed::Subacute),
        ("Bell's Palsy", OnsetSpeed::Subacute),
        ("Contact Dermatitis", OnsetSpeed::Subacute),
        // Chronic
        ("Chronic Obstructive Pulmonary Disease", OnsetSpeed::Chronic),
        ("Pulmonary Fibrosis", OnsetSpeed::Chronic),
        ("Rheumatoid Arthritis", OnsetSpeed::Chronic),
        ("Osteoarthritis", OnsetSpeed::Chronic),
        ("Multiple Sclerosis", OnsetSpeed::Chronic),
        ("Parkinson's Disease", OnsetSpeed::Chronic),
        ("Alzheimer's Disease", OnsetSpeed::Chronic),
        ("Chronic Kidney Disease", OnsetSpeed::Chronic),
        ("Chronic Hepatitis B", OnsetSpeed::Chronic),
        ("Hashimoto's Thyroiditis", OnsetSpeed::Chronic),
        ("Irritable Bowel Syndrome", OnsetSpeed::Chronic),
        ("Fibromyalgia", OnsetSpeed::Chronic),
        ("Chronic Fatigue Syndrome", OnsetSpeed::Chronic),
        ("Metabolic Syndrome", OnsetSpeed::Chronic),
        ("Ankylosing Spondylitis", OnsetSpeed::Chronic),
        ("Acromegaly", OnsetSpeed::Chronic),
        ("Leishmaniasis (Visceral)", OnsetSpeed::Chronic),
        ("Thoracic Aortic Aneurysm", OnsetSpeed::Chronic),
        ("Hyperaldosteronism (Conn's Syndrome)", OnsetSpeed::Chronic),
    ]
}

#[derive(Serialize)]
struct OnsetResult {
    onset_type: String,
    diseases: Vec<OnsetDisease>,
}

#[derive(Serialize)]
struct OnsetDisease {
    name: String,
    severity: String,
    description: String,
}

/// Run the onset command — filter diseases by onset speed and optionally cross-reference symptoms.
pub fn run(conn: &Connection, onset_input: &str, symptoms: Option<&str>, json: bool) {
    let speed = match OnsetSpeed::from_str(onset_input) {
        Some(s) => s,
        None => {
            if json {
                println!("{{\"error\": \"Unknown onset type. Use: sudden, acute, subacute, chronic\"}}");
            } else {
                println!("{}", "Unknown onset type. Use: sudden, acute (hours), subacute (days), chronic (weeks+)".red());
            }
            return;
        }
    };

    let onset_map = get_onset_map();
    let matching_names: Vec<&str> = onset_map
        .iter()
        .filter(|(_, s)| *s == speed)
        .map(|(name, _)| *name)
        .collect();

    if matching_names.is_empty() {
        if json {
            println!("{{\"onset_type\": \"{}\", \"diseases\": []}}", speed.label());
        } else {
            println!("{}", "No diseases mapped for this onset type.".yellow());
        }
        return;
    }

    // Fetch disease details from DB
    let placeholders: Vec<String> = matching_names.iter().enumerate().map(|(i, _)| format!("?{}", i + 1)).collect();
    let query = format!(
        "SELECT name, severity, description FROM diseases WHERE name IN ({})",
        placeholders.join(", ")
    );

    let mut stmt = conn.prepare(&query).unwrap();
    let params: Vec<&dyn rusqlite::types::ToSql> = matching_names.iter().map(|n| n as &dyn rusqlite::types::ToSql).collect();
    let diseases: Vec<OnsetDisease> = stmt
        .query_map(params.as_slice(), |row| {
            Ok(OnsetDisease {
                name: row.get(0)?,
                severity: row.get(1)?,
                description: row.get(2)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // If symptoms provided, filter to diseases that match at least one symptom
    let filtered = if let Some(sym_input) = symptoms {
        let sym_list: Vec<String> = sym_input
            .split([',', ';'])
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        if sym_list.is_empty() {
            diseases
        } else {
            // For each disease, check if it has matching symptoms
            let mut result = Vec::new();
            for d in diseases {
                let mut s_stmt = conn.prepare(
                    "SELECT s.name FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id JOIN diseases di ON di.id = ds.disease_id WHERE di.name = ?1"
                ).unwrap();
                let syms: Vec<String> = s_stmt
                    .query_map([&d.name], |row| row.get(0))
                    .unwrap()
                    .filter_map(|r| r.ok())
                    .collect();

                let has_match = sym_list.iter().any(|input| {
                    syms.iter().any(|s| {
                        let sl = s.to_lowercase();
                        sl.contains(input.as_str()) || input.contains(sl.as_str())
                    })
                });

                if has_match {
                    result.push(d);
                }
            }
            result
        }
    } else {
        diseases
    };

    if json {
        let result = OnsetResult {
            onset_type: speed.label().to_string(),
            diseases: filtered,
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return;
    }

    println!(
        "\n{} {}",
        speed.emoji(),
        format!("═══ {} Onset Diseases ═══", speed.label()).bold().cyan()
    );

    if let Some(sym) = symptoms {
        println!("{}", format!("  Filtered by symptoms: {}\n", sym).dimmed());
    } else {
        println!();
    }

    if filtered.is_empty() {
        println!("{}", "  No matching diseases found for this onset + symptom combination.".yellow());
    } else {
        for d in &filtered {
            let severity_colored = match d.severity.as_str() {
                "high" => "🔴 HIGH".red().bold(),
                "medium" => "🟡 MEDIUM".yellow().bold(),
                _ => "🟢 LOW".green().bold(),
            };
            println!("  {} [{}]", d.name.bold(), severity_colored);
            println!("    {}", d.description.dimmed());
        }
    }

    println!(
        "\n{}",
        "Tip: Combine with 'openhealth symptoms' for full scoring.\n".dimmed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onset_from_str() {
        assert_eq!(OnsetSpeed::from_str("sudden"), Some(OnsetSpeed::Hyperacute));
        assert_eq!(OnsetSpeed::from_str("acute"), Some(OnsetSpeed::Acute));
        assert_eq!(OnsetSpeed::from_str("days"), Some(OnsetSpeed::Subacute));
        assert_eq!(OnsetSpeed::from_str("chronic"), Some(OnsetSpeed::Chronic));
        assert_eq!(OnsetSpeed::from_str("xyz"), None);
    }

    #[test]
    fn test_onset_labels() {
        assert!(!OnsetSpeed::Hyperacute.label().is_empty());
        assert!(!OnsetSpeed::Acute.emoji().is_empty());
    }

    #[test]
    fn test_onset_map_has_entries() {
        let map = get_onset_map();
        assert!(map.len() > 50, "Should have 50+ mapped diseases");
    }

    #[test]
    fn test_onset_run_json() {
        let conn = crate::db::init_memory_database().unwrap();
        run(&conn, "sudden", None, true);
    }

    #[test]
    fn test_onset_run_with_symptoms() {
        let conn = crate::db::init_memory_database().unwrap();
        run(&conn, "acute", Some("fever, headache"), true);
    }

    #[test]
    fn test_onset_run_unknown() {
        let conn = crate::db::init_memory_database().unwrap();
        run(&conn, "xyz", None, true);
    }
}
