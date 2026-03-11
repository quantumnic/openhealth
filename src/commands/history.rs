use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HistoryEntry {
    id: i64,
    timestamp: String,
    symptoms: String,
    top_disease: Option<String>,
    top_probability: Option<f64>,
}

pub fn save(
    conn: &Connection,
    symptoms: &str,
    top_disease: Option<&str>,
    top_probability: Option<f64>,
    result_json: &str,
) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO history (timestamp, symptoms, top_disease, top_probability, result_json) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![timestamp, symptoms, top_disease, top_probability, result_json],
    )
    .ok();
}

pub fn run(conn: &Connection, limit: usize, json: bool) {
    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, symptoms, top_disease, top_probability FROM history ORDER BY id DESC LIMIT ?1",
        )
        .unwrap();

    let entries: Vec<HistoryEntry> = stmt
        .query_map([limit as i64], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                symptoms: row.get(2)?,
                top_disease: row.get(3)?,
                top_probability: row.get(4)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&entries).unwrap());
        return;
    }

    if entries.is_empty() {
        println!("{}", "📋 No diagnosis history yet. Run `symptoms` or `check` to build history.".yellow());
        return;
    }

    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "║              📋  Diagnosis History                      ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝"
            .cyan()
            .bold()
    );
    println!();

    for entry in &entries {
        let disease_str = entry
            .top_disease
            .as_deref()
            .unwrap_or("No match");
        let prob_str = entry
            .top_probability
            .map(|p| format!("{:.1}%", p))
            .unwrap_or_else(|| "—".to_string());

        println!(
            "  {} │ {} │ {} ({}) │ {}",
            format!("#{}", entry.id).dimmed(),
            entry.timestamp.dimmed(),
            disease_str.bold(),
            prob_str.yellow(),
            entry.symptoms.dimmed()
        );
    }

    println!();
    println!(
        "  {} entries shown (use --limit N to change)",
        entries.len()
    );
}

pub fn show_detail(conn: &Connection, entry_id: i64, json: bool) {
    let result = conn.query_row(
        "SELECT timestamp, symptoms, result_json FROM history WHERE id = ?1",
        [entry_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        },
    );

    match result {
        Ok((timestamp, symptoms, result_json)) => {
            if json {
                println!("{}", result_json);
            } else {
                println!("{}", format!("📋 History Entry #{entry_id}").cyan().bold());
                println!("  {} {}", "Date:".bold(), timestamp);
                println!("  {} {}", "Symptoms:".bold(), symptoms);
                println!();
                // Parse and display stored results
                if let Ok(results) = serde_json::from_str::<Vec<serde_json::Value>>(&result_json) {
                    for r in results.iter().take(5) {
                        let name = r["disease_name"].as_str().unwrap_or("?");
                        let prob = r["probability"].as_f64().unwrap_or(0.0);
                        println!("  → {} ({:.1}%)", name.bold(), prob);
                    }
                } else {
                    println!("  {}", result_json);
                }
            }
        }
        Err(_) => {
            println!("{}", format!("History entry #{entry_id} not found.").red());
        }
    }
}
