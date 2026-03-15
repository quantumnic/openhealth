use colored::Colorize;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DoseEntry {
    medication: &'static str,
    indication: &'static str,
    dose_per_kg: f64,
    unit: &'static str,
    frequency: &'static str,
    max_single_dose: f64,
    max_daily_dose: f64,
    notes: &'static str,
}

fn get_dose_entries() -> Vec<DoseEntry> {
    vec![
        DoseEntry {
            medication: "Amoxicillin",
            indication: "Otitis media, pneumonia, UTI",
            dose_per_kg: 25.0,
            unit: "mg",
            frequency: "every 8 hours (TID)",
            max_single_dose: 500.0,
            max_daily_dose: 1500.0,
            notes: "High-dose: 40-45 mg/kg/dose BID for resistant organisms",
        },
        DoseEntry {
            medication: "Ibuprofen",
            indication: "Fever, pain, inflammation",
            dose_per_kg: 10.0,
            unit: "mg",
            frequency: "every 6-8 hours",
            max_single_dose: 400.0,
            max_daily_dose: 1200.0,
            notes: "Take with food. Avoid in renal impairment, GI bleeding",
        },
        DoseEntry {
            medication: "Acetaminophen (Paracetamol)",
            indication: "Fever, mild-moderate pain",
            dose_per_kg: 15.0,
            unit: "mg",
            frequency: "every 4-6 hours",
            max_single_dose: 1000.0,
            max_daily_dose: 4000.0,
            notes: "Do NOT exceed 4g/day in adults. Hepatotoxic in overdose",
        },
        DoseEntry {
            medication: "Azithromycin",
            indication: "Respiratory infections, traveler's diarrhea",
            dose_per_kg: 10.0,
            unit: "mg",
            frequency: "once daily (day 1), then 5mg/kg (days 2-5)",
            max_single_dose: 500.0,
            max_daily_dose: 500.0,
            notes: "Day 1 loading dose is double the maintenance dose",
        },
        DoseEntry {
            medication: "Metronidazole",
            indication: "Giardiasis, amoebiasis, anaerobic infections",
            dose_per_kg: 7.5,
            unit: "mg",
            frequency: "every 8 hours (TID)",
            max_single_dose: 500.0,
            max_daily_dose: 1500.0,
            notes: "Avoid alcohol during and 48h after treatment",
        },
        DoseEntry {
            medication: "Ciprofloxacin",
            indication: "UTI, GI infections, anthrax prophylaxis",
            dose_per_kg: 10.0,
            unit: "mg",
            frequency: "every 12 hours (BID)",
            max_single_dose: 750.0,
            max_daily_dose: 1500.0,
            notes: "Avoid in children <18 unless no alternative (tendon risk). Take with water.",
        },
        DoseEntry {
            medication: "Doxycycline",
            indication: "Typhus, Lyme, malaria prophylaxis, rickettsial infections",
            dose_per_kg: 2.0,
            unit: "mg",
            frequency: "every 12 hours (BID)",
            max_single_dose: 100.0,
            max_daily_dose: 200.0,
            notes: "Avoid in children <8 (tooth staining). Take upright with water.",
        },
        DoseEntry {
            medication: "ORS (Oral Rehydration Salts)",
            indication: "Dehydration from diarrhea/vomiting",
            dose_per_kg: 75.0,
            unit: "mL",
            frequency: "over 4 hours (moderate dehydration)",
            max_single_dose: 2000.0,
            max_daily_dose: 6000.0,
            notes: "Mild: 50mL/kg over 4h. Severe: IV Ringer's Lactate 100mL/kg",
        },
        DoseEntry {
            medication: "Prednisolone",
            indication: "Asthma exacerbation, croup, allergic reactions",
            dose_per_kg: 1.0,
            unit: "mg",
            frequency: "once daily",
            max_single_dose: 60.0,
            max_daily_dose: 60.0,
            notes: "Croup: single dose 0.6mg/kg. Taper if >5 days use.",
        },
        DoseEntry {
            medication: "Epinephrine (Adrenaline)",
            indication: "Anaphylaxis",
            dose_per_kg: 0.01,
            unit: "mg",
            frequency: "IM, repeat every 5-15 min if needed",
            max_single_dose: 0.5,
            max_daily_dose: 1.5,
            notes: "Use 1:1000 (1mg/mL) IM in anterolateral thigh. EpiPen Jr <30kg: 0.15mg; >30kg: 0.3mg",
        },
        DoseEntry {
            medication: "Ceftriaxone",
            indication: "Meningitis, sepsis, pneumonia, gonorrhea",
            dose_per_kg: 50.0,
            unit: "mg",
            frequency: "once daily IV/IM",
            max_single_dose: 2000.0,
            max_daily_dose: 4000.0,
            notes: "Meningitis: 100mg/kg/day divided BID. Avoid with IV calcium in neonates.",
        },
        DoseEntry {
            medication: "Albendazole",
            indication: "Helminth infections (roundworm, hookworm, whipworm)",
            dose_per_kg: 7.5,
            unit: "mg",
            frequency: "BID for 3 days (or single 400mg dose for intestinal worms)",
            max_single_dose: 400.0,
            max_daily_dose: 800.0,
            notes: "Single 400mg dose for soil-transmitted helminths. Take with fatty food for better absorption.",
        },
    ]
}

pub fn run(weight: f64, medication_filter: Option<&str>, json: bool) {
    if weight <= 0.0 || weight > 300.0 {
        eprintln!("Invalid weight: please provide a value between 1 and 300 kg.");
        return;
    }

    let entries = get_dose_entries();
    let filtered: Vec<&DoseEntry> = if let Some(filter) = medication_filter {
        let f = filter.to_lowercase();
        entries
            .iter()
            .filter(|e| {
                e.medication.to_lowercase().contains(&f)
                    || e.indication.to_lowercase().contains(&f)
            })
            .collect()
    } else {
        entries.iter().collect()
    };

    if filtered.is_empty() {
        println!("No matching medications found.");
        return;
    }

    if json {
        #[derive(Serialize)]
        struct CalcResult {
            medication: String,
            indication: String,
            weight_kg: f64,
            calculated_dose: f64,
            unit: String,
            frequency: String,
            max_single_dose: f64,
            max_daily_dose: f64,
            capped: bool,
            notes: String,
        }
        let results: Vec<CalcResult> = filtered
            .iter()
            .map(|e| {
                let raw_dose = e.dose_per_kg * weight;
                let capped = raw_dose > e.max_single_dose;
                let dose = raw_dose.min(e.max_single_dose);
                CalcResult {
                    medication: e.medication.to_string(),
                    indication: e.indication.to_string(),
                    weight_kg: weight,
                    calculated_dose: (dose * 10.0).round() / 10.0,
                    unit: e.unit.to_string(),
                    frequency: e.frequency.to_string(),
                    max_single_dose: e.max_single_dose,
                    max_daily_dose: e.max_daily_dose,
                    capped,
                    notes: e.notes.to_string(),
                }
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return;
    }

    println!(
        "\n{}",
        format!(
            "💊 Weight-Based Dosage Calculator — Patient: {:.1} kg",
            weight
        )
        .bold()
        .cyan()
    );
    println!("{}", "═".repeat(60));
    println!(
        "{}",
        "⚠️  For reference only — always verify with a healthcare provider"
            .yellow()
    );
    println!();

    for entry in &filtered {
        let raw_dose = entry.dose_per_kg * weight;
        let capped = raw_dose > entry.max_single_dose;
        let dose = raw_dose.min(entry.max_single_dose);

        println!("  {}", entry.medication.bold().green());
        println!("    Indication:  {}", entry.indication);
        println!(
            "    Dose:        {:.1}{} per dose  ({}mg/kg × {:.1}kg)",
            dose, entry.unit, entry.dose_per_kg, weight
        );
        if capped {
            println!(
                "    {}",
                format!("⚠ Capped at max single dose: {}{}", entry.max_single_dose, entry.unit)
                    .yellow()
            );
        }
        println!("    Frequency:   {}", entry.frequency);
        println!(
            "    Max daily:   {}{}",
            entry.max_daily_dose, entry.unit
        );
        println!("    Note:        {}", entry.notes.dimmed());
        println!();
    }

    println!("{}", "─".repeat(60));
    println!(
        "{}",
        "Disclaimer: This calculator provides general guidance.\nActual dosing depends on renal/hepatic function, age, \nindication severity, and drug interactions."
            .dimmed()
    );
}
