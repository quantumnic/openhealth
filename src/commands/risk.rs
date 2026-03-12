use crate::engine::severity::SeverityLevel;
use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct RiskResult {
    disease: String,
    severity: String,
    matched_factors: Vec<MatchedFactor>,
    risk_score: f64,
}

#[derive(Serialize)]
struct MatchedFactor {
    factor: String,
    impact: String,
}

/// Assess disease risk based on user-provided risk factors.
pub fn run(conn: &Connection, input: &str, json: bool) {
    let factors: Vec<String> = input
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if factors.is_empty() {
        if json {
            println!("[]");
        } else {
            println!("Please provide at least one risk factor (comma-separated).");
            println!("Example: openhealth risk \"smoking, obesity, age > 50\"");
        }
        return;
    }

    let mut stmt = conn
        .prepare("SELECT d.id, d.name, d.severity FROM diseases d ORDER BY d.name")
        .unwrap();
    let diseases: Vec<(i64, String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let mut results: Vec<RiskResult> = Vec::new();

    for (did, dname, dsev) in &diseases {
        let mut rf_stmt = conn
            .prepare("SELECT factor, impact FROM risk_factors WHERE disease_id = ?1")
            .unwrap();
        let risk_factors: Vec<(String, String)> = rf_stmt
            .query_map([did], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        if risk_factors.is_empty() {
            continue;
        }

        let mut matched: Vec<MatchedFactor> = Vec::new();
        let mut score = 0.0;

        for (factor, impact) in &risk_factors {
            let factor_lower = factor.to_lowercase();
            let is_match = factors.iter().any(|f| {
                factor_lower.contains(f.as_str())
                    || f.contains(factor_lower.as_str())
                    || factor_lower
                        .split_whitespace()
                        .any(|w| w.len() >= 4 && f.contains(w))
                    || f.split_whitespace()
                        .any(|w| w.len() >= 4 && factor_lower.contains(w))
            });

            if is_match {
                let impact_score = match impact.as_str() {
                    "high" => 3.0,
                    "moderate" => 2.0,
                    _ => 1.0,
                };
                score += impact_score;
                matched.push(MatchedFactor {
                    factor: factor.clone(),
                    impact: impact.clone(),
                });
            }
        }

        if !matched.is_empty() {
            // Normalize: factor count and total possible
            let total_factors = risk_factors.len() as f64;
            let risk_score =
                ((score / (total_factors * 3.0)) * 100.0).clamp(5.0, 95.0);
            results.push(RiskResult {
                disease: dname.clone(),
                severity: dsev.clone(),
                matched_factors: matched,
                risk_score,
            });
        }
    }

    results.sort_by(|a, b| b.risk_score.partial_cmp(&a.risk_score).unwrap());

    if json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return;
    }

    if results.is_empty() {
        println!(
            "{}",
            "No diseases matched the given risk factors.".yellow()
        );
        println!("Try broader terms like: smoking, obesity, diabetes, immunosuppression");
        return;
    }

    println!("{}", "━━━ Risk Assessment ━━━".bold());
    println!(
        "  Factors: {}",
        factors.join(", ").bright_cyan()
    );
    println!();

    for (i, r) in results.iter().take(15).enumerate() {
        let sev = SeverityLevel::from_str(&r.severity);
        println!(
            "{}. {} {} — risk score: {}",
            i + 1,
            sev.emoji(),
            r.disease.bold(),
            format!("{:.0}%", r.risk_score).bright_white()
        );
        for mf in &r.matched_factors {
            let impact_color = match mf.impact.as_str() {
                "high" => format!("[{}]", mf.impact).red().to_string(),
                "moderate" => format!("[{}]", mf.impact).yellow().to_string(),
                _ => format!("[{}]", mf.impact).green().to_string(),
            };
            println!("      ⚡ {} {}", mf.factor, impact_color);
        }
        println!();
    }

    if results.len() > 15 {
        println!(
            "  ... and {} more (use --json for full output)",
            results.len() - 15
        );
    }

    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━".dimmed());
    println!(
        "{}",
        "⚠️  Risk assessment is informational only. Consult a healthcare provider."
            .yellow()
    );
    println!();
}
