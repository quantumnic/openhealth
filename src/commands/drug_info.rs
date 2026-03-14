use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DrugInfo {
    pub name: &'static str,
    pub class: &'static str,
    pub common_uses: &'static str,
    pub typical_dose: &'static str,
    pub common_side_effects: &'static str,
    pub serious_warnings: &'static str,
    pub contraindications: &'static str,
}

const DRUGS: &[DrugInfo] = &[
    DrugInfo {
        name: "Ibuprofen",
        class: "NSAID",
        common_uses: "Pain, inflammation, fever, headache, menstrual cramps, arthritis",
        typical_dose: "200-400mg every 4-6h (max 1200mg/day OTC, 3200mg/day Rx)",
        common_side_effects: "GI upset, nausea, heartburn, dizziness",
        serious_warnings: "GI bleeding, kidney injury, cardiovascular events with long-term use",
        contraindications: "Active GI bleed, severe renal failure, third trimester pregnancy, aspirin-sensitive asthma",
    },
    DrugInfo {
        name: "Paracetamol (Acetaminophen)",
        class: "Analgesic / Antipyretic",
        common_uses: "Pain, fever, headache",
        typical_dose: "500-1000mg every 4-6h (max 4g/day; 2g/day if liver disease)",
        common_side_effects: "Generally well tolerated at therapeutic doses",
        serious_warnings: "Hepatotoxicity in overdose — leading cause of acute liver failure",
        contraindications: "Severe liver disease, active liver failure",
    },
    DrugInfo {
        name: "Amoxicillin",
        class: "Penicillin antibiotic",
        common_uses: "Ear infections, sinusitis, pneumonia, UTIs, H. pylori",
        typical_dose: "250-500mg every 8h or 500-875mg every 12h",
        common_side_effects: "Diarrhea, nausea, rash",
        serious_warnings: "Anaphylaxis (penicillin allergy), C. diff colitis, antibiotic resistance",
        contraindications: "Penicillin allergy, history of amoxicillin-associated hepatitis",
    },
    DrugInfo {
        name: "Metformin",
        class: "Biguanide (antidiabetic)",
        common_uses: "Type 2 diabetes, PCOS, prediabetes",
        typical_dose: "500mg twice daily, titrate to 2000mg/day",
        common_side_effects: "GI upset, diarrhea, nausea, metallic taste",
        serious_warnings: "Lactic acidosis (rare, with renal impairment). Withhold before contrast dye.",
        contraindications: "eGFR <30, metabolic acidosis, acute heart failure, severe liver disease",
    },
    DrugInfo {
        name: "Omeprazole",
        class: "Proton pump inhibitor (PPI)",
        common_uses: "GERD, peptic ulcers, H. pylori eradication, Zollinger-Ellison",
        typical_dose: "20-40mg once daily before breakfast",
        common_side_effects: "Headache, nausea, abdominal pain, diarrhea",
        serious_warnings: "Long-term: C. diff, bone fractures, Mg/B12 deficiency, fundic gland polyps",
        contraindications: "Known hypersensitivity to PPIs. Avoid with rilpivirine.",
    },
    DrugInfo {
        name: "Lisinopril",
        class: "ACE inhibitor",
        common_uses: "Hypertension, heart failure, post-MI, diabetic nephropathy",
        typical_dose: "10-40mg once daily",
        common_side_effects: "Dry cough, dizziness, hyperkalemia, headache",
        serious_warnings: "Angioedema (life-threatening), acute kidney injury, hyperkalemia",
        contraindications: "Pregnancy, bilateral renal artery stenosis, history of ACE-inhibitor angioedema",
    },
    DrugInfo {
        name: "Atorvastatin",
        class: "Statin (HMG-CoA reductase inhibitor)",
        common_uses: "Hyperlipidemia, cardiovascular risk reduction",
        typical_dose: "10-80mg once daily (usually evening)",
        common_side_effects: "Myalgia, GI upset, elevated liver enzymes",
        serious_warnings: "Rhabdomyolysis (rare), hepatotoxicity, new-onset diabetes",
        contraindications: "Active liver disease, pregnancy, breastfeeding",
    },
    DrugInfo {
        name: "Salbutamol (Albuterol)",
        class: "Short-acting beta-2 agonist (SABA)",
        common_uses: "Acute asthma, bronchospasm, COPD exacerbation",
        typical_dose: "100-200mcg (1-2 puffs) every 4-6h as needed",
        common_side_effects: "Tremor, palpitations, headache, tachycardia",
        serious_warnings: "Paradoxical bronchospasm, hypokalemia with high doses",
        contraindications: "Hypersensitivity to salbutamol (rare absolute contraindications)",
    },
    DrugInfo {
        name: "Prednisolone",
        class: "Corticosteroid",
        common_uses: "Asthma exacerbation, autoimmune disease, allergic reactions, croup, inflammatory conditions",
        typical_dose: "5-60mg/day depending on indication; taper for courses >7 days",
        common_side_effects: "Insomnia, appetite increase, mood changes, GI upset",
        serious_warnings: "Adrenal suppression, immunosuppression, glucose elevation, osteoporosis, peptic ulcers",
        contraindications: "Systemic fungal infection, live vaccines during high-dose therapy",
    },
    DrugInfo {
        name: "Metoprolol",
        class: "Beta-blocker (beta-1 selective)",
        common_uses: "Hypertension, angina, heart failure, post-MI, tachyarrhythmias",
        typical_dose: "25-200mg once or twice daily (tartrate vs succinate)",
        common_side_effects: "Fatigue, bradycardia, dizziness, cold extremities",
        serious_warnings: "Do not stop abruptly (rebound tachycardia). Severe bradycardia, heart block.",
        contraindications: "Severe bradycardia, heart block >1st degree (without pacemaker), cardiogenic shock, decompensated heart failure",
    },
    DrugInfo {
        name: "Ciprofloxacin",
        class: "Fluoroquinolone antibiotic",
        common_uses: "UTIs, traveler's diarrhea, bone/joint infections, anthrax",
        typical_dose: "250-750mg twice daily",
        common_side_effects: "Nausea, diarrhea, dizziness, photosensitivity",
        serious_warnings: "Tendon rupture, aortic dissection/aneurysm, peripheral neuropathy, QT prolongation, C. diff",
        contraindications: "Myasthenia gravis, concurrent tizanidine. Age <18 (relative). Pregnancy.",
    },
    DrugInfo {
        name: "Diazepam",
        class: "Benzodiazepine",
        common_uses: "Anxiety, seizures, muscle spasm, alcohol withdrawal, procedural sedation",
        typical_dose: "2-10mg 2-4 times daily (anxiety); 5-10mg IV for seizures",
        common_side_effects: "Drowsiness, fatigue, ataxia, memory impairment",
        serious_warnings: "Respiratory depression (especially with opioids), dependence, paradoxical agitation",
        contraindications: "Severe respiratory insufficiency, sleep apnea, myasthenia gravis, acute narrow-angle glaucoma",
    },
];

pub fn run(name: &str, json: bool) {
    let query = name.to_lowercase();
    let matches: Vec<&DrugInfo> = DRUGS
        .iter()
        .filter(|d| {
            d.name.to_lowercase().contains(&query)
                || d.class.to_lowercase().contains(&query)
        })
        .collect();

    if matches.is_empty() {
        if json {
            println!("{{\"error\": \"No drug found matching '{name}'\"}}");
        } else {
            println!("❌ No drug found matching \"{name}\".");
            println!("   Try: ibuprofen, paracetamol, amoxicillin, metformin, omeprazole, lisinopril,");
            println!("        atorvastatin, salbutamol, prednisolone, metoprolol, ciprofloxacin, diazepam");
        }
        return;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&matches).unwrap());
        return;
    }

    for drug in matches {
        println!("💊 {} ({})", drug.name, drug.class);
        println!("   Uses:            {}", drug.common_uses);
        println!("   Typical dose:    {}", drug.typical_dose);
        println!("   Side effects:    {}", drug.common_side_effects);
        println!("   ⚠️  Warnings:     {}", drug.serious_warnings);
        println!("   🚫 Contra:       {}", drug.contraindications);
        println!();
    }
}

pub fn run_list(json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(&DRUGS).unwrap());
        return;
    }

    println!("💊 Drug Information Reference ({} drugs)\n", DRUGS.len());
    let mut by_class: std::collections::BTreeMap<&str, Vec<&DrugInfo>> = std::collections::BTreeMap::new();
    for drug in DRUGS {
        by_class.entry(drug.class).or_default().push(drug);
    }
    for (class, drugs) in &by_class {
        println!("  📋 {class}");
        for drug in drugs {
            println!("     • {}", drug.name);
        }
    }
    println!("\nUse: openhealth drug-info <name> for details");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drug_list_not_empty() {
        assert!(!DRUGS.is_empty());
        assert!(DRUGS.len() >= 10);
    }

    #[test]
    fn test_drug_lookup_ibuprofen() {
        let matches: Vec<&DrugInfo> = DRUGS.iter().filter(|d| d.name.to_lowercase().contains("ibuprofen")).collect();
        assert_eq!(matches.len(), 1);
        assert!(matches[0].class.contains("NSAID"));
    }

    #[test]
    fn test_drug_lookup_by_class() {
        let matches: Vec<&DrugInfo> = DRUGS.iter().filter(|d| d.class.to_lowercase().contains("antibiotic")).collect();
        assert!(matches.len() >= 2, "Should find at least 2 antibiotics");
    }

    #[test]
    fn test_all_drugs_have_fields() {
        for drug in DRUGS {
            assert!(!drug.name.is_empty());
            assert!(!drug.class.is_empty());
            assert!(!drug.common_uses.is_empty());
            assert!(!drug.typical_dose.is_empty());
            assert!(!drug.common_side_effects.is_empty());
            assert!(!drug.serious_warnings.is_empty());
            assert!(!drug.contraindications.is_empty());
        }
    }
}
