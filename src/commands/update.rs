use rusqlite::Connection;
use colored::*;

pub fn run(conn: &Connection) {
    println!("{}", "🔄 Checking database...".bold());

    let disease_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0))
        .unwrap_or(0);
    let symptom_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM symptoms", [], |r| r.get(0))
        .unwrap_or(0);
    let version: String = conn
        .query_row(
            "SELECT value FROM metadata WHERE key = 'seed_version'",
            [],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "unknown".to_string());

    println!("  Database version: {}", version.bright_cyan());
    println!("  Diseases:         {disease_count}");
    println!("  Symptoms:         {symptom_count}");
    println!();

    // Re-seed if empty
    if disease_count == 0 {
        println!("{}", "Database is empty. Seeding...".yellow());
        match crate::db::seed::seed_all(conn) {
            Ok(()) => println!("{}", "✅ Database seeded successfully!".green()),
            Err(e) => println!("{}", format!("❌ Seeding failed: {e}").red()),
        }
    } else {
        println!("{}", "✅ Database is up to date.".green());
        println!("   Future versions will support online database updates.");
    }
}
