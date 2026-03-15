use colored::Colorize;
use rusqlite::Connection;
use std::collections::HashMap;

struct Complication {
    condition: &'static str,
    risk: &'static str,
    timeframe: &'static str,
    preventable: bool,
}

pub fn run(conn: &Connection, name: &str, json: bool) {
    let disease_name = resolve_disease_name(conn, name);

    let complications_map = build_complications_map();
    let key = disease_name.as_deref().unwrap_or(name);

    if let Some(complications) = complications_map.get(key) {
        if json {
            let json_out: Vec<serde_json::Value> = complications
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "disease": key,
                        "complication": c.condition,
                        "risk": c.risk,
                        "timeframe": c.timeframe,
                        "preventable": c.preventable,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_out).unwrap());
            return;
        }

        println!(
            "\n{}",
            "╔══════════════════════════════════════════════════╗".cyan()
        );
        println!(
            "{}",
            "║      ⚠️  Potential Complications                ║".cyan()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════════════╝".cyan()
        );
        println!("\n  Disease: {}\n", key.bold());

        for (i, c) in complications.iter().enumerate() {
            let risk_color = match c.risk {
                "high" => c.risk.red().bold().to_string(),
                "moderate" => c.risk.yellow().to_string(),
                _ => c.risk.green().to_string(),
            };
            let preventable_str = if c.preventable { "✅ Yes" } else { "❌ No" };

            println!("  {}. {}", i + 1, c.condition.bold());
            println!("     Risk: {}  |  Timeframe: {}", risk_color, c.timeframe);
            println!("     Preventable: {}", preventable_str);
            println!();
        }

        println!(
            "  {}",
            "⚕️  Early treatment significantly reduces complication risk."
                .yellow()
        );
        println!(
            "  {}",
            "📋 Discuss monitoring strategies with your healthcare provider.\n"
                .dimmed()
        );
    } else {
        if json {
            println!(
                "{}",
                serde_json::json!({"error": format!("No complication data for '{}'", name)})
            );
            return;
        }
        eprintln!(
            "\n  {} No complication data found for '{}'.",
            "⚠️".yellow(),
            name
        );
        eprintln!("  Try: openhealth complication \"diabetes\"\n");
    }
}

fn resolve_disease_name(conn: &Connection, name: &str) -> Option<String> {
    let mut stmt = conn
        .prepare("SELECT name FROM diseases WHERE name LIKE ?1 LIMIT 1")
        .ok()?;
    let pattern = format!("%{}%", name);
    stmt.query_row([&pattern], |row| row.get(0)).ok()
}

fn build_complications_map() -> HashMap<&'static str, Vec<Complication>> {
    let mut map = HashMap::new();

    map.insert(
        "Diabetes Type 2",
        vec![
            Complication { condition: "Diabetic retinopathy (vision loss)", risk: "high", timeframe: "5-10 years", preventable: true },
            Complication { condition: "Diabetic nephropathy (kidney failure)", risk: "high", timeframe: "10-20 years", preventable: true },
            Complication { condition: "Peripheral neuropathy", risk: "high", timeframe: "5-10 years", preventable: true },
            Complication { condition: "Cardiovascular disease", risk: "high", timeframe: "ongoing", preventable: true },
            Complication { condition: "Diabetic foot ulcers / amputation", risk: "moderate", timeframe: "years", preventable: true },
        ],
    );

    map.insert(
        "Hypertension",
        vec![
            Complication { condition: "Stroke", risk: "high", timeframe: "years", preventable: true },
            Complication { condition: "Heart failure", risk: "high", timeframe: "years", preventable: true },
            Complication { condition: "Chronic kidney disease", risk: "moderate", timeframe: "years", preventable: true },
            Complication { condition: "Retinopathy", risk: "moderate", timeframe: "years", preventable: true },
            Complication { condition: "Aortic aneurysm", risk: "low", timeframe: "years", preventable: true },
        ],
    );

    map.insert(
        "Malaria",
        vec![
            Complication { condition: "Cerebral malaria (coma, seizures)", risk: "high", timeframe: "days", preventable: true },
            Complication { condition: "Severe anemia", risk: "high", timeframe: "days-weeks", preventable: true },
            Complication { condition: "Acute kidney injury", risk: "moderate", timeframe: "days", preventable: true },
            Complication { condition: "Pulmonary edema / ARDS", risk: "moderate", timeframe: "days", preventable: true },
            Complication { condition: "Splenic rupture", risk: "low", timeframe: "weeks", preventable: false },
        ],
    );

    map.insert(
        "Pneumonia",
        vec![
            Complication { condition: "Sepsis", risk: "high", timeframe: "days", preventable: true },
            Complication { condition: "Pleural effusion / empyema", risk: "moderate", timeframe: "days-weeks", preventable: true },
            Complication { condition: "Lung abscess", risk: "low", timeframe: "weeks", preventable: true },
            Complication { condition: "Respiratory failure", risk: "moderate", timeframe: "days", preventable: true },
        ],
    );

    map.insert(
        "Appendicitis",
        vec![
            Complication { condition: "Perforation and peritonitis", risk: "high", timeframe: "24-72 hours", preventable: true },
            Complication { condition: "Abscess formation", risk: "moderate", timeframe: "days", preventable: true },
            Complication { condition: "Sepsis", risk: "moderate", timeframe: "days", preventable: true },
        ],
    );

    map.insert(
        "Cholera",
        vec![
            Complication { condition: "Severe dehydration and shock", risk: "high", timeframe: "hours", preventable: true },
            Complication { condition: "Hypokalemia (cardiac arrhythmia)", risk: "high", timeframe: "hours", preventable: true },
            Complication { condition: "Acute kidney failure", risk: "moderate", timeframe: "hours-days", preventable: true },
            Complication { condition: "Hypoglycemia (especially children)", risk: "moderate", timeframe: "hours", preventable: true },
        ],
    );

    map.insert(
        "Measles",
        vec![
            Complication { condition: "Pneumonia", risk: "high", timeframe: "days", preventable: true },
            Complication { condition: "Encephalitis", risk: "moderate", timeframe: "days-weeks", preventable: true },
            Complication { condition: "Subacute sclerosing panencephalitis (SSPE)", risk: "low", timeframe: "years", preventable: false },
            Complication { condition: "Corneal ulceration / blindness", risk: "moderate", timeframe: "weeks", preventable: true },
        ],
    );

    map.insert(
        "Heart Attack",
        vec![
            Complication { condition: "Heart failure", risk: "high", timeframe: "days-weeks", preventable: true },
            Complication { condition: "Cardiac arrhythmia", risk: "high", timeframe: "hours-days", preventable: true },
            Complication { condition: "Cardiogenic shock", risk: "moderate", timeframe: "hours", preventable: true },
            Complication { condition: "Ventricular rupture", risk: "low", timeframe: "days", preventable: false },
        ],
    );

    map.insert(
        "Stroke",
        vec![
            Complication { condition: "Paralysis / disability", risk: "high", timeframe: "immediate", preventable: false },
            Complication { condition: "Aspiration pneumonia", risk: "moderate", timeframe: "days-weeks", preventable: true },
            Complication { condition: "Deep vein thrombosis", risk: "moderate", timeframe: "weeks", preventable: true },
            Complication { condition: "Depression", risk: "moderate", timeframe: "weeks-months", preventable: true },
        ],
    );

    map.insert(
        "Acute Rheumatic Fever",
        vec![
            Complication { condition: "Rheumatic heart disease (valve damage)", risk: "high", timeframe: "weeks-months", preventable: true },
            Complication { condition: "Heart failure", risk: "moderate", timeframe: "months-years", preventable: true },
            Complication { condition: "Sydenham's chorea", risk: "moderate", timeframe: "weeks", preventable: true },
        ],
    );

    map.insert(
        "Hookworm Infection",
        vec![
            Complication { condition: "Severe iron-deficiency anemia", risk: "high", timeframe: "months", preventable: true },
            Complication { condition: "Growth retardation in children", risk: "high", timeframe: "months-years", preventable: true },
            Complication { condition: "Protein malnutrition", risk: "moderate", timeframe: "months", preventable: true },
            Complication { condition: "Cognitive impairment in children", risk: "moderate", timeframe: "years", preventable: true },
        ],
    );

    map.insert(
        "Trachoma",
        vec![
            Complication { condition: "Corneal scarring", risk: "high", timeframe: "years", preventable: true },
            Complication { condition: "Irreversible blindness", risk: "high", timeframe: "years-decades", preventable: true },
            Complication { condition: "Trichiasis (inturned eyelashes)", risk: "high", timeframe: "years", preventable: true },
        ],
    );

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_complication_diabetes() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "diabetes", false);
    }

    #[test]
    fn test_complication_malaria_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "malaria", true);
    }

    #[test]
    fn test_complication_unknown() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "xyzzy_not_a_disease", false);
    }

    #[test]
    fn test_complication_map_populated() {
        let map = build_complications_map();
        assert!(map.len() >= 10, "Should have complications for at least 10 diseases");
    }
}
