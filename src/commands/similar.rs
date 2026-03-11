use crate::display;
use crate::engine::severity::SeverityLevel;
use colored::*;
use rusqlite::Connection;

/// Find diseases most similar to a given disease based on symptom overlap (Jaccard similarity).
pub fn run(conn: &Connection, name: &str, limit: usize, json: bool) {
    let target = conn.query_row(
        "SELECT id, name FROM diseases WHERE name LIKE ?1",
        [format!("%{name}%")],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
    );

    let (target_id, target_name) = match target {
        Ok(t) => t,
        Err(_) => {
            if json {
                println!("{{\"error\": \"Disease '{}' not found.\"}}", name);
            } else {
                println!("{}", format!("Disease '{name}' not found.").red());
            }
            return;
        }
    };

    let target_symptoms = get_symptom_ids(conn, target_id);
    if target_symptoms.is_empty() {
        if json {
            println!("[]");
        } else {
            println!("No symptoms found for '{target_name}'.");
        }
        return;
    }

    let mut stmt = conn
        .prepare("SELECT id, name, severity FROM diseases WHERE id != ?1")
        .unwrap();
    let others: Vec<(i64, String, String)> = stmt
        .query_map([target_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let mut similarities: Vec<(String, String, f64, usize, usize)> = Vec::new();

    for (other_id, other_name, other_sev) in &others {
        let other_symptoms = get_symptom_ids(conn, *other_id);
        if other_symptoms.is_empty() {
            continue;
        }

        let intersection = target_symptoms.iter().filter(|s| other_symptoms.contains(s)).count();
        if intersection == 0 {
            continue;
        }
        let union = target_symptoms.len() + other_symptoms.len() - intersection;
        let jaccard = intersection as f64 / union as f64;

        similarities.push((
            other_name.clone(),
            other_sev.clone(),
            jaccard,
            intersection,
            union,
        ));
    }

    similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    similarities.truncate(limit);

    if json {
        let entries: Vec<serde_json::Value> = similarities
            .iter()
            .map(|(name, severity, score, shared, _)| {
                serde_json::json!({
                    "name": name,
                    "severity": severity,
                    "similarity": (score * 100.0).round() / 100.0,
                    "shared_symptoms": shared,
                })
            })
            .collect();
        let obj = serde_json::json!({
            "target": target_name,
            "similar_diseases": entries,
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return;
    }

    display::print_banner();
    println!("{}", format!("━━━ Diseases Similar to {} ━━━", target_name).bold());
    println!();

    if similarities.is_empty() {
        println!("  No similar diseases found.");
        return;
    }

    for (i, (name, severity, score, shared, _)) in similarities.iter().enumerate() {
        let sev = SeverityLevel::from_str(severity);
        let pct = (score * 100.0).round();
        println!(
            "  {}. {} {} — {:.0}% similar ({} shared symptoms)",
            i + 1,
            sev.emoji(),
            name.bold(),
            pct,
            shared,
        );
    }
    println!();
}

fn get_symptom_ids(conn: &Connection, disease_id: i64) -> Vec<i64> {
    let mut stmt = conn
        .prepare("SELECT symptom_id FROM disease_symptoms WHERE disease_id = ?1")
        .unwrap();
    stmt.query_map([disease_id], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_similar_malaria() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "malaria", 5, false);
    }

    #[test]
    fn test_similar_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "malaria", 3, true);
    }

    #[test]
    fn test_similar_not_found() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "xyznothing", 5, false);
    }

    #[test]
    fn test_similar_returns_results() {
        let conn = db::init_memory_database().unwrap();
        // Malaria should have similar diseases (e.g., Dengue, Typhoid)
        let target_id: i64 = conn
            .query_row("SELECT id FROM diseases WHERE name = 'Malaria'", [], |r| r.get(0))
            .unwrap();
        let syms = get_symptom_ids(&conn, target_id);
        assert!(!syms.is_empty());
    }
}
