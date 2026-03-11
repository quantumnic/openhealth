use crate::engine::severity::SeverityLevel;
use colored::*;
use rusqlite::Connection;

/// Body system groupings for categories.
fn system_for_category(category: &str) -> &'static str {
    match category {
        "respiratory" => "Respiratory System",
        "cardiovascular" => "Cardiovascular System",
        "gastrointestinal" => "Digestive System",
        "neurological" => "Nervous System",
        "dermatological" => "Skin & Integumentary",
        "musculoskeletal" => "Musculoskeletal System",
        "urological" | "renal" => "Urinary System",
        "ophthalmological" => "Eyes & Vision",
        "ENT" => "Ear, Nose & Throat",
        "endocrine" | "metabolic" => "Endocrine & Metabolic",
        "hematological" => "Blood & Lymphatic",
        "immunological" | "autoimmune" => "Immune System",
        "mental_health" => "Mental Health",
        "obstetric" | "gynecological" => "Reproductive System",
        "neonatal" | "pediatric" => "Pediatric",
        "nutritional" => "Nutrition & Metabolism",
        "surgical" | "trauma" => "Trauma & Surgery",
        "anesthesiology" => "Anesthesiology",
        _ => "General / Other",
    }
}

/// List all body systems with disease counts, or show diseases in a specific system.
pub fn run(conn: &Connection, system_filter: Option<&str>, json: bool) {
    let mut stmt = conn
        .prepare("SELECT name, severity, category FROM diseases ORDER BY category, name")
        .unwrap();
    let diseases: Vec<(String, String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // Group by body system
    let mut systems: std::collections::BTreeMap<&str, Vec<(&str, &str)>> =
        std::collections::BTreeMap::new();
    for (name, severity, category) in &diseases {
        let system = system_for_category(category);
        systems
            .entry(system)
            .or_default()
            .push((name.as_str(), severity.as_str()));
    }

    if let Some(filter) = system_filter {
        let filter_lower = filter.to_lowercase();
        let matched_system = systems.keys().find(|k| k.to_lowercase().contains(&filter_lower));

        if let Some(&system_name) = matched_system {
            let entries = &systems[system_name];
            if json {
                let items: Vec<serde_json::Value> = entries
                    .iter()
                    .map(|(n, s)| serde_json::json!({"name": n, "severity": s}))
                    .collect();
                let obj = serde_json::json!({
                    "system": system_name,
                    "disease_count": entries.len(),
                    "diseases": items,
                });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
                return;
            }
            println!("{}", format!("━━━ {} ━━━", system_name).bold());
            println!("  {} diseases\n", entries.len());
            for (name, severity) in entries {
                let sev = SeverityLevel::from_str(severity);
                println!("    {} {}", sev.emoji(), name);
            }
            println!();
        } else if json {
            println!("{{\"error\": \"Body system '{}' not found.\"}}", filter);
        } else {
            println!("{}", format!("Body system matching '{filter}' not found.").red());
            println!("\nAvailable systems:");
            for system in systems.keys() {
                println!("  • {system}");
            }
        }
        return;
    }

    // Show overview of all systems
    if json {
        let items: Vec<serde_json::Value> = systems
            .iter()
            .map(|(system, entries)| {
                serde_json::json!({
                    "system": system,
                    "disease_count": entries.len(),
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&items).unwrap());
        return;
    }

    println!("{}", "━━━ Body Systems Overview ━━━".bold());
    println!();
    for (system, entries) in &systems {
        let severe = entries.iter().filter(|(_, s)| *s == "high").count();
        let indicator = if severe > 0 {
            format!(" (🔴 {} critical)", severe).red().to_string()
        } else {
            String::new()
        };
        println!(
            "  🫀 {} — {} diseases{}",
            system.bold(),
            entries.len(),
            indicator
        );
    }
    println!();
    println!("  Total: {} diseases across {} body systems", diseases.len(), systems.len());
    println!("  Drill down: openhealth body-system \"respiratory\"");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_body_system_overview() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, None, false);
    }

    #[test]
    fn test_body_system_filter() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, Some("respiratory"), false);
    }

    #[test]
    fn test_body_system_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, None, true);
    }

    #[test]
    fn test_body_system_not_found() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, Some("xyznothing"), false);
    }

    #[test]
    fn test_system_for_category_coverage() {
        // Ensure common categories map properly
        assert_eq!(system_for_category("respiratory"), "Respiratory System");
        assert_eq!(system_for_category("cardiovascular"), "Cardiovascular System");
        assert_eq!(system_for_category("mental_health"), "Mental Health");
        assert_eq!(system_for_category("unknown_cat"), "General / Other");
    }
}
