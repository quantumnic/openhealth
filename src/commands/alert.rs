use colored::*;
use rusqlite::Connection;
use serde::Serialize;

use crate::engine::scorer;

/// Emergency alert patterns — combinations of symptoms that indicate
/// time-critical emergencies requiring immediate action.
#[derive(Debug, Clone, Serialize)]
pub struct EmergencyAlert {
    pub condition: String,
    pub urgency: String,
    pub matched_indicators: Vec<String>,
    pub action: String,
    pub time_window: String,
}

struct AlertPattern {
    condition: &'static str,
    /// At least this many indicators must match
    min_match: usize,
    indicators: &'static [&'static str],
    action: &'static str,
    time_window: &'static str,
}

const ALERT_PATTERNS: &[AlertPattern] = &[
    AlertPattern {
        condition: "Stroke (FAST)",
        min_match: 2,
        indicators: &["face drooping", "arm weakness", "speech difficulty", "facial drooping", "slurred speech", "confusion", "vision problems", "sudden headache"],
        action: "Call emergency services IMMEDIATELY. Note time of symptom onset. Do NOT give food/water.",
        time_window: "Treatment within 3 hours critical (tPA window)",
    },
    AlertPattern {
        condition: "Heart Attack (STEMI)",
        min_match: 2,
        indicators: &["chest pain", "chest pressure", "left arm pain", "jaw pain", "cold sweat", "shortness of breath", "crushing chest pain"],
        action: "Call emergency services. Chew aspirin 300mg if available. Do NOT drive yourself.",
        time_window: "PCI within 90 minutes of symptom onset",
    },
    AlertPattern {
        condition: "Anaphylaxis",
        min_match: 2,
        indicators: &["difficulty breathing", "swelling of face", "swelling of throat", "rash", "low blood pressure", "rapid heartbeat", "hives"],
        action: "Use epinephrine auto-injector (EpiPen) if available. Call emergency. Lie down with legs elevated.",
        time_window: "Epinephrine within minutes — death can occur in 5-30 minutes",
    },
    AlertPattern {
        condition: "Sepsis",
        min_match: 3,
        indicators: &["fever", "rapid heart rate", "rapid breathing", "confusion", "low blood pressure", "decreased urination", "mottled skin"],
        action: "Emergency department immediately. Hour-1 bundle: blood cultures + IV antibiotics + IV fluids.",
        time_window: "Each hour of delay increases mortality 4-8%",
    },
    AlertPattern {
        condition: "Meningitis",
        min_match: 2,
        indicators: &["severe headache", "stiff neck", "high fever", "sensitivity to light", "rash", "confusion", "vomiting"],
        action: "Emergency department IMMEDIATELY. IV antibiotics must start before imaging.",
        time_window: "Antibiotic administration within 1 hour",
    },
    AlertPattern {
        condition: "Pulmonary Embolism",
        min_match: 2,
        indicators: &["sudden shortness of breath", "chest pain", "coughing blood", "rapid heart rate", "leg swelling", "leg pain"],
        action: "Call emergency services. Keep patient still and upright. Anticoagulation in hospital.",
        time_window: "Massive PE: thrombolysis within hours",
    },
    AlertPattern {
        condition: "Tension Pneumothorax",
        min_match: 2,
        indicators: &["sudden chest pain", "shortness of breath", "decreased breath sounds", "tracheal deviation", "rapid heart rate", "cyanosis"],
        action: "Needle decompression (2nd intercostal space, midclavicular line). Call emergency.",
        time_window: "Minutes — can be rapidly fatal",
    },
    AlertPattern {
        condition: "Diabetic Ketoacidosis",
        min_match: 2,
        indicators: &["fruity breath odor", "excessive thirst", "frequent urination", "nausea", "vomiting", "rapid breathing", "confusion", "abdominal pain"],
        action: "Emergency department: IV fluids + insulin infusion + potassium replacement.",
        time_window: "Hours — progressive organ damage without treatment",
    },
    AlertPattern {
        condition: "Aortic Dissection",
        min_match: 2,
        indicators: &["sudden severe chest pain", "tearing pain", "pain radiating to back", "unequal blood pressure", "weak pulse"],
        action: "Emergency department IMMEDIATELY. IV beta-blockers. Type A → surgery.",
        time_window: "Type A: mortality increases 1-2% per hour without surgery",
    },
    AlertPattern {
        condition: "Status Epilepticus",
        min_match: 1,
        indicators: &["prolonged seizure", "repeated seizures", "seizure lasting more than 5 minutes"],
        action: "Call emergency. IV benzodiazepines. Protect from injury. Do NOT restrain.",
        time_window: "Brain damage risk increases after 5 minutes of continuous seizure",
    },
];

pub fn run(conn: &Connection, symptoms_input: &str, json: bool) {
    let symptoms: Vec<&str> = symptoms_input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if symptoms.is_empty() {
        eprintln!("Please provide symptoms to check.");
        return;
    }

    let normalized: Vec<String> = symptoms.iter().map(|s| s.to_lowercase()).collect();

    // Check alert patterns
    let mut alerts: Vec<EmergencyAlert> = Vec::new();

    for pattern in ALERT_PATTERNS {
        let matched: Vec<String> = pattern.indicators.iter()
            .filter(|ind| {
                let ind_lower = ind.to_lowercase();
                normalized.iter().any(|input| {
                    input.contains(&ind_lower) || ind_lower.contains(input.as_str())
                })
            })
            .map(|s| s.to_string())
            .collect();

        if matched.len() >= pattern.min_match {
            alerts.push(EmergencyAlert {
                condition: pattern.condition.to_string(),
                urgency: "EMERGENCY".to_string(),
                matched_indicators: matched,
                action: pattern.action.to_string(),
                time_window: pattern.time_window.to_string(),
            });
        }
    }

    // Also run the regular scorer for context
    let results = scorer::score_symptoms(conn, &symptoms);
    let has_high_severity = results.iter().take(5).any(|r| r.severity == "high");

    if json {
        let output = serde_json::json!({
            "alerts": alerts,
            "emergency_detected": !alerts.is_empty(),
            "high_severity_matches": has_high_severity,
            "top_matches": results.iter().take(3).collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
        return;
    }

    if alerts.is_empty() && !has_high_severity {
        println!("{}", "✅ No emergency patterns detected.".green().bold());
        println!();
        println!("Your symptoms don't match known emergency patterns.");
        println!("If you're concerned, consult a healthcare provider.");
        if !results.is_empty() {
            println!();
            println!("{}", "Top matches from symptom checker:".cyan());
            for r in results.iter().take(3) {
                let sev = crate::engine::severity::SeverityLevel::from_str(&r.severity);
                println!("  {} {} ({:.0}%) — {}", sev.emoji(), r.disease_name, r.probability, sev.label());
            }
        }
        return;
    }

    if !alerts.is_empty() {
        println!("{}", "🚨 EMERGENCY ALERT 🚨".red().bold());
        println!("{}", "═".repeat(50).red());
        println!();

        for alert in &alerts {
            println!("{} {}", "⚠️ ".red(), alert.condition.red().bold());
            println!("  {}: {}", "Matched indicators".yellow(), alert.matched_indicators.join(", "));
            println!("  {}: {}", "Action".white().bold(), alert.action);
            println!("  {}: {}", "Time window".yellow(), alert.time_window);
            println!();
        }

        println!("{}", "━".repeat(50).red());
        println!("{}", "Call emergency services immediately.".red().bold());
        println!("{}", "Do not delay seeking help.".red());
    } else if has_high_severity {
        println!("{}", "⚠️  HIGH SEVERITY SYMPTOMS DETECTED".yellow().bold());
        println!();
        println!("No specific emergency pattern matched, but your symptoms");
        println!("are associated with high-severity conditions:");
        println!();
        for r in results.iter().take(3).filter(|r| r.severity == "high") {
            println!("  🔴 {} ({:.0}%)", r.disease_name, r.probability);
        }
        println!();
        println!("{}", "Seek medical attention promptly.".yellow().bold());
    }
}
