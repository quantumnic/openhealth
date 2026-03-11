use crate::display;
use crate::engine::scorer;
use rusqlite::Connection;

pub fn run(conn: &Connection, input: &str) {
    display::print_banner();
    display::print_disclaimer();

    let symptom_list: Vec<&str> = input
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if symptom_list.is_empty() {
        println!("Please provide at least one symptom.");
        return;
    }

    println!("Analyzing symptoms: {}", symptom_list.join(", "));
    println!();

    let results = scorer::score_symptoms(conn, &symptom_list);
    display::print_diagnosis_results(&results, 5);
}
