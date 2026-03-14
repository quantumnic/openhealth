use colored::Colorize;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct SymptomMapEntry {
    symptom: String,
    disease_count: usize,
    diseases: Vec<String>,
}

/// Show how many diseases each symptom maps to — helps understand symptom specificity.
pub fn run(conn: &Connection, filter: Option<&str>, json: bool) {
    let mut stmt = conn
        .prepare(
            "SELECT s.name, d.name
             FROM symptoms s
             JOIN disease_symptoms ds ON ds.symptom_id = s.id
             JOIN diseases d ON d.id = ds.disease_id
             ORDER BY s.name",
        )
        .unwrap();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
            ))
        })
        .unwrap();

    for row in rows {
        let (symptom, disease) = row.unwrap();
        map.entry(symptom).or_default().push(disease);
    }

    let mut entries: Vec<SymptomMapEntry> = map
        .into_iter()
        .map(|(symptom, diseases)| SymptomMapEntry {
            disease_count: diseases.len(),
            symptom,
            diseases,
        })
        .collect();

    // Apply filter if provided
    if let Some(f) = filter {
        let f_lower = f.to_lowercase();
        entries.retain(|e| e.symptom.to_lowercase().contains(&f_lower));
    }

    // Sort by disease count descending (most common symptoms first)
    entries.sort_by(|a, b| b.disease_count.cmp(&a.disease_count));

    if json {
        println!("{}", serde_json::to_string_pretty(&entries).unwrap());
        return;
    }

    if entries.is_empty() {
        println!("{}", "No symptoms found matching your filter.".yellow());
        return;
    }

    println!(
        "\n{}",
        "═══ Symptom Specificity Map ═══".bold().cyan()
    );
    println!(
        "{}",
        "Shows how many diseases share each symptom (higher = less specific)\n"
            .dimmed()
    );

    for entry in &entries {
        let specificity = if entry.disease_count <= 3 {
            "HIGHLY SPECIFIC".green()
        } else if entry.disease_count <= 10 {
            "MODERATE".yellow()
        } else {
            "COMMON".red()
        };

        println!(
            "  {} {} [{}]",
            format!("{:>3}", entry.disease_count).bold(),
            entry.symptom,
            specificity,
        );
    }

    println!(
        "\n{} {} symptoms tracked across the database.",
        "Total:".bold(),
        entries.len()
    );
    println!(
        "{}",
        "Tip: Highly specific symptoms narrow diagnoses faster.\n"
            .dimmed()
    );
}
