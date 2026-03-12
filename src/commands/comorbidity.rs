use colored::*;
use rusqlite::Connection;
use serde::Serialize;


#[derive(Serialize)]
struct ComorbidityResult {
    disease: String,
    related: Vec<RelatedDisease>,
}

#[derive(Serialize)]
struct RelatedDisease {
    name: String,
    shared_risk_factors: Vec<String>,
    shared_symptoms: Vec<String>,
    score: f64,
}

pub fn run(conn: &Connection, disease_name: &str, limit: usize, json: bool) {
    // Find the target disease
    let disease_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM diseases WHERE name = ?1 COLLATE NOCASE",
            [disease_name],
            |r| r.get(0),
        )
        .ok();

    let disease_id = match disease_id {
        Some(id) => id,
        None => {
            // Try fuzzy match
            let mut stmt = conn
                .prepare("SELECT id, name FROM diseases")
                .unwrap();
            let matches: Vec<(i64, String)> = stmt
                .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))
                .unwrap()
                .filter_map(|r| r.ok())
                .filter(|(_, name)| name.to_lowercase().contains(&disease_name.to_lowercase()))
                .collect();

            if matches.is_empty() {
                if json {
                    println!("{{\"error\": \"Disease not found: {disease_name}\"}}");
                } else {
                    println!("Disease not found: {disease_name}");
                    println!("Try: openhealth search {disease_name}");
                }
                return;
            }
            if !json {
                println!("Using: {}", matches[0].1);
            }
            matches[0].0
        }
    };

    let actual_name: String = conn
        .query_row("SELECT name FROM diseases WHERE id = ?1", [disease_id], |r| r.get(0))
        .unwrap();

    // Get risk factors for target disease
    let mut rf_stmt = conn
        .prepare("SELECT factor FROM risk_factors WHERE disease_id = ?1")
        .unwrap();
    let target_factors: Vec<String> = rf_stmt
        .query_map([disease_id], |r| r.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // Get symptoms for target disease
    let mut sym_stmt = conn
        .prepare(
            "SELECT s.name FROM symptoms s
             JOIN disease_symptoms ds ON s.id = ds.symptom_id
             WHERE ds.disease_id = ?1",
        )
        .unwrap();
    let target_symptoms: Vec<String> = sym_stmt
        .query_map([disease_id], |r| r.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // Find all other diseases and their shared risk factors + symptoms
    let all_diseases: Vec<(i64, String)> = conn
        .prepare("SELECT id, name FROM diseases WHERE id != ?1")
        .unwrap()
        .query_map([disease_id], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let mut scored: Vec<RelatedDisease> = Vec::new();

    for (did, dname) in &all_diseases {
        let other_factors: Vec<String> = conn
            .prepare("SELECT factor FROM risk_factors WHERE disease_id = ?1")
            .unwrap()
            .query_map([did], |r| r.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        let other_symptoms: Vec<String> = conn
            .prepare(
                "SELECT s.name FROM symptoms s
                 JOIN disease_symptoms ds ON s.id = ds.symptom_id
                 WHERE ds.disease_id = ?1",
            )
            .unwrap()
            .query_map([did], |r| r.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        let shared_rf: Vec<String> = target_factors
            .iter()
            .filter(|f| {
                other_factors
                    .iter()
                    .any(|of| of.to_lowercase() == f.to_lowercase())
            })
            .cloned()
            .collect();

        let shared_sym: Vec<String> = target_symptoms
            .iter()
            .filter(|s| {
                other_symptoms
                    .iter()
                    .any(|os| os.to_lowercase() == s.to_lowercase())
            })
            .cloned()
            .collect();

        if shared_rf.is_empty() && shared_sym.is_empty() {
            continue;
        }

        // Score: risk factors weighted more (2x) than symptoms
        let score = (shared_rf.len() as f64 * 2.0) + (shared_sym.len() as f64);

        scored.push(RelatedDisease {
            name: dname.clone(),
            shared_risk_factors: shared_rf,
            shared_symptoms: shared_sym,
            score,
        });
    }

    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    scored.truncate(limit);

    if json {
        let result = ComorbidityResult {
            disease: actual_name,
            related: scored,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
        );
        return;
    }

    println!();
    println!(
        "{}",
        format!("🔗 Comorbidity Analysis: {actual_name}").bright_cyan().bold()
    );
    println!(
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    );
    println!();

    if scored.is_empty() {
        println!("No closely related diseases found.");
        return;
    }

    for (i, rd) in scored.iter().enumerate() {
        println!(
            "{}. {} (relevance: {:.0})",
            i + 1,
            rd.name.bright_white().bold(),
            rd.score
        );
        if !rd.shared_risk_factors.is_empty() {
            println!(
                "   Risk factors: {}",
                rd.shared_risk_factors.join(", ").yellow()
            );
        }
        if !rd.shared_symptoms.is_empty() {
            println!(
                "   Shared symptoms: {}",
                rd.shared_symptoms.join(", ").bright_blue()
            );
        }
        println!();
    }

    println!(
        "{}",
        "ℹ️  Comorbidity = diseases that may co-occur or share underlying risk.".dimmed()
    );
    println!(
        "{}",
        "   This is statistical association, not causation.".dimmed()
    );
    println!();
}
