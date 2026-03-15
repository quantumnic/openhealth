use rusqlite::Connection;
use colored::*;

/// Multi-drug interaction checker — checks all pairwise interactions for a list of medications.
pub fn run(_conn: &Connection, drugs_input: &str, json: bool) {
    let drugs: Vec<&str> = drugs_input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if drugs.len() < 2 {
        if json {
            println!("{{\"error\": \"Please provide at least 2 medications separated by commas\"}}");
        } else {
            eprintln!("{}", "Please provide at least 2 medications separated by commas.".red());
        }
        return;
    }

    let interactions = get_interaction_database();
    let mut found_interactions = Vec::new();
    let mut risk_score: u32 = 0;

    // Check all pairs
    for i in 0..drugs.len() {
        for j in (i + 1)..drugs.len() {
            let a = drugs[i].to_lowercase();
            let b = drugs[j].to_lowercase();
            for inter in &interactions {
                let (d1, d2) = (&inter.drug_a.to_lowercase(), &inter.drug_b.to_lowercase());
                if (a.contains(d1) || d1.contains(&a)) && (b.contains(d2) || d2.contains(&b))
                    || (a.contains(d2) || d2.contains(&a)) && (b.contains(d1) || d1.contains(&b))
                {
                    found_interactions.push((drugs[i], drugs[j], inter));
                    risk_score += match inter.severity {
                        "critical" => 10,
                        "high" => 7,
                        "moderate" => 4,
                        "low" => 1,
                        _ => 2,
                    };
                }
            }
        }
    }

    if json {
        print_json(&drugs, &found_interactions, risk_score);
    } else {
        print_table(&drugs, &found_interactions, risk_score);
    }

    // Check for therapeutic duplication
    let duplications = check_therapeutic_duplication(&drugs);
    if !duplications.is_empty() && !json {
        println!("\n{}", "⚠️  Therapeutic Duplication Warnings:".yellow().bold());
        for dup in &duplications {
            println!("  {} {} and {} are both {}",
                "•".yellow(), dup.0.bold(), dup.1.bold(), dup.2);
        }
    }

    // Renal/hepatic warnings
    let organ_warnings = check_organ_burden(&drugs);
    if !organ_warnings.is_empty() && !json {
        println!("\n{}", "🏥 Organ Burden Warnings:".yellow().bold());
        for w in &organ_warnings {
            println!("  {} {}", "•".yellow(), w);
        }
    }
}

struct DrugInteraction {
    drug_a: &'static str,
    drug_b: &'static str,
    severity: &'static str,
    effect: &'static str,
    recommendation: &'static str,
}

fn get_interaction_database() -> Vec<DrugInteraction> {
    vec![
        DrugInteraction { drug_a: "warfarin", drug_b: "aspirin", severity: "high", effect: "Increased bleeding risk — both anticoagulant/antiplatelet", recommendation: "Avoid combination unless specifically indicated. Monitor INR closely." },
        DrugInteraction { drug_a: "warfarin", drug_b: "ibuprofen", severity: "high", effect: "Increased bleeding risk + GI ulceration", recommendation: "Avoid NSAIDs with warfarin. Use acetaminophen for pain if needed." },
        DrugInteraction { drug_a: "metformin", drug_b: "alcohol", severity: "moderate", effect: "Increased risk of lactic acidosis", recommendation: "Limit alcohol intake. Monitor for symptoms of lactic acidosis." },
        DrugInteraction { drug_a: "lisinopril", drug_b: "potassium", severity: "high", effect: "Hyperkalemia risk — ACE inhibitors retain potassium", recommendation: "Monitor serum potassium levels regularly. Avoid potassium supplements unless directed." },
        DrugInteraction { drug_a: "lisinopril", drug_b: "spironolactone", severity: "high", effect: "Severe hyperkalemia risk — dual potassium-sparing effect", recommendation: "If combination necessary, monitor potassium frequently. Start low dose." },
        DrugInteraction { drug_a: "simvastatin", drug_b: "erythromycin", severity: "critical", effect: "Rhabdomyolysis risk — CYP3A4 inhibition increases statin levels", recommendation: "Contraindicated. Use alternative antibiotic or switch statin." },
        DrugInteraction { drug_a: "simvastatin", drug_b: "grapefruit", severity: "moderate", effect: "Increased statin levels via CYP3A4 inhibition", recommendation: "Avoid grapefruit juice. Consider rosuvastatin (not CYP3A4 metabolized)." },
        DrugInteraction { drug_a: "methotrexate", drug_b: "ibuprofen", severity: "critical", effect: "Reduced methotrexate clearance — toxicity risk (pancytopenia, renal failure)", recommendation: "Avoid NSAIDs with methotrexate. Use acetaminophen." },
        DrugInteraction { drug_a: "ssri", drug_b: "tramadol", severity: "high", effect: "Serotonin syndrome risk", recommendation: "Avoid combination. If needed, monitor closely for agitation, hyperthermia, clonus." },
        DrugInteraction { drug_a: "ssri", drug_b: "maoi", severity: "critical", effect: "Life-threatening serotonin syndrome", recommendation: "Absolutely contraindicated. 14-day washout period between." },
        DrugInteraction { drug_a: "fluoxetine", drug_b: "maoi", severity: "critical", effect: "Life-threatening serotonin syndrome", recommendation: "Contraindicated. 5-week washout for fluoxetine before MAOI." },
        DrugInteraction { drug_a: "ciprofloxacin", drug_b: "antacid", severity: "moderate", effect: "Reduced ciprofloxacin absorption — chelation with metal ions", recommendation: "Separate by 2 hours. Take ciprofloxacin first." },
        DrugInteraction { drug_a: "digoxin", drug_b: "amiodarone", severity: "high", effect: "Increased digoxin levels — toxicity risk (arrhythmias, vision changes)", recommendation: "Reduce digoxin dose by 50% when starting amiodarone. Monitor levels." },
        DrugInteraction { drug_a: "clopidogrel", drug_b: "omeprazole", severity: "moderate", effect: "Reduced clopidogrel activation via CYP2C19 inhibition", recommendation: "Use pantoprazole instead of omeprazole. Avoid esomeprazole." },
        DrugInteraction { drug_a: "lithium", drug_b: "ibuprofen", severity: "high", effect: "Increased lithium levels — NSAIDs reduce renal lithium clearance", recommendation: "Avoid NSAIDs. If needed, monitor lithium levels closely." },
        DrugInteraction { drug_a: "amlodipine", drug_b: "simvastatin", severity: "moderate", effect: "Increased simvastatin exposure — CYP3A4 competition", recommendation: "Limit simvastatin to 20 mg/day when combined with amlodipine." },
        DrugInteraction { drug_a: "metronidazole", drug_b: "alcohol", severity: "high", effect: "Disulfiram-like reaction — severe nausea, vomiting, flushing", recommendation: "Absolutely avoid alcohol during and 48h after metronidazole." },
        DrugInteraction { drug_a: "insulin", drug_b: "beta-blocker", severity: "moderate", effect: "Masked hypoglycemia symptoms — beta-blockers block tachycardia warning sign", recommendation: "Monitor blood glucose more frequently. Educate patient on non-adrenergic hypoglycemia signs." },
        DrugInteraction { drug_a: "theophylline", drug_b: "ciprofloxacin", severity: "high", effect: "Increased theophylline levels — seizure and arrhythmia risk", recommendation: "Reduce theophylline dose by 30-50%. Monitor levels. Consider alternative antibiotic." },
        DrugInteraction { drug_a: "aspirin", drug_b: "ibuprofen", severity: "moderate", effect: "Ibuprofen blocks aspirin's antiplatelet effect when taken first", recommendation: "Take aspirin 30 min before ibuprofen. Consider alternative analgesic." },
    ]
}

fn check_therapeutic_duplication<'a>(drugs: &[&'a str]) -> Vec<(&'a str, &'a str, &'static str)> {
    let mut dups = Vec::new();
    let classes: Vec<(&str, &[&str])> = vec![
        ("NSAIDs", &["ibuprofen", "naproxen", "diclofenac", "aspirin", "celecoxib", "meloxicam", "indomethacin", "ketorolac"]),
        ("ACE inhibitors", &["lisinopril", "enalapril", "ramipril", "captopril", "perindopril", "benazepril"]),
        ("Statins", &["simvastatin", "atorvastatin", "rosuvastatin", "pravastatin", "lovastatin", "fluvastatin"]),
        ("SSRIs", &["fluoxetine", "sertraline", "paroxetine", "citalopram", "escitalopram", "fluvoxamine"]),
        ("PPIs", &["omeprazole", "pantoprazole", "esomeprazole", "lansoprazole", "rabeprazole"]),
        ("Beta-blockers", &["metoprolol", "atenolol", "propranolol", "bisoprolol", "carvedilol", "nebivolol"]),
        ("Benzodiazepines", &["diazepam", "lorazepam", "alprazolam", "clonazepam", "midazolam", "temazepam"]),
        ("Opioids", &["morphine", "codeine", "oxycodone", "hydrocodone", "fentanyl", "tramadol", "methadone"]),
    ];

    for (class_name, members) in &classes {
        let mut found: Vec<&str> = Vec::new();
        for drug in drugs {
            let dl = drug.to_lowercase();
            for member in *members {
                if dl.contains(member) {
                    found.push(drug);
                    break;
                }
            }
        }
        if found.len() >= 2 {
            dups.push((found[0], found[1], *class_name));
        }
    }
    dups
}

fn check_organ_burden(drugs: &[&str]) -> Vec<String> {
    let mut warnings = Vec::new();
    let hepatotoxic = ["acetaminophen", "paracetamol", "methotrexate", "amiodarone", "isoniazid", "valproate", "statins", "simvastatin", "atorvastatin"];
    let nephrotoxic = ["ibuprofen", "naproxen", "diclofenac", "gentamicin", "vancomycin", "lithium", "cisplatin", "methotrexate", "amphotericin"];

    let mut hepato_count = 0;
    let mut nephro_count = 0;
    for drug in drugs {
        let dl = drug.to_lowercase();
        if hepatotoxic.iter().any(|h| dl.contains(h)) { hepato_count += 1; }
        if nephrotoxic.iter().any(|n| dl.contains(n)) { nephro_count += 1; }
    }
    if hepato_count >= 2 {
        warnings.push(format!("{} hepatotoxic drugs detected — increased liver injury risk. Monitor LFTs.", hepato_count));
    }
    if nephro_count >= 2 {
        warnings.push(format!("{} nephrotoxic drugs detected — increased renal injury risk. Monitor creatinine/GFR.", nephro_count));
    }
    warnings
}

fn print_table(drugs: &[&str], interactions: &[(&str, &str, &DrugInteraction)], risk_score: u32) {
    println!("\n{}", "💊 Polypharmacy Interaction Check".bold());
    println!("{}", "─".repeat(50));
    println!("{} {}", "Medications:".bold(), drugs.join(", "));
    println!("{} {}", "Total checked:".bold(), drugs.len());
    println!("{} {}/{}", "Pairs analyzed:".bold(), drugs.len() * (drugs.len() - 1) / 2, drugs.len() * (drugs.len() - 1) / 2);

    if interactions.is_empty() {
        println!("\n{}", "✅ No known interactions found between these medications.".green().bold());
        println!("{}", "Note: Always consult a healthcare provider. This database does not cover all possible interactions.".dimmed());
        return;
    }

    let risk_label = match risk_score {
        0..=3 => "LOW".green().bold(),
        4..=10 => "MODERATE".yellow().bold(),
        11..=20 => "HIGH".red().bold(),
        _ => "CRITICAL".on_red().white().bold(),
    };
    println!("{} {}", "\n⚡ Overall Risk Score:".bold(), risk_label);

    println!("\n{}", "Interactions Found:".bold().underline());
    for (i, (a, b, inter)) in interactions.iter().enumerate() {
        let sev_display = match inter.severity {
            "critical" => inter.severity.to_uppercase().on_red().white().bold().to_string(),
            "high" => inter.severity.to_uppercase().red().bold().to_string(),
            "moderate" => inter.severity.to_uppercase().yellow().bold().to_string(),
            _ => inter.severity.to_uppercase().to_string(),
        };

        println!("\n{}. {} ↔ {} [{}]", i + 1, a.bold(), b.bold(), sev_display);
        println!("   Effect: {}", inter.effect);
        println!("   Action: {}", inter.recommendation.cyan());
    }

    println!("\n{}", "⚕️  Disclaimer: This is a reference tool, not medical advice. Always consult a healthcare provider or pharmacist.".dimmed());
}

fn print_json(drugs: &[&str], interactions: &[(&str, &str, &DrugInteraction)], risk_score: u32) {
    let inters: Vec<serde_json::Value> = interactions.iter().map(|(a, b, inter)| {
        serde_json::json!({
            "drug_a": a,
            "drug_b": b,
            "severity": inter.severity,
            "effect": inter.effect,
            "recommendation": inter.recommendation,
        })
    }).collect();

    let out = serde_json::json!({
        "medications": drugs,
        "pairs_analyzed": drugs.len() * (drugs.len() - 1) / 2,
        "risk_score": risk_score,
        "interactions": inters,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
