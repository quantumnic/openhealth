use crate::display;
use crate::engine::scorer;
use rusqlite::Connection;

pub fn run(conn: &Connection, input: &str, json: bool) {
    let symptom_list: Vec<&str> = input
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if symptom_list.is_empty() {
        if json {
            println!("[]");
        } else {
            println!("Please provide at least one symptom.");
        }
        return;
    }

    let results = scorer::score_symptoms(conn, &symptom_list);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&results).unwrap_or_else(|_| "[]".to_string())
        );
        return;
    }

    display::print_banner();
    display::print_disclaimer();

    println!("Analyzing symptoms: {}", symptom_list.join(", "));
    println!();

    display::print_diagnosis_results(&results, 5);
}
