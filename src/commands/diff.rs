use crate::display;
use crate::engine::severity::SeverityLevel;
use colored::*;
use rusqlite::Connection;

/// Compare two diseases side-by-side for differential diagnosis.
pub fn run(conn: &Connection, disease_a: &str, disease_b: &str, json: bool) {
    let a = load_disease(conn, disease_a);
    let b = load_disease(conn, disease_b);

    let (a_id, a_name, a_desc, a_sev) = match a {
        Some(d) => d,
        None => {
            let msg = format!("Disease '{disease_a}' not found.");
            if json { println!("{{\"error\": \"{msg}\"}}"); } else { println!("{}", msg.red()); }
            return;
        }
    };
    let (b_id, b_name, b_desc, b_sev) = match b {
        Some(d) => d,
        None => {
            let msg = format!("Disease '{disease_b}' not found.");
            if json { println!("{{\"error\": \"{msg}\"}}"); } else { println!("{}", msg.red()); }
            return;
        }
    };

    let a_symptoms = load_symptoms(conn, a_id);
    let b_symptoms = load_symptoms(conn, b_id);

    let a_set: std::collections::HashSet<&str> = a_symptoms.iter().map(|(n, _, _)| n.as_str()).collect();
    let b_set: std::collections::HashSet<&str> = b_symptoms.iter().map(|(n, _, _)| n.as_str()).collect();

    let shared: Vec<&str> = a_set.intersection(&b_set).copied().collect();
    let only_a: Vec<&str> = a_set.difference(&b_set).copied().collect();
    let only_b: Vec<&str> = b_set.difference(&a_set).copied().collect();

    if json {
        let obj = serde_json::json!({
            "disease_a": { "name": a_name, "severity": a_sev, "description": a_desc },
            "disease_b": { "name": b_name, "severity": b_sev, "description": b_desc },
            "shared_symptoms": shared,
            "only_a": only_a,
            "only_b": only_b,
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return;
    }

    if !json {
        display::print_banner();
    }

    println!("{}", "━━━ Differential Diagnosis ━━━".bold());
    println!();
    let sev_a = SeverityLevel::from_str(&a_sev);
    let sev_b = SeverityLevel::from_str(&b_sev);
    println!("  {} {} {}", sev_a.emoji(), a_name.bold(), format!("({})", a_sev).dimmed());
    println!("    {}", a_desc.dimmed());
    println!("  vs.");
    println!("  {} {} {}", sev_b.emoji(), b_name.bold(), format!("({})", b_sev).dimmed());
    println!("    {}", b_desc.dimmed());
    println!();

    if !shared.is_empty() {
        println!("  {} ({})", "Shared Symptoms:".underline(), shared.len());
        for sym in &shared {
            println!("    🔗 {}", sym);
        }
        println!();
    }

    if !only_a.is_empty() {
        println!("  {} ({})", format!("Only in {}:", a_name).underline(), only_a.len());
        for sym in &only_a {
            println!("    🅰️  {}", sym.green());
        }
        println!();
    }

    if !only_b.is_empty() {
        println!("  {} ({})", format!("Only in {}:", b_name).underline(), only_b.len());
        for sym in &only_b {
            println!("    🅱️  {}", sym.blue());
        }
        println!();
    }

    println!("  💡 Distinguishing symptoms help tell these conditions apart.");
    if !only_a.is_empty() || !only_b.is_empty() {
        println!("     Ask about: {}", 
            only_a.iter().chain(only_b.iter())
                .take(5)
                .copied()
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
    println!();
}

fn load_disease(conn: &Connection, name: &str) -> Option<(i64, String, String, String)> {
    conn.query_row(
        "SELECT id, name, description, severity FROM diseases WHERE name LIKE ?1",
        [format!("%{name}%")],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    ).ok()
}

fn load_symptoms(conn: &Connection, disease_id: i64) -> Vec<(String, f64, bool)> {
    let mut stmt = conn
        .prepare("SELECT s.name, ds.weight, ds.is_primary FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.disease_id = ?1")
        .unwrap();
    stmt.query_map([disease_id], |row| Ok((row.get(0)?, row.get(1)?, row.get::<_, i32>(2)? != 0)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_diff_malaria_dengue() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "malaria", "dengue", false);
    }

    #[test]
    fn test_diff_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "malaria", "dengue", true);
    }

    #[test]
    fn test_diff_not_found() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, "xyznothing", "malaria", false);
    }
}
