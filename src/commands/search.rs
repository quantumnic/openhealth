use colored::*;
use rusqlite::Connection;

pub fn run(conn: &Connection, query: &str, json: bool) {
    let query_lower = query.to_lowercase();
    let pattern = format!("%{query_lower}%");

    // Search in symptoms
    let mut sym_stmt = conn
        .prepare(
            "SELECT DISTINCT s.name FROM symptoms s WHERE LOWER(s.name) LIKE ?1 ORDER BY s.name",
        )
        .unwrap();
    let symptoms: Vec<String> = sym_stmt
        .query_map([&pattern], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // Search in diseases
    let mut dis_stmt = conn
        .prepare(
            "SELECT name, category, severity FROM diseases WHERE LOWER(name) LIKE ?1 OR LOWER(description) LIKE ?1 ORDER BY name",
        )
        .unwrap();
    let diseases: Vec<(String, String, String)> = dis_stmt
        .query_map([&pattern], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    if json {
        let obj = serde_json::json!({
            "query": query,
            "matching_symptoms": symptoms,
            "matching_diseases": diseases.iter().map(|(n, c, s)| serde_json::json!({"name": n, "category": c, "severity": s})).collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return;
    }

    if symptoms.is_empty() && diseases.is_empty() {
        println!("{}", format!("No results for '{query}'.").yellow());
        return;
    }

    println!("{}", format!("━━━ Search: '{query}' ━━━").bold());
    println!();

    if !symptoms.is_empty() {
        println!("  {} ({} found)", "Matching Symptoms:".underline(), symptoms.len());
        for sym in &symptoms {
            // Find which diseases have this symptom
            let mut d_stmt = conn
                .prepare(
                    "SELECT d.name FROM diseases d JOIN disease_symptoms ds ON d.id = ds.disease_id JOIN symptoms s ON s.id = ds.symptom_id WHERE s.name = ?1 ORDER BY d.name",
                )
                .unwrap();
            let related: Vec<String> = d_stmt
                .query_map([sym], |row| row.get(0))
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();
            println!("    🔍 {} → {}", sym.bold(), related.join(", ").dimmed());
        }
        println!();
    }

    if !diseases.is_empty() {
        println!("  {} ({} found)", "Matching Diseases:".underline(), diseases.len());
        for (name, category, severity) in &diseases {
            let emoji = match severity.as_str() {
                "high" => "🔴",
                "medium" => "🟡",
                _ => "🟢",
            };
            println!("    {emoji} {} [{}]", name.bold(), category.bright_cyan());
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_search_fever() {
        let conn = db::init_memory_database().unwrap();
        // Should not panic and should find results
        run(&conn, "fever", false);
    }

    #[test]
    fn test_search_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "malaria", true);
    }

    #[test]
    fn test_search_no_results() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "xyznonexistent", false);
    }
}
