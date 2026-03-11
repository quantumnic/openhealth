use crate::display;
use crate::engine::severity::SeverityLevel;
use colored::*;
use rusqlite::Connection;

pub fn run(conn: &Connection, category: Option<&str>, json: bool) {
    if !json {
        display::print_banner();
    }

    let query = if let Some(cat) = category {
        format!(
            "SELECT name, severity, contagious, category FROM diseases WHERE category = '{}' ORDER BY name",
            cat.replace('\'', "''")
        )
    } else {
        "SELECT name, severity, contagious, category FROM diseases ORDER BY category, name".to_string()
    };

    let mut stmt = conn.prepare(&query).unwrap();
    let diseases: Vec<(String, String, bool, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get::<_, i32>(2)? != 0,
                row.get(3)?,
            ))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    if json {
        let entries: Vec<serde_json::Value> = diseases
            .iter()
            .map(|(name, severity, contagious, category)| {
                serde_json::json!({
                    "name": name,
                    "severity": severity,
                    "contagious": contagious,
                    "category": category,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&entries).unwrap());
        return;
    }

    if diseases.is_empty() {
        println!("{}", "No diseases found.".yellow());
        if category.is_some() {
            println!("Available categories: infectious, respiratory, cardiovascular, metabolic, neurological, etc.");
        }
        return;
    }

    println!(
        "{} ({} diseases)\n",
        "━━━ Disease Database ━━━".bold(),
        diseases.len()
    );

    let mut current_cat = String::new();
    for (name, severity, contagious, cat) in &diseases {
        if *cat != current_cat {
            if !current_cat.is_empty() {
                println!();
            }
            println!("  {}", format!("[{cat}]").bright_cyan().bold());
            current_cat.clone_from(cat);
        }
        let sev = SeverityLevel::from_str(severity);
        let tag = if *contagious { " ⚠️" } else { "" };
        println!("    {} {}{}", sev.emoji(), name, tag);
    }
    println!();
    println!("  ⚠️ contagious  {} total", diseases.len());
}
