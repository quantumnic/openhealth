use colored::*;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Serialize)]
struct CompareResult {
    diseases: Vec<String>,
    shared_symptoms: Vec<String>,
    unique_symptoms: Vec<Vec<String>>,
    shared_risk_factors: Vec<String>,
    severity_comparison: Vec<String>,
}

pub fn run(conn: &Connection, names: &[&str], json: bool) {
    if names.len() < 2 {
        println!("{}", "Please provide at least 2 disease names to compare.".yellow());
        return;
    }

    if names.len() > 5 {
        println!("{}", "Maximum 5 diseases can be compared at once.".yellow());
        return;
    }

    let mut all_disease_data: Vec<(String, String, BTreeSet<String>, Vec<String>)> = Vec::new();

    for name in names {
        let disease = conn.query_row(
            "SELECT id, name, severity FROM diseases WHERE name = ?1 COLLATE NOCASE",
            [name],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)),
        );

        let (disease_id, disease_name, severity) = match disease {
            Ok(d) => d,
            Err(_) => {
                if !json {
                    println!("⚠️ Disease '{}' not found.", name.bold());
                }
                return;
            }
        };

        let mut stmt = conn
            .prepare("SELECT s.name FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.disease_id = ?1")
            .unwrap();
        let symptoms: BTreeSet<String> = stmt
            .query_map([disease_id], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        let mut rf_stmt = conn
            .prepare("SELECT factor FROM risk_factors WHERE disease_id = ?1")
            .unwrap();
        let risk_factors: Vec<String> = rf_stmt
            .query_map([disease_id], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        all_disease_data.push((disease_name, severity, symptoms, risk_factors));
    }

    // Compute shared symptoms (intersection of all)
    let shared_symptoms: BTreeSet<String> = all_disease_data
        .iter()
        .map(|(_, _, syms, _)| syms.clone())
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap_or_default();

    // Compute unique symptoms per disease
    let unique_symptoms: Vec<Vec<String>> = all_disease_data
        .iter()
        .map(|(_, _, syms, _)| {
            syms.difference(&shared_symptoms)
                .cloned()
                .collect()
        })
        .collect();

    // Shared risk factors
    let all_rf_sets: Vec<BTreeSet<String>> = all_disease_data
        .iter()
        .map(|(_, _, _, rfs)| rfs.iter().cloned().collect())
        .collect();
    let shared_risk_factors: BTreeSet<String> = all_rf_sets
        .iter()
        .cloned()
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap_or_default();

    let disease_names: Vec<String> = all_disease_data.iter().map(|(n, _, _, _)| n.clone()).collect();
    let severities: Vec<String> = all_disease_data.iter().map(|(_, s, _, _)| s.clone()).collect();

    if json {
        let result = CompareResult {
            diseases: disease_names.clone(),
            shared_symptoms: shared_symptoms.iter().cloned().collect(),
            unique_symptoms,
            shared_risk_factors: shared_risk_factors.iter().cloned().collect(),
            severity_comparison: severities.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return;
    }

    // Pretty print
    println!("{}", "━━━ Disease Comparison ━━━".bold());
    println!();

    // Header
    print!("   {:30}", "");
    for name in &disease_names {
        print!("{:25}", name.bold());
    }
    println!();

    // Severity row
    print!("   {:30}", "Severity".dimmed());
    for sev in &severities {
        let (emoji, label) = match sev.as_str() {
            "high" => ("🔴", "High"),
            "medium" => ("🟡", "Medium"),
            _ => ("🟢", "Low"),
        };
        print!("{:25}", format!("{emoji} {label}"));
    }
    println!();

    // Symptom count row
    print!("   {:30}", "Symptoms".dimmed());
    for (_, _, syms, _) in &all_disease_data {
        print!("{:25}", syms.len().to_string());
    }
    println!();
    println!();

    // Shared symptoms
    if shared_symptoms.is_empty() {
        println!("   🔗 {}", "No shared symptoms".yellow());
    } else {
        println!(
            "   🔗 {} ({})",
            "Shared Symptoms".green().bold(),
            shared_symptoms.len()
        );
        for sym in &shared_symptoms {
            println!("      • {sym}");
        }
    }
    println!();

    // Unique symptoms per disease
    for (i, (name, _, _, _)) in all_disease_data.iter().enumerate() {
        let unique = &unique_symptoms[i];
        if unique.is_empty() {
            println!("   🔹 {} — no unique symptoms", name.bold());
        } else {
            println!(
                "   🔹 {} — {} unique symptoms",
                name.bold(),
                unique.len()
            );
            for sym in unique {
                println!("      • {sym}");
            }
        }
    }
    println!();

    // Shared risk factors
    if !shared_risk_factors.is_empty() {
        println!(
            "   ⚡ {} ({})",
            "Shared Risk Factors".yellow().bold(),
            shared_risk_factors.len()
        );
        for rf in &shared_risk_factors {
            println!("      • {rf}");
        }
        println!();
    }

    println!();
}
