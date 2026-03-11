use colored::*;
use rusqlite::Connection;

pub fn run(conn: &Connection) {
    println!(
        "{}",
        "━━━ OpenHealth Database Statistics ━━━".bold()
    );
    println!();

    let disease_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0))
        .unwrap_or(0);
    let symptom_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM symptoms", [], |r| r.get(0))
        .unwrap_or(0);
    let treatment_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM treatments", [], |r| r.get(0))
        .unwrap_or(0);
    let risk_factor_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM risk_factors", [], |r| r.get(0))
        .unwrap_or(0);
    let mapping_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM disease_symptoms", [], |r| r.get(0))
        .unwrap_or(0);
    let version: String = conn
        .query_row(
            "SELECT value FROM metadata WHERE key = 'seed_version'",
            [],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "unknown".to_string());

    println!("  Database version:       {}", version.bright_cyan());
    println!("  Diseases:               {disease_count}");
    println!("  Unique symptoms:        {symptom_count}");
    println!("  Treatments:             {treatment_count}");
    println!("  Risk factors:           {risk_factor_count}");
    println!("  Symptom-disease links:  {mapping_count}");
    println!();

    // Severity breakdown
    let mut stmt = conn
        .prepare("SELECT severity, COUNT(*) FROM diseases GROUP BY severity ORDER BY severity")
        .unwrap();
    let severities: Vec<(String, i64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    println!("  {}", "Severity breakdown:".underline());
    for (sev, count) in &severities {
        let emoji = match sev.as_str() {
            "high" => "🔴",
            "medium" => "🟡",
            _ => "🟢",
        };
        println!("    {emoji} {sev}: {count}");
    }
    println!();

    // Category breakdown
    let mut stmt = conn
        .prepare("SELECT category, COUNT(*) FROM diseases GROUP BY category ORDER BY COUNT(*) DESC")
        .unwrap();
    let categories: Vec<(String, i64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    println!("  {}", "Categories:".underline());
    for (cat, count) in &categories {
        println!("    {cat}: {count}");
    }
    println!();

    // Contagious stats
    let contagious: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM diseases WHERE contagious = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    println!(
        "  Contagious diseases:    {contagious}/{disease_count} ({:.0}%)",
        (contagious as f64 / disease_count.max(1) as f64) * 100.0
    );
    println!();
}
