use colored::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MedicationInfo {
    pub name: &'static str,
    pub class: &'static str,
    pub uses: &'static str,
    pub dosage: &'static str,
    pub side_effects: &'static str,
    pub contraindications: &'static str,
    pub interactions: &'static str,
    pub pregnancy_category: &'static str,
}

fn get_medications() -> Vec<MedicationInfo> {
    vec![
        MedicationInfo {
            name: "Paracetamol (Acetaminophen)",
            class: "Analgesic / Antipyretic",
            uses: "Pain relief, fever reduction. First-line for mild-moderate pain.",
            dosage: "Adults: 500-1000 mg every 4-6h, max 4g/day. Children: 10-15 mg/kg every 4-6h.",
            side_effects: "Rare at therapeutic doses. Hepatotoxicity in overdose. Allergic reactions (rare).",
            contraindications: "Severe hepatic impairment. Active liver disease. Known hypersensitivity.",
            interactions: "Warfarin (increased INR). Alcohol (hepatotoxicity risk). Carbamazepine. Isoniazid.",
            pregnancy_category: "Generally safe in pregnancy (all trimesters).",
        },
        MedicationInfo {
            name: "Ibuprofen",
            class: "NSAID (Non-Steroidal Anti-Inflammatory Drug)",
            uses: "Pain, inflammation, fever. Arthritis, headache, menstrual cramps, dental pain.",
            dosage: "Adults: 200-400 mg every 4-6h, max 1200 mg/day (OTC) or 2400 mg/day (Rx). Children >6mo: 5-10 mg/kg every 6-8h.",
            side_effects: "GI upset, ulcers, bleeding. Renal impairment. Cardiovascular risk with prolonged use. Allergic reactions.",
            contraindications: "Active GI bleeding. Severe renal/hepatic impairment. Third trimester pregnancy. Aspirin-sensitive asthma.",
            interactions: "Anticoagulants (bleeding risk). ACE inhibitors (reduced effect). Lithium (increased levels). SSRIs (bleeding risk). Methotrexate.",
            pregnancy_category: "Avoid in third trimester. Use with caution in first/second trimester.",
        },
        MedicationInfo {
            name: "Amoxicillin",
            class: "Antibiotic (Penicillin)",
            uses: "Bacterial infections: otitis media, sinusitis, pneumonia, UTI, H. pylori (combo), dental infections.",
            dosage: "Adults: 250-500 mg every 8h or 500-875 mg every 12h. Children: 25-50 mg/kg/day divided every 8h.",
            side_effects: "Diarrhea, nausea, rash. Allergic reactions (including anaphylaxis). C. difficile colitis.",
            contraindications: "Penicillin allergy. History of amoxicillin-associated cholestatic jaundice.",
            interactions: "Methotrexate (increased toxicity). Warfarin (increased INR). Oral contraceptives (reduced efficacy debated).",
            pregnancy_category: "Generally safe in pregnancy (Category B).",
        },
        MedicationInfo {
            name: "Metformin",
            class: "Biguanide (Antidiabetic)",
            uses: "Type 2 diabetes mellitus (first-line). Polycystic ovary syndrome. Prediabetes prevention.",
            dosage: "Start 500 mg once or twice daily with meals. Titrate to 1000 mg twice daily. Max 2550 mg/day.",
            side_effects: "GI upset (nausea, diarrhea, bloating — usually transient). Lactic acidosis (rare). Vitamin B12 deficiency with long-term use.",
            contraindications: "Severe renal impairment (eGFR <30). Acute/chronic metabolic acidosis. Before iodinated contrast (hold 48h).",
            interactions: "Alcohol (lactic acidosis risk). Iodinated contrast agents. Carbonic anhydrase inhibitors.",
            pregnancy_category: "Generally used in gestational diabetes (off-label). Insulin preferred.",
        },
        MedicationInfo {
            name: "Omeprazole",
            class: "Proton Pump Inhibitor (PPI)",
            uses: "GERD, peptic ulcers, H. pylori eradication (combo), Zollinger-Ellison syndrome, NSAID gastroprotection.",
            dosage: "Adults: 20-40 mg once daily before breakfast. Ulcer healing: 4-8 weeks. H. pylori: 20 mg twice daily for 14 days.",
            side_effects: "Headache, nausea, diarrhea. Long-term: fracture risk, C. difficile, hypomagnesemia, vitamin B12 deficiency.",
            contraindications: "Known hypersensitivity. Rilpivirine co-administration.",
            interactions: "Clopidogrel (reduced activation — avoid). Methotrexate (increased levels). Ketoconazole, itraconazole (reduced absorption).",
            pregnancy_category: "Use if clearly needed. Limited data but generally considered safe.",
        },
        MedicationInfo {
            name: "Salbutamol (Albuterol)",
            class: "Short-Acting Beta-2 Agonist (SABA)",
            uses: "Acute bronchospasm, asthma rescue, exercise-induced bronchospasm, COPD exacerbations.",
            dosage: "Inhaler: 1-2 puffs every 4-6h as needed. Nebulizer: 2.5-5 mg every 20 min for acute attacks (up to 3 doses).",
            side_effects: "Tremor, tachycardia, palpitations, headache, hypokalemia. Paradoxical bronchospasm (rare).",
            contraindications: "Known hypersensitivity. Use with caution in cardiovascular disease, hyperthyroidism.",
            interactions: "Beta-blockers (antagonism). MAOIs. Diuretics (hypokalemia). Digoxin.",
            pregnancy_category: "Generally safe. Use when benefit outweighs risk.",
        },
        MedicationInfo {
            name: "Aspirin (Acetylsalicylic Acid)",
            class: "NSAID / Antiplatelet",
            uses: "Pain, fever, inflammation. Low-dose: cardiovascular prophylaxis, post-MI, post-stroke. Kawasaki disease.",
            dosage: "Analgesic: 300-600 mg every 4-6h, max 4g/day. Antiplatelet: 75-100 mg daily. Kawasaki: 80-100 mg/kg/day acute phase.",
            side_effects: "GI bleeding, ulcers. Tinnitus at high doses. Reye syndrome in children with viral illness. Bleeding risk.",
            contraindications: "Children <16 with viral illness (Reye syndrome). Active GI bleeding. Hemophilia. Third trimester pregnancy.",
            interactions: "Anticoagulants (major bleeding risk). Methotrexate. SSRIs. Other NSAIDs. ACE inhibitors.",
            pregnancy_category: "Low-dose may be used for preeclampsia prevention. Avoid in third trimester.",
        },
        MedicationInfo {
            name: "Metoprolol",
            class: "Beta-Blocker (Beta-1 Selective)",
            uses: "Hypertension, angina, heart failure, post-MI, rate control in atrial fibrillation, migraine prophylaxis.",
            dosage: "Hypertension: start 25-50 mg twice daily (tartrate) or 25-100 mg daily (succinate). Max 400 mg/day. Heart failure: start 12.5-25 mg daily, titrate slowly.",
            side_effects: "Bradycardia, fatigue, dizziness, cold extremities, depression, bronchospasm, weight gain.",
            contraindications: "Severe bradycardia. Heart block (2nd/3rd degree). Decompensated heart failure. Cardiogenic shock.",
            interactions: "Calcium channel blockers (additive bradycardia). Digoxin. Clonidine (rebound hypertension). CYP2D6 inhibitors (increased levels).",
            pregnancy_category: "Use with caution. May cause fetal bradycardia and growth restriction.",
        },
        MedicationInfo {
            name: "Prednisolone",
            class: "Corticosteroid",
            uses: "Inflammation, autoimmune disorders, asthma exacerbations, croup, allergic reactions, organ transplant rejection.",
            dosage: "Variable by condition. Asthma flare: 40-60 mg/day for 5-7 days. Autoimmune: 0.5-1 mg/kg/day, taper gradually. Children croup: 1-2 mg/kg single dose.",
            side_effects: "Short-term: mood changes, insomnia, appetite increase, hyperglycemia. Long-term: osteoporosis, Cushing's, adrenal suppression, immunosuppression, cataracts.",
            contraindications: "Systemic fungal infections. Live vaccines during high-dose therapy. Avoid abrupt discontinuation after prolonged use.",
            interactions: "NSAIDs (GI bleeding risk). Diabetes medications (hyperglycemia). Warfarin (altered effect). CYP3A4 inhibitors/inducers.",
            pregnancy_category: "Use when benefit outweighs risk. Minimal placental transfer of prednisolone vs prednisone.",
        },
        MedicationInfo {
            name: "Ciprofloxacin",
            class: "Antibiotic (Fluoroquinolone)",
            uses: "UTI, pyelonephritis, prostatitis, GI infections, bone/joint infections, anthrax prophylaxis.",
            dosage: "Adults: 250-750 mg every 12h. UTI uncomplicated: 250 mg every 12h for 3 days. Pyelonephritis: 500 mg every 12h for 7 days.",
            side_effects: "Tendon rupture, peripheral neuropathy, QT prolongation, C. difficile. GI upset, dizziness, photosensitivity.",
            contraindications: "Children <18 (except specific indications). Concurrent tizanidine. History of tendon disorders with fluoroquinolones.",
            interactions: "Antacids, iron, calcium (reduced absorption — space 2h). Warfarin (increased INR). Theophylline (increased levels). QT-prolonging drugs.",
            pregnancy_category: "Avoid in pregnancy and breastfeeding (cartilage toxicity risk).",
        },
        MedicationInfo {
            name: "Diazepam",
            class: "Benzodiazepine",
            uses: "Anxiety, seizures (status epilepticus), muscle spasm, alcohol withdrawal, procedural sedation.",
            dosage: "Anxiety: 2-10 mg 2-4 times daily. Seizures: 5-10 mg IV (repeat once). Muscle spasm: 2-10 mg 3-4 times daily. Use lowest effective dose, shortest duration.",
            side_effects: "Drowsiness, confusion, ataxia, respiratory depression, dependence, paradoxical agitation (elderly/children).",
            contraindications: "Severe respiratory insufficiency. Sleep apnea. Myasthenia gravis. Acute narrow-angle glaucoma.",
            interactions: "Opioids (respiratory depression — avoid). Alcohol. Other CNS depressants. CYP3A4 inhibitors (increased levels).",
            pregnancy_category: "Avoid. Risk of neonatal withdrawal, floppy infant syndrome. Cleft palate risk in first trimester.",
        },
        MedicationInfo {
            name: "Oral Rehydration Salts (ORS)",
            class: "Electrolyte Solution",
            uses: "Dehydration from diarrhea, vomiting, cholera. WHO-recommended for all ages. Cornerstone of diarrheal disease treatment.",
            dosage: "WHO formula: dissolve 1 packet in 1L clean water. Mild dehydration: 50-100 mL/kg over 4h. Maintenance: replace ongoing losses. Give small frequent sips if vomiting.",
            side_effects: "Vomiting if given too fast. Hypernatremia if prepared incorrectly (too concentrated).",
            contraindications: "Severe dehydration requiring IV fluids. Ileus. Persistent vomiting unresponsive to small sips.",
            interactions: "None significant. Can be given with zinc supplementation (recommended by WHO for children).",
            pregnancy_category: "Safe in pregnancy and breastfeeding.",
        },
    ]
}

pub fn run(name: &str, json: bool) {
    let meds = get_medications();
    let query = name.to_lowercase();

    let matches: Vec<&MedicationInfo> = meds
        .iter()
        .filter(|m| {
            m.name.to_lowercase().contains(&query)
                || m.class.to_lowercase().contains(&query)
                || m.uses.to_lowercase().contains(&query)
        })
        .collect();

    if matches.is_empty() {
        // Try fuzzy: search individual words
        let word_matches: Vec<&MedicationInfo> = meds
            .iter()
            .filter(|m| {
                query.split_whitespace().any(|w| {
                    w.len() >= 3
                        && (m.name.to_lowercase().contains(w)
                            || m.class.to_lowercase().contains(w))
                })
            })
            .collect();

        if word_matches.is_empty() {
            if json {
                println!("[]");
            } else {
                println!(
                    "{}",
                    format!("No medication found matching '{}'.", name).yellow()
                );
                println!("\n{}", "Available medications:".bright_cyan());
                for m in &meds {
                    println!("  • {}", m.name);
                }
            }
            return;
        }

        if json {
            println!("{}", serde_json::to_string_pretty(&word_matches).unwrap());
        } else {
            for m in &word_matches {
                print_medication(m);
            }
        }
        return;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&matches).unwrap());
    } else {
        for m in &matches {
            print_medication(m);
        }
    }
}

pub fn run_list(json: bool) {
    let meds = get_medications();
    if json {
        let names: Vec<&str> = meds.iter().map(|m| m.name).collect();
        println!("{}", serde_json::to_string_pretty(&names).unwrap());
    } else {
        println!("{}", "💊 Medication Reference".bright_cyan().bold());
        println!("{}", "═".repeat(50).bright_cyan());
        println!();
        for m in &meds {
            println!(
                "  {} {} — {}",
                "•".bright_cyan(),
                m.name.bold(),
                m.class.dimmed()
            );
        }
        println!();
        println!(
            "{}",
            "Use 'openhealth medication <name>' for detailed info.".dimmed()
        );
    }
}

fn print_medication(m: &MedicationInfo) {
    println!();
    println!("💊 {}", m.name.bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    println!("  {} {}", "Class:".bold(), m.class);
    println!();
    println!("  {}", "Uses:".bold());
    println!("  {}", m.uses);
    println!();
    println!("  {}", "Dosage:".bold());
    println!("  {}", m.dosage);
    println!();
    println!("  {}", "Side Effects:".bold());
    println!("  {}", m.side_effects);
    println!();
    println!("  {}", "Contraindications:".bold());
    println!("  {}", m.contraindications);
    println!();
    println!("  {}", "Drug Interactions:".bold());
    println!("  {}", m.interactions);
    println!();
    println!("  {}", "Pregnancy:".bold());
    println!("  {}", m.pregnancy_category);
    println!();
    println!(
        "  {}",
        "⚠️  Always consult a healthcare professional before taking any medication."
            .yellow()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medication_data_not_empty() {
        let meds = get_medications();
        assert!(meds.len() >= 10, "Should have at least 10 medications");
    }

    #[test]
    fn test_medication_fields_not_empty() {
        for m in get_medications() {
            assert!(!m.name.is_empty());
            assert!(!m.class.is_empty());
            assert!(!m.uses.is_empty());
            assert!(!m.dosage.is_empty());
            assert!(!m.side_effects.is_empty());
            assert!(!m.contraindications.is_empty());
        }
    }

    #[test]
    fn test_medication_search_paracetamol() {
        let meds = get_medications();
        let found = meds.iter().any(|m| m.name.to_lowercase().contains("paracetamol"));
        assert!(found, "Should find paracetamol");
    }

    #[test]
    fn test_medication_search_by_class() {
        let meds = get_medications();
        let nsaids: Vec<_> = meds.iter().filter(|m| m.class.to_lowercase().contains("nsaid")).collect();
        assert!(nsaids.len() >= 2, "Should find at least 2 NSAIDs");
    }
}
