use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ValidationIssue {
    severity: String,
    entity: String,
    message: String,
}

/// Validate database integrity: check for orphaned records, missing data, and inconsistencies.
pub fn run(conn: &Connection, json: bool) {
    let mut issues: Vec<ValidationIssue> = Vec::new();

    // 1. Diseases without symptoms
    let mut stmt = conn
        .prepare(
            "SELECT d.name FROM diseases d LEFT JOIN disease_symptoms ds ON d.id = ds.disease_id WHERE ds.disease_id IS NULL",
        )
        .unwrap();
    let orphan_diseases: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for name in &orphan_diseases {
        issues.push(ValidationIssue {
            severity: "error".to_string(),
            entity: name.clone(),
            message: "Disease has no symptoms linked".to_string(),
        });
    }

    // 2. Diseases without treatments
    let mut stmt = conn
        .prepare(
            "SELECT d.name FROM diseases d LEFT JOIN treatments t ON d.id = t.disease_id WHERE t.disease_id IS NULL",
        )
        .unwrap();
    let no_treatment: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for name in &no_treatment {
        issues.push(ValidationIssue {
            severity: "warning".to_string(),
            entity: name.clone(),
            message: "Disease has no treatment protocol".to_string(),
        });
    }

    // 3. Orphaned symptoms (not linked to any disease)
    let mut stmt = conn
        .prepare(
            "SELECT s.name FROM symptoms s LEFT JOIN disease_symptoms ds ON s.id = ds.symptom_id WHERE ds.symptom_id IS NULL",
        )
        .unwrap();
    let orphan_symptoms: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for name in &orphan_symptoms {
        issues.push(ValidationIssue {
            severity: "info".to_string(),
            entity: name.clone(),
            message: "Symptom not linked to any disease".to_string(),
        });
    }

    // 4. Diseases with fewer than 3 symptoms (potentially under-specified)
    let mut stmt = conn
        .prepare(
            "SELECT d.name, COUNT(ds.symptom_id) as cnt FROM diseases d JOIN disease_symptoms ds ON d.id = ds.disease_id GROUP BY d.id HAVING cnt < 3",
        )
        .unwrap();
    let few_symptoms: Vec<(String, i64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for (name, count) in &few_symptoms {
        issues.push(ValidationIssue {
            severity: "warning".to_string(),
            entity: name.clone(),
            message: format!("Disease has only {} symptoms (recommend ≥3)", count),
        });
    }

    // 5. Diseases without any primary symptom
    let mut stmt = conn
        .prepare(
            "SELECT d.name FROM diseases d WHERE d.id NOT IN (SELECT DISTINCT ds.disease_id FROM disease_symptoms ds WHERE ds.is_primary = 1)",
        )
        .unwrap();
    let no_primary: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for name in &no_primary {
        issues.push(ValidationIssue {
            severity: "warning".to_string(),
            entity: name.clone(),
            message: "Disease has no primary symptoms marked".to_string(),
        });
    }

    // 6. Symptom weight out of range [0.0, 1.0]
    let mut stmt = conn
        .prepare(
            "SELECT d.name, s.name, ds.weight FROM disease_symptoms ds JOIN diseases d ON d.id = ds.disease_id JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.weight < 0.0 OR ds.weight > 1.0",
        )
        .unwrap();
    let bad_weights: Vec<(String, String, f64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for (disease, symptom, weight) in &bad_weights {
        issues.push(ValidationIssue {
            severity: "error".to_string(),
            entity: format!("{disease}/{symptom}"),
            message: format!("Symptom weight {weight} is outside valid range [0.0, 1.0]"),
        });
    }

    // 7. Duplicate disease-symptom links (should be caught by PK, but check)
    let dup_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM (SELECT disease_id, symptom_id, COUNT(*) as cnt FROM disease_symptoms GROUP BY disease_id, symptom_id HAVING cnt > 1)",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if dup_count > 0 {
        issues.push(ValidationIssue {
            severity: "error".to_string(),
            entity: "disease_symptoms".to_string(),
            message: format!("{dup_count} duplicate disease-symptom links found"),
        });
    }

    // Summary stats
    let disease_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0))
        .unwrap_or(0);
    let symptom_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM symptoms", [], |r| r.get(0))
        .unwrap_or(0);
    let treatment_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM treatments", [], |r| r.get(0))
        .unwrap_or(0);

    if json {
        let obj = serde_json::json!({
            "valid": issues.iter().all(|i| i.severity != "error"),
            "diseases": disease_count,
            "symptoms": symptom_count,
            "treatments": treatment_count,
            "issues": issues,
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return;
    }

    println!("{}", "━━━ Database Validation ━━━".bold());
    println!();
    println!(
        "  Diseases:   {}  Symptoms: {}  Treatments: {}",
        disease_count, symptom_count, treatment_count
    );
    println!();

    let errors = issues.iter().filter(|i| i.severity == "error").count();
    let warnings = issues.iter().filter(|i| i.severity == "warning").count();
    let infos = issues.iter().filter(|i| i.severity == "info").count();

    if issues.is_empty() {
        println!("  {} All checks passed!", "✅".green());
    } else {
        for issue in &issues {
            let icon = match issue.severity.as_str() {
                "error" => "❌",
                "warning" => "⚠️ ",
                _ => "ℹ️ ",
            };
            let msg = match issue.severity.as_str() {
                "error" => format!("{icon} {} — {}", issue.entity, issue.message).red().to_string(),
                "warning" => format!("{icon} {} — {}", issue.entity, issue.message).yellow().to_string(),
                _ => format!("{icon} {} — {}", issue.entity, issue.message).dimmed().to_string(),
            };
            println!("  {msg}");
        }
        println!();
        println!(
            "  Summary: {} errors, {} warnings, {} info",
            errors, warnings, infos
        );
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_validate_runs() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, false);
    }

    #[test]
    fn test_validate_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, true);
    }

    #[test]
    fn test_validate_no_errors() {
        let conn = db::init_memory_database().unwrap();
        // The seeded database should have no errors
        let orphan_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM diseases d LEFT JOIN disease_symptoms ds ON d.id = ds.disease_id WHERE ds.disease_id IS NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(orphan_count, 0, "No diseases should be without symptoms");
    }
}
