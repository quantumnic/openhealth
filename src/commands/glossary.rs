use colored::Colorize;

struct GlossaryEntry {
    term: &'static str,
    definition: &'static str,
    context: &'static str,
}

fn get_glossary() -> Vec<GlossaryEntry> {
    vec![
        GlossaryEntry { term: "Acute", definition: "Rapid onset, severe but short-lived", context: "A disease that starts suddenly and lasts a short time" },
        GlossaryEntry { term: "Chronic", definition: "Long-lasting, persistent over time", context: "A condition that persists for months or years" },
        GlossaryEntry { term: "Benign", definition: "Not harmful, not cancerous", context: "A growth or condition that is not life-threatening" },
        GlossaryEntry { term: "Malignant", definition: "Harmful, cancerous, tends to spread", context: "A tumor that invades nearby tissues and can metastasize" },
        GlossaryEntry { term: "Edema", definition: "Swelling caused by excess fluid in tissues", context: "Puffy ankles, swollen legs, fluid retention" },
        GlossaryEntry { term: "Tachycardia", definition: "Abnormally fast heart rate (>100 bpm)", context: "Heart beating too fast at rest" },
        GlossaryEntry { term: "Bradycardia", definition: "Abnormally slow heart rate (<60 bpm)", context: "Heart beating too slowly" },
        GlossaryEntry { term: "Hypotension", definition: "Low blood pressure (<90/60 mmHg)", context: "Dizziness, fainting, lightheadedness" },
        GlossaryEntry { term: "Hypertension", definition: "High blood pressure (>140/90 mmHg)", context: "The 'silent killer' — often no symptoms" },
        GlossaryEntry { term: "Dyspnea", definition: "Difficulty breathing, shortness of breath", context: "Feeling of not getting enough air" },
        GlossaryEntry { term: "Cyanosis", definition: "Bluish discoloration of skin from low oxygen", context: "Blue lips, fingers, or toes — sign of oxygen deprivation" },
        GlossaryEntry { term: "Pyrexia", definition: "Fever — body temperature above normal (>38°C / 100.4°F)", context: "The body's response to infection or inflammation" },
        GlossaryEntry { term: "Sepsis", definition: "Life-threatening organ dysfunction from infection", context: "When the body's response to infection damages its own organs" },
        GlossaryEntry { term: "Anemia", definition: "Low red blood cells or hemoglobin", context: "Causes fatigue, weakness, pale skin" },
        GlossaryEntry { term: "Jaundice", definition: "Yellowing of skin and eyes from high bilirubin", context: "Sign of liver problems, bile duct blockage, or excessive red cell breakdown" },
        GlossaryEntry { term: "Emesis", definition: "Vomiting — forceful expulsion of stomach contents", context: "Can indicate poisoning, infection, or obstruction" },
        GlossaryEntry { term: "Hemorrhage", definition: "Bleeding — loss of blood from blood vessels", context: "Can be internal (unseen) or external (visible)" },
        GlossaryEntry { term: "Necrosis", definition: "Death of body tissue", context: "Tissue dies due to lack of blood supply or infection" },
        GlossaryEntry { term: "Prognosis", definition: "Expected outcome or course of a disease", context: "The likely result of treatment — good or poor" },
        GlossaryEntry { term: "Prophylaxis", definition: "Preventive treatment", context: "Taking medicine to prevent disease, not to treat it" },
        GlossaryEntry { term: "Palliative", definition: "Treatment that relieves symptoms without curing", context: "Focus on comfort and quality of life" },
        GlossaryEntry { term: "Pathogen", definition: "Organism that causes disease (bacteria, virus, parasite, fungus)", context: "The germ responsible for an infection" },
        GlossaryEntry { term: "Etiology", definition: "The cause or origin of a disease", context: "What triggers or leads to a condition" },
        GlossaryEntry { term: "Subcutaneous", definition: "Under the skin", context: "Injections or lumps beneath the skin surface" },
        GlossaryEntry { term: "Intravenous (IV)", definition: "Into a vein — fastest way to deliver medication", context: "Needle or catheter directly into a blood vessel" },
        GlossaryEntry { term: "Intramuscular (IM)", definition: "Into a muscle — common for vaccines", context: "Injection into deltoid, thigh, or buttock muscle" },
        GlossaryEntry { term: "Afebrile", definition: "Without fever — normal body temperature", context: "Patient has no fever" },
        GlossaryEntry { term: "Diaphoresis", definition: "Excessive sweating", context: "Profuse sweating, often a sign of distress or shock" },
        GlossaryEntry { term: "Petechiae", definition: "Tiny red or purple spots from bleeding under skin", context: "Pinpoint spots that don't blanch with pressure — can indicate serious conditions" },
        GlossaryEntry { term: "Erythema", definition: "Redness of skin from increased blood flow", context: "Redness around a wound, rash, or inflammation" },
        GlossaryEntry { term: "Pruritus", definition: "Itching", context: "Desire to scratch — can indicate allergic reaction, skin disease, or liver problems" },
        GlossaryEntry { term: "Dysphagia", definition: "Difficulty swallowing", context: "Food or liquid feels stuck or painful when swallowing" },
        GlossaryEntry { term: "Syncope", definition: "Fainting — temporary loss of consciousness", context: "Caused by reduced blood flow to the brain" },
        GlossaryEntry { term: "Effusion", definition: "Abnormal fluid collection in a body cavity", context: "Pleural effusion (around lungs), pericardial effusion (around heart)" },
        GlossaryEntry { term: "Abscess", definition: "Pocket of pus from infection", context: "Painful, swollen, warm area that may need drainage" },
        GlossaryEntry { term: "Contraindication", definition: "Reason NOT to use a treatment — it could cause harm", context: "Aspirin is contraindicated in children with viral illness (Reye syndrome risk)" },
        GlossaryEntry { term: "Immunocompromised", definition: "Weakened immune system", context: "HIV, chemotherapy, organ transplant patients — higher infection risk" },
        GlossaryEntry { term: "Zoonosis", definition: "Disease transmitted from animals to humans", context: "Rabies, brucellosis, leptospirosis — animal-to-human infections" },
        GlossaryEntry { term: "Vector", definition: "Organism that transmits disease (mosquito, tick, fly)", context: "The carrier that spreads pathogens between hosts" },
        GlossaryEntry { term: "Endemic", definition: "Constantly present in a region or population", context: "Malaria is endemic in Sub-Saharan Africa" },
        GlossaryEntry { term: "Epidemic", definition: "Sudden increase of disease in a community", context: "More cases than normally expected in an area" },
        GlossaryEntry { term: "Pandemic", definition: "Epidemic spread across multiple countries or continents", context: "COVID-19 was declared a pandemic in 2020" },
        GlossaryEntry { term: "Comorbidity", definition: "Two or more diseases present at the same time", context: "Having diabetes AND hypertension simultaneously" },
        GlossaryEntry { term: "Idiopathic", definition: "Unknown cause", context: "When doctors cannot determine why a disease occurred" },
        GlossaryEntry { term: "Iatrogenic", definition: "Caused by medical treatment", context: "Side effects or complications from medical procedures" },
        GlossaryEntry { term: "ORS", definition: "Oral Rehydration Solution — WHO formula: 6 tsp sugar + ½ tsp salt per liter of clean water", context: "Life-saving treatment for dehydration from diarrhea" },
        GlossaryEntry { term: "RUTF", definition: "Ready-to-Use Therapeutic Food — energy-dense paste for severe malnutrition", context: "Plumpy'Nut and similar products used in famine response" },
        GlossaryEntry { term: "ACT", definition: "Artemisinin-based Combination Therapy — first-line malaria treatment", context: "WHO-recommended treatment for uncomplicated malaria" },
        GlossaryEntry { term: "MDA", definition: "Mass Drug Administration — distributing medicine to entire at-risk populations", context: "Used for neglected tropical diseases like lymphatic filariasis" },
        GlossaryEntry { term: "WASH", definition: "Water, Sanitation, and Hygiene — fundamental public health interventions", context: "Clean water + toilets + handwashing = fewer diseases" },
    ]
}

pub fn run(query: Option<&str>, as_json: bool) {
    let glossary = get_glossary();

    let filtered: Vec<&GlossaryEntry> = if let Some(q) = query {
        let q_lower = q.to_lowercase();
        glossary
            .iter()
            .filter(|e| {
                e.term.to_lowercase().contains(&q_lower)
                    || e.definition.to_lowercase().contains(&q_lower)
                    || e.context.to_lowercase().contains(&q_lower)
            })
            .collect()
    } else {
        glossary.iter().collect()
    };

    if as_json {
        let json_entries: Vec<serde_json::Value> = filtered
            .iter()
            .map(|e| {
                serde_json::json!({
                    "term": e.term,
                    "definition": e.definition,
                    "context": e.context,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_entries).unwrap());
        return;
    }

    if filtered.is_empty() {
        println!(
            "{} No glossary entries found for '{}'",
            "⚠".yellow(),
            query.unwrap_or("")
        );
        return;
    }

    println!(
        "\n{}",
        "📖 Medical Glossary".bold().cyan()
    );
    if let Some(q) = query {
        println!(
            "   Showing entries matching: {}\n",
            q.bold()
        );
    } else {
        println!(
            "   {} terms — search with: openhealth glossary \"term\"\n",
            filtered.len()
        );
    }

    for entry in &filtered {
        println!("  {} {}", "▸".cyan(), entry.term.bold());
        println!("    {}", entry.definition);
        println!("    {} {}", "↳".dimmed(), entry.context.dimmed());
        println!();
    }
}
