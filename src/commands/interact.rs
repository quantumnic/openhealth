use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct InteractionResult {
    drug: String,
    interactions: Vec<DrugInteraction>,
}

#[derive(Debug, Serialize)]
struct DrugInteraction {
    disease: String,
    severity: String,
    warning: String,
    recommendation: String,
}

/// Known drug-disease interactions database (embedded).
/// Each entry: (drug_keyword, disease_name, severity, warning, recommendation)
const INTERACTIONS: &[(&str, &str, &str, &str, &str)] = &[
    // NSAIDs
    ("ibuprofen", "Asthma", "high", "Can trigger bronchospasm in aspirin-sensitive asthma", "Avoid in aspirin-sensitive patients. Use paracetamol instead."),
    ("ibuprofen", "Peptic Ulcer Disease", "high", "Increases risk of GI bleeding and ulcer perforation", "Avoid or use with PPI cover. Prefer paracetamol."),
    ("ibuprofen", "Chronic Kidney Disease", "high", "Can worsen renal function and cause acute kidney injury", "Avoid in CKD stages 3-5. Monitor renal function if unavoidable."),
    ("ibuprofen", "Heart Attack", "moderate", "Increased cardiovascular risk with prolonged use", "Use lowest dose for shortest duration. Naproxen may be safer."),
    ("ibuprofen", "Hypertension", "moderate", "Can raise blood pressure and reduce antihypertensive efficacy", "Monitor BP. Use lowest effective dose."),
    ("aspirin", "Peptic Ulcer Disease", "high", "High risk of GI bleeding", "Avoid if possible. If essential (cardiac), add PPI cover."),
    ("aspirin", "Dengue Fever", "high", "Increases hemorrhagic risk in dengue", "NEVER use aspirin in suspected dengue. Use paracetamol only."),
    ("aspirin", "Dengue Hemorrhagic Fever", "high", "Worsens bleeding and thrombocytopenia", "Strictly contraindicated. Use paracetamol."),
    ("naproxen", "Chronic Kidney Disease", "high", "Nephrotoxic, can worsen kidney function", "Avoid in advanced CKD. Use with caution in early CKD."),

    // Corticosteroids
    ("prednisone", "Diabetes Type 2", "high", "Causes hyperglycemia, may precipitate diabetic ketoacidosis", "Monitor blood glucose closely. Adjust diabetic medications."),
    ("prednisone", "Hypertension", "moderate", "Causes fluid retention and raises blood pressure", "Monitor BP. May need antihypertensive adjustment."),
    ("prednisone", "Osteoporosis", "high", "Accelerates bone loss, increases fracture risk", "Use lowest dose. Add calcium/vitamin D. Consider bisphosphonate."),
    ("prednisone", "Peptic Ulcer Disease", "moderate", "Increased ulcer risk especially with concurrent NSAIDs", "Add PPI if prolonged use. Avoid combining with NSAIDs."),
    ("dexamethasone", "Diabetes Type 2", "high", "Potent hyperglycemic effect", "Monitor blood glucose. Insulin may be required."),

    // Antibiotics
    ("metronidazole", "Liver Cirrhosis", "moderate", "Hepatic metabolism impaired, increased toxicity risk", "Reduce dose. Monitor for neurotoxicity."),
    ("aminoglycoside", "Myasthenia Gravis", "high", "Can precipitate myasthenic crisis", "Avoid aminoglycosides. Use alternative antibiotics."),
    ("gentamicin", "Myasthenia Gravis", "high", "Neuromuscular blockade, may worsen weakness", "Contraindicated. Use alternative antibiotics."),
    ("ciprofloxacin", "Epilepsy", "moderate", "Lowers seizure threshold", "Use with caution. Consider alternative antibiotic."),
    ("ciprofloxacin", "Myasthenia Gravis", "moderate", "May worsen muscle weakness", "Use with caution. Monitor closely."),

    // Cardiovascular
    ("beta-blocker", "Asthma", "high", "Can cause severe bronchospasm", "Avoid non-selective beta-blockers. Cardioselective (bisoprolol) may be used cautiously."),
    ("beta-blocker", "Diabetes Type 2", "moderate", "Masks hypoglycemia symptoms, impairs glucose recovery", "Monitor blood glucose. Use cardioselective agents."),
    ("metformin", "Chronic Kidney Disease", "high", "Risk of lactic acidosis with reduced renal clearance", "Contraindicated if eGFR <30. Reduce dose if eGFR 30-45."),
    ("metformin", "Liver Cirrhosis", "high", "Increased risk of lactic acidosis", "Avoid in decompensated liver disease."),
    ("warfarin", "Peptic Ulcer Disease", "high", "High risk of life-threatening GI hemorrhage", "Add PPI. Monitor INR closely. Consider alternative anticoagulant."),
    ("warfarin", "Liver Cirrhosis", "high", "Impaired clotting factor synthesis, unpredictable INR", "Reduce dose. Frequent INR monitoring."),
    ("ace-inhibitor", "Chronic Kidney Disease", "moderate", "Can cause hyperkalemia and acute kidney injury", "Monitor potassium and creatinine. Start low dose."),
    ("statin", "Liver Cirrhosis", "moderate", "Hepatotoxicity risk in decompensated liver disease", "Avoid in decompensated cirrhosis. Monitor LFTs in compensated."),

    // Psychiatric
    ("ssri", "Epilepsy", "moderate", "Some SSRIs lower seizure threshold", "Use with caution. Sertraline may be safest option."),
    ("lithium", "Chronic Kidney Disease", "high", "Nephrotoxic, accumulates with impaired clearance", "Monitor levels and renal function. Dose reduction required."),
    ("lithium", "Hypothyroidism", "moderate", "Can worsen or cause hypothyroidism", "Monitor thyroid function regularly."),
    ("benzodiazepine", "COPD", "high", "Respiratory depression, can worsen hypoxemia", "Avoid or use very cautiously at lowest dose."),
    ("benzodiazepine", "Liver Cirrhosis", "high", "Prolonged sedation, may precipitate hepatic encephalopathy", "Avoid if possible. Oxazepam preferred (no hepatic metabolism)."),

    // Antimalarials
    ("chloroquine", "Epilepsy", "moderate", "Lowers seizure threshold", "Use with caution. Monitor closely."),
    ("chloroquine", "Myasthenia Gravis", "high", "Can worsen neuromuscular weakness", "Avoid. Use alternative antimalarial."),

    // Analgesics
    ("paracetamol", "Liver Cirrhosis", "moderate", "Hepatotoxicity risk at lower doses in liver disease", "Max 2g/day. Avoid in decompensated disease."),
    ("opioid", "COPD", "high", "Respiratory depression risk", "Use lowest dose. Avoid in severe COPD. Monitor SpO2."),
    ("opioid", "Liver Cirrhosis", "high", "Prolonged effect, may precipitate encephalopathy", "Reduce dose. Avoid codeine (prodrug). Prefer low-dose fentanyl."),

    // Immunosuppressants
    ("allopurinol", "Chronic Kidney Disease", "moderate", "Dose adjustment required, risk of severe hypersensitivity", "Start low (100mg). HLA-B*5801 testing before starting."),

    // Anticoagulants
    ("heparin", "Peptic Ulcer Disease", "high", "Risk of GI hemorrhage", "Use with caution. Add PPI. Monitor for bleeding."),
];

pub fn run(conn: &Connection, drug: &str, json: bool) {
    let drug_lower = drug.trim().to_lowercase();

    let interactions: Vec<DrugInteraction> = INTERACTIONS
        .iter()
        .filter(|(d, _, _, _, _)| drug_lower.contains(d) || d.contains(drug_lower.as_str()))
        .map(|(_, disease, sev, warning, rec)| {
            // Check if this disease exists in our database for enrichment
            let disease_exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) FROM diseases WHERE name = ?1",
                    [disease],
                    |r| r.get::<_, i64>(0),
                )
                .map(|c| c > 0)
                .unwrap_or(false);
            let _ = disease_exists; // Available for future enrichment

            DrugInteraction {
                disease: disease.to_string(),
                severity: sev.to_string(),
                warning: warning.to_string(),
                recommendation: rec.to_string(),
            }
        })
        .collect();

    if json {
        let result = InteractionResult {
            drug: drug.to_string(),
            interactions,
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
        "║           💊  DRUG-DISEASE INTERACTIONS                  ║"
            .bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝"
            .bright_cyan()
    );
    println!();
    println!(
        "Drug: {}",
        drug.bright_white().bold()
    );
    println!();

    if interactions.is_empty() {
        println!("No known interactions found for '{}'.", drug);
        println!();
        println!(
            "{}",
            "Tip: Try common drug names like 'ibuprofen', 'aspirin', 'prednisone', 'metformin'."
                .yellow()
        );
    } else {
        println!(
            "Found {} interaction(s):",
            interactions.len().to_string().bold()
        );
        println!();

        for (i, inter) in interactions.iter().enumerate() {
            let sev_display = match inter.severity.as_str() {
                "high" => "🔴 HIGH".red().bold().to_string(),
                "moderate" => "🟡 MODERATE".yellow().bold().to_string(),
                _ => "🟢 LOW".green().to_string(),
            };

            println!("  {}. {} × {}", i + 1, drug.bright_white(), inter.disease.bright_white());
            println!("     Severity: {sev_display}");
            println!("     ⚠️  {}", inter.warning);
            println!("     ✅ {}", inter.recommendation.green());
            println!();
        }
    }

    println!(
        "{}",
        "⚠️  This is reference information only. Always consult a pharmacist or physician."
            .yellow()
    );
    println!();
}
