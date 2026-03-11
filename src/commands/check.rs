use crate::display;
use crate::engine::scorer;
use rusqlite::Connection;
use std::io::{self, Write};

pub fn run(conn: &Connection) {
    display::print_banner();
    display::print_disclaimer();

    println!("🩺 Interactive Symptom Checker");
    println!("Answer the following questions to help identify possible conditions.");
    println!("Type 'done' when finished, or 'quit' to exit.\n");

    let mut all_symptoms: Vec<String> = Vec::new();

    // Step 1: Ask main complaint
    print!("What is your main complaint? > ");
    io::stdout().flush().ok();
    let mut main_complaint = String::new();
    if io::stdin().read_line(&mut main_complaint).is_err() {
        return;
    }
    let main_complaint = main_complaint.trim().to_lowercase();
    if main_complaint == "quit" { return; }
    all_symptoms.push(main_complaint);

    // Step 2: Ask about common associated symptoms
    let followup_questions = [
        ("Do you have a fever?", "fever"),
        ("Do you have a headache?", "headache"),
        ("Any nausea or vomiting?", "nausea"),
        ("Any cough?", "cough"),
        ("Any diarrhea?", "diarrhea"),
        ("Any difficulty breathing?", "difficulty breathing"),
        ("Any chest pain?", "chest pain"),
        ("Any rash?", "rash"),
    ];

    for (question, symptom) in &followup_questions {
        print!("{question} (y/n) > ");
        io::stdout().flush().ok();
        let mut answer = String::new();
        if io::stdin().read_line(&mut answer).is_err() {
            break;
        }
        let answer = answer.trim().to_lowercase();
        if answer == "quit" || answer == "done" { break; }
        if answer.starts_with('y') {
            all_symptoms.push(symptom.to_string());
        }
    }

    // Step 3: Additional symptoms
    println!("\nAny other symptoms? (comma-separated, or 'done') > ");
    io::stdout().flush().ok();
    let mut extra = String::new();
    if io::stdin().read_line(&mut extra).is_ok() {
        let extra = extra.trim();
        if extra != "done" && extra != "quit" && !extra.is_empty() {
            for s in extra.split(',') {
                let s = s.trim();
                if !s.is_empty() {
                    all_symptoms.push(s.to_lowercase());
                }
            }
        }
    }

    println!("\n📊 Analyzing your symptoms...\n");

    let refs: Vec<&str> = all_symptoms.iter().map(|s| s.as_str()).collect();
    let results = scorer::score_symptoms(conn, &refs);
    display::print_diagnosis_results(&results, 5);
}
