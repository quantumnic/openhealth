use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct PrevalenceEntry {
    disease: String,
    severity: String,
    category: String,
    symptom_count: usize,
    risk_factor_count: usize,
}

pub fn run(conn: &Connection, category_filter: Option<&str>, json: bool) {
    let mut stmt = conn
        .prepare(
            "SELECT d.id, d.name, d.severity, d.category,
                    (SELECT COUNT(*) FROM disease_symptoms WHERE disease_id = d.id) as sym_count,
                    (SELECT COUNT(*) FROM risk_factors WHERE disease_id = d.id) as rf_count
             FROM diseases d
             ORDER BY d.category, d.name",
        )
        .unwrap();

    let entries: Vec<PrevalenceEntry> = stmt
        .query_map([], |row| {
            Ok(PrevalenceEntry {
                disease: row.get(1)?,
                severity: row.get(2)?,
                category: row.get::<_, Option<String>>(3)?.unwrap_or_else(|| "general".into()),
                symptom_count: row.get::<_, usize>(4)?,
                risk_factor_count: row.get::<_, usize>(5)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .filter(|e| {
            if let Some(cat) = category_filter {
                e.category.to_lowercase().contains(&cat.to_lowercase())
            } else {
                true
            }
        })
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&entries).unwrap());
        return;
    }

    if entries.is_empty() {
        println!("{}", "No diseases found for the given filter.".yellow());
        return;
    }

    // Group by category
    let mut current_cat = String::new();
    let mut cat_count = 0;

    println!("{}", "━━━ Disease Prevalence Overview ━━━".bold());
    println!();

    for entry in &entries {
        if entry.category != current_cat {
            if !current_cat.is_empty() {
                println!("   {} diseases in category", cat_count.to_string().bold());
                println!();
            }
            current_cat = entry.category.clone();
            cat_count = 0;
            println!(
                "📂 {}",
                current_cat.to_uppercase().bright_cyan().bold()
            );
        }
        cat_count += 1;

        let sev_indicator = match entry.severity.as_str() {
            "high" => "🔴".to_string(),
            "medium" => "🟡".to_string(),
            _ => "🟢".to_string(),
        };

        let completeness = entry.symptom_count + entry.risk_factor_count;
        let completeness_bar = if completeness >= 10 {
            "████".green().to_string()
        } else if completeness >= 6 {
            "███░".yellow().to_string()
        } else {
            "██░░".red().to_string()
        };

        println!(
            "   {sev_indicator} {} — {} symptoms, {} risk factors [{completeness_bar}]",
            entry.disease.bold(),
            entry.symptom_count.to_string().bright_white(),
            entry.risk_factor_count.to_string().bright_white(),
        );
    }
    if cat_count > 0 {
        println!("   {} diseases in category", cat_count.to_string().bold());
    }

    println!();
    println!(
        "📊 Total: {} diseases across all categories",
        entries.len().to_string().bold()
    );
    println!();
}
