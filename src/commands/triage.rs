use crate::engine::scorer::{self, PatientContext};
use crate::engine::severity::SeverityLevel;
use crate::commands::profile;
use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct TriageResult {
    level: String,
    label: String,
    symptoms_analyzed: Vec<String>,
    top_conditions: Vec<TriageCondition>,
    red_flags: Vec<String>,
    action: String,
}

#[derive(Serialize)]
struct TriageCondition {
    name: String,
    probability: f64,
    severity: String,
}

/// Red flag symptoms that always warrant urgent evaluation.
const RED_FLAG_SYMPTOMS: &[&str] = &[
    "chest pain",
    "difficulty breathing",
    "shortness of breath",
    "sudden severe headache",
    "loss of consciousness",
    "seizures",
    "coughing up blood",
    "blood in stool",
    "severe abdominal pain",
    "confusion",
    "sudden vision loss",
    "facial drooping",
    "inability to speak",
    "severe bleeding",
    "suicidal thoughts",
    "high fever",
    "stiff neck",
    "severe allergic reaction",
    "anaphylaxis",
    "hydrophobia",
];

pub fn run(conn: &Connection, input: &str, json: bool) {
    let symptom_list: Vec<&str> = input
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if symptom_list.is_empty() {
        if json {
            println!("{{\"error\": \"No symptoms provided\"}}");
        } else {
            println!("Please provide symptoms for triage assessment.");
        }
        return;
    }

    let context = PatientContext {
        age: profile::get_profile_age(conn),
        sex: profile::get_profile_sex(conn),
    };

    let results = scorer::score_symptoms_with_context(conn, &symptom_list, &context);

    // Detect red flags
    let input_lower: Vec<String> = symptom_list.iter().map(|s| s.to_lowercase()).collect();
    let input_joined = input_lower.join(" ");
    let red_flags: Vec<String> = RED_FLAG_SYMPTOMS
        .iter()
        .filter(|rf| input_joined.contains(*rf) || input_lower.iter().any(|s| s.contains(*rf)))
        .map(|rf| rf.to_string())
        .collect();

    // Determine triage level
    let has_red_flags = !red_flags.is_empty();
    let top_severity = results
        .iter()
        .take(3)
        .map(|r| r.severity.as_str())
        .collect::<Vec<_>>();
    let overall = crate::engine::severity::overall_severity(&top_severity);

    let triage_level = if has_red_flags {
        SeverityLevel::Red
    } else {
        overall
    };

    let action = match triage_level {
        SeverityLevel::Red => "🚨 SEEK EMERGENCY CARE IMMEDIATELY. Do not delay. Call emergency services or go to nearest hospital.",
        SeverityLevel::Yellow => "📋 Schedule a medical appointment within 24-48 hours. Monitor symptoms. Go to emergency if symptoms worsen rapidly.",
        SeverityLevel::Green => "🏠 Self-care at home. Rest, hydrate, and monitor. See a doctor if symptoms persist beyond 3-5 days or worsen.",
    };

    let top_conditions: Vec<TriageCondition> = results
        .iter()
        .take(3)
        .map(|r| TriageCondition {
            name: r.disease_name.clone(),
            probability: r.probability,
            severity: r.severity.clone(),
        })
        .collect();

    if json {
        let result = TriageResult {
            level: triage_level.label().to_string(),
            label: triage_level.emoji().to_string(),
            symptoms_analyzed: symptom_list.iter().map(|s| s.to_string()).collect(),
            top_conditions,
            red_flags,
            action: action.to_string(),
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
        );
        return;
    }

    // Display triage card
    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║              🚑  TRIAGE ASSESSMENT                      ║".bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!();

    println!("Symptoms: {}", symptom_list.join(", ").bright_white());
    println!();

    // Triage level
    let level_display = match triage_level {
        SeverityLevel::Red => format!("{} {}", triage_level.emoji(), triage_level.label()).red().bold().to_string(),
        SeverityLevel::Yellow => format!("{} {}", triage_level.emoji(), triage_level.label()).yellow().bold().to_string(),
        SeverityLevel::Green => format!("{} {}", triage_level.emoji(), triage_level.label()).green().bold().to_string(),
    };
    println!("Triage Level: {level_display}");
    println!();

    // Red flags
    if !red_flags.is_empty() {
        println!("{}", "⚠️  RED FLAGS DETECTED:".red().bold());
        for flag in &red_flags {
            println!("  🔴 {}", flag.red());
        }
        println!();
    }

    // Top conditions
    if !results.is_empty() {
        println!("{}", "Most likely conditions:".bold());
        for (i, r) in results.iter().take(3).enumerate() {
            let sev = SeverityLevel::from_str(&r.severity);
            println!(
                "  {}. {} {} ({:.0}% match)",
                i + 1,
                sev.emoji(),
                r.disease_name,
                r.probability
            );
        }
        println!();
    }

    // Action
    println!("{}", "━━━ Recommended Action ━━━".bold());
    println!("{action}");
    println!();
    println!(
        "{}",
        "⚠️  This is automated triage guidance, NOT a medical diagnosis.".yellow()
    );
    println!(
        "{}",
        "   When in doubt, always seek professional medical help.".yellow()
    );
    println!();
}
