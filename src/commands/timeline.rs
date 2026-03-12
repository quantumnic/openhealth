use colored::*;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct TimelineResult {
    disease: String,
    phases: Vec<Phase>,
    total_duration: String,
    warning_signs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Phase {
    name: String,
    timeframe: String,
    description: String,
    key_symptoms: Vec<String>,
}

/// Built-in disease progression timelines for common conditions.
fn get_timelines() -> HashMap<&'static str, TimelineData> {
    let mut map = HashMap::new();

    map.insert("Malaria", TimelineData {
        phases: vec![
            ("Incubation", "7-30 days", "Parasite multiplies in liver. No symptoms.", vec![]),
            ("Prodrome", "1-2 days", "Non-specific symptoms begin.", vec!["fatigue", "malaise", "mild headache"]),
            ("Acute paroxysms", "Days 3-7", "Classic cyclic fever pattern every 48-72h.", vec!["high fever", "rigors/chills", "profuse sweating", "headache", "nausea"]),
            ("Crisis (if untreated)", "Week 2+", "Risk of severe/cerebral malaria.", vec!["confusion", "seizures", "severe anemia", "respiratory distress"]),
            ("Recovery", "2-4 weeks", "With treatment, symptoms resolve. Relapses possible with P. vivax.", vec!["resolving fever", "lingering fatigue"]),
        ],
        total_duration: "2-6 weeks (treated); fatal if untreated cerebral malaria",
        warning_signs: vec!["confusion or altered consciousness", "severe anemia (extreme pallor)", "respiratory distress", "jaundice", "repeated vomiting"],
    });

    map.insert("Heart Attack", TimelineData {
        phases: vec![
            ("Warning signs", "Hours to weeks before", "Some patients have prodromal symptoms.", vec!["unusual fatigue", "mild chest discomfort", "shortness of breath on exertion"]),
            ("Acute event", "Minutes", "Sudden onset of symptoms.", vec!["crushing chest pain", "left arm/jaw pain", "cold sweat", "nausea", "shortness of breath"]),
            ("Golden hour", "0-60 minutes", "Critical window for intervention. Every minute counts.", vec!["worsening chest pain", "anxiety", "feeling of doom"]),
            ("Early hospital", "1-12 hours", "PCI or thrombolytics. Arrhythmia monitoring.", vec!["variable pain", "ECG changes"]),
            ("Recovery", "Days to weeks", "Cardiac rehabilitation begins.", vec!["fatigue", "limited exertion tolerance"]),
        ],
        total_duration: "Acute event: minutes. Recovery: 4-12 weeks",
        warning_signs: vec!["chest pain lasting >15 minutes", "pain radiating to arm, jaw, or back", "sudden breathlessness at rest", "loss of consciousness"],
    });

    map.insert("Stroke", TimelineData {
        phases: vec![
            ("Onset", "Seconds to minutes", "Sudden neurological deficit. FAST: Face, Arms, Speech, Time.", vec!["facial drooping", "arm weakness", "speech difficulty"]),
            ("Golden window", "0-4.5 hours", "tPA (clot-buster) effective within this window.", vec!["progressing or stable deficits"]),
            ("Acute phase", "24-72 hours", "Monitoring for extension, edema, hemorrhagic transformation.", vec!["neurological fluctuations", "risk of aspiration"]),
            ("Early recovery", "1-4 weeks", "Brain plasticity. Intensive rehabilitation begins.", vec!["improving deficits", "fatigue"]),
            ("Chronic recovery", "3-12 months", "Continued rehab. Plateau typically at 6 months.", vec!["residual deficits", "depression", "spasticity"]),
        ],
        total_duration: "Acute: hours. Recovery: months to years",
        warning_signs: vec!["sudden severe headache", "sudden vision loss", "sudden confusion", "sudden weakness on one side"],
    });

    map.insert("Pneumonia", TimelineData {
        phases: vec![
            ("Onset", "Days 1-3", "Initial infection takes hold.", vec!["fever", "cough", "malaise"]),
            ("Progression", "Days 3-5", "Lung consolidation develops.", vec!["productive cough", "high fever", "chest pain", "shortness of breath"]),
            ("Peak illness", "Days 5-7", "Highest risk of complications.", vec!["high fever", "severe cough", "tachypnea", "possible confusion in elderly"]),
            ("Recovery", "Weeks 1-3", "With antibiotics, improvement in 48-72h.", vec!["resolving fever", "persistent cough", "fatigue"]),
            ("Full recovery", "3-6 weeks", "Complete resolution. Chest X-ray may lag behind.", vec!["mild cough", "reduced stamina"]),
        ],
        total_duration: "2-3 weeks (treated community-acquired)",
        warning_signs: vec!["SpO2 <92%", "confusion (especially elderly)", "unable to drink fluids", "rapid breathing >30/min"],
    });

    map.insert("COVID-19", TimelineData {
        phases: vec![
            ("Incubation", "2-14 days (avg 5)", "Virus replicating. Potentially contagious late in this phase.", vec![]),
            ("Early symptoms", "Days 1-5", "Mild upper respiratory and systemic symptoms.", vec!["fever", "cough", "fatigue", "loss of taste/smell", "sore throat"]),
            ("Progression (if moderate)", "Days 5-8", "Pulmonary involvement begins in some patients.", vec!["shortness of breath", "persistent fever", "chest tightness"]),
            ("Critical (if severe)", "Days 8-12", "Cytokine storm risk. ARDS possible.", vec!["severe dyspnea", "hypoxemia", "confusion"]),
            ("Recovery", "2-6 weeks", "Most recover. Long COVID possible.", vec!["fatigue", "brain fog", "exertional dyspnea"]),
        ],
        total_duration: "Mild: 1-2 weeks. Severe: 3-6 weeks. Long COVID: months",
        warning_signs: vec!["difficulty breathing at rest", "persistent chest pain", "confusion", "bluish lips/face", "inability to stay awake"],
    });

    map.insert("Appendicitis", TimelineData {
        phases: vec![
            ("Early", "0-12 hours", "Periumbilical pain, vague discomfort.", vec!["periumbilical pain", "nausea", "loss of appetite"]),
            ("Localization", "12-24 hours", "Pain migrates to right lower quadrant.", vec!["RLQ pain", "fever", "rebound tenderness"]),
            ("Perforation risk", "24-72 hours", "Appendix at risk of rupture.", vec!["worsening pain", "high fever", "rigid abdomen"]),
            ("Perforation (if untreated)", "48-72+ hours", "Peritonitis develops.", vec!["severe generalized pain", "high fever", "sepsis signs"]),
        ],
        total_duration: "24-72 hours to perforation if untreated",
        warning_signs: vec!["sudden worsening then brief relief (may indicate perforation)", "high fever >39°C", "rigid abdomen", "rapid heart rate"],
    });

    map.insert("Dengue Fever", TimelineData {
        phases: vec![
            ("Incubation", "4-10 days", "Virus multiplying after mosquito bite.", vec![]),
            ("Febrile phase", "Days 1-3", "High fever and systemic symptoms.", vec!["high fever", "severe headache", "retro-orbital pain", "muscle pain", "rash"]),
            ("Critical phase", "Days 3-7", "Defervescence period — HIGHEST RISK. Plasma leakage possible.", vec!["dropping fever", "abdominal pain", "persistent vomiting", "bleeding"]),
            ("Recovery", "Days 7-10", "Fluid reabsorption. Appetite returns.", vec!["improving appetite", "rash", "itching", "bradycardia"]),
        ],
        total_duration: "7-10 days. Critical phase day 3-7 most dangerous",
        warning_signs: vec!["abdominal pain or tenderness", "persistent vomiting", "mucosal bleeding", "lethargy/restlessness", "liver enlargement", "rising hematocrit with dropping platelets"],
    });

    map
}

struct TimelineData {
    phases: Vec<(&'static str, &'static str, &'static str, Vec<&'static str>)>,
    total_duration: &'static str,
    warning_signs: Vec<&'static str>,
}

pub fn run(conn: &Connection, name: &str, json: bool) {
    let timelines = get_timelines();

    // Find disease by fuzzy name match
    let name_lower = name.to_lowercase();
    let matched = timelines
        .iter()
        .find(|(k, _)| k.to_lowercase().contains(&name_lower) || name_lower.contains(&k.to_lowercase()));

    // Also verify disease exists in DB
    let db_disease: Option<String> = conn
        .query_row(
            "SELECT name FROM diseases WHERE LOWER(name) LIKE ?1",
            [format!("%{name_lower}%")],
            |r| r.get(0),
        )
        .ok();

    if let Some((disease_key, data)) = matched {
        let phases: Vec<Phase> = data
            .phases
            .iter()
            .map(|(name, timeframe, desc, symptoms)| Phase {
                name: name.to_string(),
                timeframe: timeframe.to_string(),
                description: desc.to_string(),
                key_symptoms: symptoms.iter().map(|s| s.to_string()).collect(),
            })
            .collect();

        if json {
            let result = TimelineResult {
                disease: disease_key.to_string(),
                phases,
                total_duration: data.total_duration.to_string(),
                warning_signs: data.warning_signs.iter().map(|s| s.to_string()).collect(),
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
            );
            return;
        }

        println!();
        println!(
            "{}",
            "╔══════════════════════════════════════════════════════════╗"
                .bright_cyan()
        );
        println!(
            "{}",
            "║           📅  DISEASE PROGRESSION TIMELINE              ║"
                .bright_cyan()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════════════════════╝"
                .bright_cyan()
        );
        println!();
        println!("Disease: {}", disease_key.bright_white().bold());
        println!(
            "Expected duration: {}",
            data.total_duration.bright_yellow()
        );
        println!();

        for (i, phase) in data.phases.iter().enumerate() {
            let (name, timeframe, desc, symptoms) = phase;
            let marker = if i == 0 { "┌" } else if i == data.phases.len() - 1 { "└" } else { "├" };
            println!(
                "  {} {} {}",
                marker.bright_cyan(),
                format!("[{timeframe}]").bright_yellow(),
                name.bold(),
            );
            println!("  │   {desc}");
            if !symptoms.is_empty() {
                println!(
                    "  │   Symptoms: {}",
                    symptoms.join(", ").bright_white()
                );
            }
            println!("  │");
        }

        if !data.warning_signs.is_empty() {
            println!();
            println!("{}", "⚠️  WARNING SIGNS — seek immediate help if:".red().bold());
            for sign in &data.warning_signs {
                println!("  🔴 {}", sign.red());
            }
        }

        println!();
        println!(
            "{}",
            "⚠️  Timelines vary by individual. This is a general guide."
                .yellow()
        );
    } else {
        if json {
            println!("{{\"error\": \"No timeline available for '{}'\"}}", name);
            return;
        }

        println!();
        if let Some(db_name) = db_disease {
            println!(
                "Disease '{}' exists in the database but no progression timeline is available yet.",
                db_name
            );
        } else {
            println!("Disease '{}' not found.", name);
        }
        println!();
        println!("{}", "Available timelines:".bold());
        let mut available: Vec<&&str> = timelines.keys().collect();
        available.sort();
        for name in available {
            println!("  • {name}");
        }
    }
    println!();
}
