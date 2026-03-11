use rusqlite::Connection;

pub fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0))?;
    if count > 0 {
        return Ok(());
    }
    seed_all(conn)
}

pub fn seed_all(conn: &Connection) -> rusqlite::Result<()> {
    let diseases = get_disease_data();
    let tx = conn.unchecked_transaction()?;

    for d in &diseases {
        tx.execute(
            "INSERT OR IGNORE INTO diseases (name, description, severity, contagious, icd11_code) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![d.name, d.description, d.severity, d.contagious as i32, d.icd11_code],
        )?;
        let disease_id: i64 = tx.query_row(
            "SELECT id FROM diseases WHERE name = ?1",
            [d.name],
            |r| r.get(0),
        )?;

        for s in &d.symptoms {
            tx.execute(
                "INSERT OR IGNORE INTO symptoms (name) VALUES (?1)",
                [s.name],
            )?;
            let symptom_id: i64 = tx.query_row(
                "SELECT id FROM symptoms WHERE name = ?1",
                [s.name],
                |r| r.get(0),
            )?;
            tx.execute(
                "INSERT OR IGNORE INTO disease_symptoms (disease_id, symptom_id, weight, is_primary) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![disease_id, symptom_id, s.weight, s.is_primary as i32],
            )?;
        }

        tx.execute(
            "INSERT INTO treatments (disease_id, protocol, source, first_aid, prevention) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![disease_id, d.treatment, "WHO", d.first_aid, d.prevention],
        )?;
    }

    tx.execute(
        "INSERT OR REPLACE INTO metadata (key, value) VALUES ('seed_version', '1.0')",
        [],
    )?;
    tx.commit()?;
    Ok(())
}

struct SymptomEntry {
    name: &'static str,
    weight: f64,
    is_primary: bool,
}

struct DiseaseEntry {
    name: &'static str,
    description: &'static str,
    severity: &'static str,
    contagious: bool,
    icd11_code: &'static str,
    symptoms: Vec<SymptomEntry>,
    treatment: &'static str,
    first_aid: &'static str,
    prevention: &'static str,
}

fn s(name: &'static str, weight: f64, primary: bool) -> SymptomEntry {
    SymptomEntry { name, weight, is_primary: primary }
}

fn get_disease_data() -> Vec<DiseaseEntry> {
    vec![
        DiseaseEntry {
            name: "Malaria",
            description: "Parasitic infection transmitted by Anopheles mosquitoes. Life-threatening if untreated.",
            severity: "high", contagious: false, icd11_code: "1F40",
            symptoms: vec![s("fever",0.9,true), s("chills",0.8,true), s("sweating",0.7,false), s("headache",0.6,false), s("nausea",0.5,false), s("vomiting",0.5,false), s("muscle pain",0.4,false), s("fatigue",0.6,false)],
            treatment: "ACT (Artemisinin-based combination therapy). Severe: IV artesunate. Supportive care, fluids, antipyretics.",
            first_aid: "Keep patient cool, provide fluids, seek medical care urgently.",
            prevention: "Insecticide-treated bed nets, indoor spraying, antimalarial prophylaxis in endemic areas.",
        },
        DiseaseEntry {
            name: "Cholera",
            description: "Acute diarrheal disease caused by Vibrio cholerae. Can kill within hours if untreated.",
            severity: "high", contagious: true, icd11_code: "1A00",
            symptoms: vec![s("watery diarrhea",0.95,true), s("vomiting",0.7,true), s("dehydration",0.9,true), s("leg cramps",0.5,false), s("rapid heart rate",0.6,false)],
            treatment: "Oral rehydration salts (ORS) immediately. Severe: IV fluids (Ringer's lactate). Antibiotics: doxycycline or azithromycin.",
            first_aid: "Begin oral rehydration immediately: 1L water + 6 tsp sugar + 0.5 tsp salt.",
            prevention: "Clean water, sanitation, hand washing, oral cholera vaccine in endemic areas.",
        },
        DiseaseEntry {
            name: "Typhoid Fever",
            description: "Bacterial infection (Salmonella typhi) spread through contaminated food/water.",
            severity: "high", contagious: true, icd11_code: "1A07",
            symptoms: vec![s("sustained fever",0.9,true), s("headache",0.7,true), s("abdominal pain",0.7,false), s("constipation",0.5,false), s("diarrhea",0.4,false), s("rose spots",0.6,false), s("fatigue",0.6,false)],
            treatment: "Antibiotics: azithromycin, fluoroquinolones, or ceftriaxone. Fluids and rest.",
            first_aid: "Rest, fluids, fever management. Seek medical care.",
            prevention: "Safe food/water, hand hygiene, typhoid vaccination.",
        },
        DiseaseEntry {
            name: "Dengue Fever",
            description: "Viral infection transmitted by Aedes mosquitoes. Can progress to severe dengue (hemorrhagic).",
            severity: "medium", contagious: false, icd11_code: "1D20",
            symptoms: vec![s("high fever",0.9,true), s("severe headache",0.8,true), s("pain behind eyes",0.7,true), s("joint pain",0.8,false), s("muscle pain",0.7,false), s("rash",0.6,false), s("nausea",0.5,false), s("bleeding gums",0.4,false)],
            treatment: "No specific antiviral. Supportive: fluids, acetaminophen (NOT aspirin/ibuprofen). Monitor for severe dengue.",
            first_aid: "Rest, hydration, paracetamol only. Avoid NSAIDs. Watch for warning signs.",
            prevention: "Mosquito control, eliminate standing water, use repellent, wear long sleeves.",
        },
        DiseaseEntry {
            name: "Tuberculosis",
            description: "Bacterial infection (Mycobacterium tuberculosis) primarily affecting lungs.",
            severity: "high", contagious: true, icd11_code: "1B10",
            symptoms: vec![s("persistent cough",0.9,true), s("coughing blood",0.7,true), s("night sweats",0.8,true), s("weight loss",0.7,false), s("fever",0.6,false), s("fatigue",0.6,false), s("chest pain",0.5,false)],
            treatment: "DOTS: 6-month regimen — 2 months RIPE (rifampicin, isoniazid, pyrazinamide, ethambutol), then 4 months RI.",
            first_aid: "Isolate if possible, cover mouth when coughing, seek medical care.",
            prevention: "BCG vaccination, ventilation, early detection and treatment of cases.",
        },
        DiseaseEntry {
            name: "HIV/AIDS Symptoms",
            description: "Human immunodeficiency virus attacks the immune system. Early detection saves lives.",
            severity: "high", contagious: true, icd11_code: "1C60",
            symptoms: vec![s("persistent fever",0.6,false), s("rapid weight loss",0.7,true), s("chronic diarrhea",0.6,false), s("night sweats",0.7,true), s("swollen lymph nodes",0.7,true), s("fatigue",0.6,false), s("oral thrush",0.5,false), s("recurrent infections",0.7,false)],
            treatment: "Antiretroviral therapy (ART) — lifelong. Start immediately upon diagnosis. Regular viral load monitoring.",
            first_aid: "Get tested. If exposed: post-exposure prophylaxis (PEP) within 72 hours.",
            prevention: "Condoms, PrEP, avoid sharing needles, PMTCT for pregnant women.",
        },
        DiseaseEntry {
            name: "Pneumonia",
            description: "Infection of the lungs causing inflammation of air sacs. Leading killer of children under 5.",
            severity: "high", contagious: true, icd11_code: "CA40",
            symptoms: vec![s("cough",0.8,true), s("fever",0.8,true), s("difficulty breathing",0.9,true), s("chest pain",0.7,false), s("rapid breathing",0.8,false), s("fatigue",0.5,false), s("confusion",0.4,false)],
            treatment: "Bacterial: amoxicillin (first-line). Severe: IV antibiotics. Oxygen if SpO2 < 90%. Supportive care.",
            first_aid: "Keep upright, ensure airflow, seek medical care urgently for children.",
            prevention: "Pneumococcal vaccine, Hib vaccine, hand washing, adequate nutrition.",
        },
        DiseaseEntry {
            name: "Appendicitis",
            description: "Inflammation of the appendix. Surgical emergency if untreated (risk of rupture).",
            severity: "high", contagious: false, icd11_code: "DB10",
            symptoms: vec![s("right lower abdominal pain",0.95,true), s("nausea",0.7,true), s("vomiting",0.6,false), s("fever",0.5,false), s("loss of appetite",0.7,false), s("rebound tenderness",0.8,true)],
            treatment: "Appendectomy (surgical removal). Pre-op: IV antibiotics, NPO, pain management.",
            first_aid: "Do NOT eat or drink. Do NOT take laxatives. Go to hospital immediately.",
            prevention: "No known prevention. Early recognition prevents rupture.",
        },
        DiseaseEntry {
            name: "Heart Attack",
            description: "Myocardial infarction — blocked blood flow to the heart muscle. Minutes matter.",
            severity: "high", contagious: false, icd11_code: "BA41",
            symptoms: vec![s("chest pain",0.9,true), s("chest pressure",0.9,true), s("left arm pain",0.7,true), s("jaw pain",0.5,false), s("shortness of breath",0.7,false), s("cold sweat",0.6,false), s("nausea",0.5,false), s("dizziness",0.5,false)],
            treatment: "EMERGENCY: Call ambulance. Aspirin 300mg chewed. Hospital: PCI or thrombolysis within 90 min.",
            first_aid: "Call emergency services. Chew aspirin if available. Keep calm, sit upright. CPR if unresponsive.",
            prevention: "Exercise, healthy diet, no smoking, manage blood pressure and cholesterol.",
        },
        DiseaseEntry {
            name: "Stroke",
            description: "Brain blood supply interrupted. Use FAST: Face drooping, Arm weakness, Speech difficulty, Time to call.",
            severity: "high", contagious: false, icd11_code: "8B20",
            symptoms: vec![s("face drooping",0.9,true), s("arm weakness",0.9,true), s("speech difficulty",0.9,true), s("sudden headache",0.7,false), s("vision problems",0.6,false), s("confusion",0.7,false), s("dizziness",0.5,false)],
            treatment: "EMERGENCY: Hospital within 3 hours. Ischemic: tPA thrombolysis. Hemorrhagic: surgery may be needed.",
            first_aid: "FAST assessment. Call emergency immediately. Note time of onset. Do NOT give food/water.",
            prevention: "Control blood pressure, exercise, healthy diet, no smoking, limit alcohol.",
        },
        DiseaseEntry {
            name: "Diabetes Type 2",
            description: "Chronic condition affecting blood sugar regulation. Manageable with lifestyle changes.",
            severity: "medium", contagious: false, icd11_code: "5A11",
            symptoms: vec![s("increased thirst",0.8,true), s("frequent urination",0.8,true), s("unexplained weight loss",0.6,false), s("blurred vision",0.5,false), s("fatigue",0.6,false), s("slow wound healing",0.6,true), s("tingling in hands/feet",0.5,false)],
            treatment: "Lifestyle: diet, exercise, weight management. Medications: metformin first-line. Monitor blood glucose regularly.",
            first_aid: "Hypoglycemia: give sugar immediately. Hyperglycemia: hydrate, seek care if vomiting.",
            prevention: "Healthy weight, regular exercise, balanced diet low in processed sugar.",
        },
        DiseaseEntry {
            name: "Hypertension",
            description: "High blood pressure — 'the silent killer'. Often no symptoms until complications occur.",
            severity: "medium", contagious: false, icd11_code: "BA00",
            symptoms: vec![s("headache",0.4,false), s("dizziness",0.4,false), s("blurred vision",0.3,false), s("nosebleed",0.3,false), s("chest pain",0.3,false), s("shortness of breath",0.3,false)],
            treatment: "Lifestyle changes first. Medications: ACE inhibitors, ARBs, calcium channel blockers, diuretics. Regular monitoring.",
            first_aid: "Hypertensive crisis (>180/120): seek emergency care. Stay calm, lie down.",
            prevention: "Reduce salt, exercise, maintain healthy weight, limit alcohol, manage stress.",
        },
        DiseaseEntry {
            name: "Malnutrition",
            description: "Deficiency of essential nutrients. Affects growth, immunity, and cognitive development.",
            severity: "high", contagious: false, icd11_code: "5B70",
            symptoms: vec![s("weight loss",0.9,true), s("fatigue",0.8,true), s("muscle wasting",0.8,true), s("swollen belly",0.6,false), s("hair loss",0.5,false), s("dry skin",0.4,false), s("poor wound healing",0.5,false), s("irritability",0.4,false)],
            treatment: "Severe: therapeutic feeding (F-75 then F-100 formula, RUTF). Micronutrient supplementation. Treat underlying infections.",
            first_aid: "Small frequent meals, oral rehydration, seek medical care for children.",
            prevention: "Adequate nutrition, breastfeeding, food fortification, vitamin A supplementation.",
        },
        DiseaseEntry {
            name: "Common Cold",
            description: "Viral upper respiratory infection. Self-limiting, usually resolves in 7-10 days.",
            severity: "low", contagious: true, icd11_code: "CA02",
            symptoms: vec![s("runny nose",0.9,true), s("sneezing",0.8,true), s("sore throat",0.7,true), s("cough",0.6,false), s("mild headache",0.4,false), s("mild fever",0.3,false), s("fatigue",0.3,false)],
            treatment: "Rest, fluids, symptomatic relief. No antibiotics needed. Honey for cough (>1 year old).",
            first_aid: "Rest, warm fluids, saline nasal drops.",
            prevention: "Hand washing, avoid touching face, avoid close contact with sick people.",
        },
        DiseaseEntry {
            name: "Influenza",
            description: "Seasonal viral respiratory infection. More severe than common cold, can be fatal in vulnerable groups.",
            severity: "medium", contagious: true, icd11_code: "1E30",
            symptoms: vec![s("high fever",0.9,true), s("body aches",0.8,true), s("headache",0.7,false), s("fatigue",0.8,true), s("cough",0.7,false), s("sore throat",0.5,false), s("chills",0.7,false)],
            treatment: "Antivirals: oseltamivir within 48h of onset. Rest, fluids, antipyretics. Watch for complications.",
            first_aid: "Rest, fluids, isolate from vulnerable people.",
            prevention: "Annual flu vaccine, hand hygiene, respiratory etiquette.",
        },
        DiseaseEntry {
            name: "Diarrheal Disease",
            description: "Abnormally loose/watery stools. Leading cause of child mortality in developing countries.",
            severity: "medium", contagious: true, icd11_code: "ME05",
            symptoms: vec![s("watery diarrhea",0.9,true), s("abdominal cramps",0.7,true), s("nausea",0.6,false), s("vomiting",0.5,false), s("dehydration",0.8,true), s("fever",0.4,false)],
            treatment: "ORS (oral rehydration salts) — WHO formula. Zinc supplementation for children. Continue feeding. Antibiotics only if bloody diarrhea.",
            first_aid: "ORS: 1L clean water + 6 tsp sugar + 0.5 tsp salt. Give frequently in small sips.",
            prevention: "Clean water, hand washing, safe food preparation, rotavirus vaccine.",
        },
        DiseaseEntry {
            name: "Dehydration",
            description: "Body lacks sufficient water. Can be rapidly fatal in children and elderly.",
            severity: "high", contagious: false, icd11_code: "5C70",
            symptoms: vec![s("dry mouth",0.8,true), s("decreased urination",0.8,true), s("dark urine",0.7,true), s("dizziness",0.6,false), s("rapid heartbeat",0.6,false), s("sunken eyes",0.7,false), s("skin turgor loss",0.7,false), s("confusion",0.5,false)],
            treatment: "Mild/moderate: ORS. Severe: IV normal saline or Ringer's lactate. Monitor urine output.",
            first_aid: "Small frequent sips of ORS or clean water with salt and sugar.",
            prevention: "Adequate fluid intake, early ORS use during diarrhea/vomiting.",
        },
        DiseaseEntry {
            name: "Wound Infection",
            description: "Bacterial infection of a wound. Can progress to sepsis if untreated.",
            severity: "medium", contagious: false, icd11_code: "NE80",
            symptoms: vec![s("wound redness",0.8,true), s("wound swelling",0.8,true), s("pus discharge",0.9,true), s("increased pain",0.7,false), s("warmth around wound",0.6,false), s("fever",0.5,false), s("red streaks from wound",0.7,false)],
            treatment: "Clean wound, antiseptic, antibiotics if spreading. Tetanus prophylaxis if needed. Surgical debridement if severe.",
            first_aid: "Clean with clean water, apply antiseptic if available, cover with clean bandage.",
            prevention: "Clean wounds promptly, tetanus vaccination, proper wound care.",
        },
        DiseaseEntry {
            name: "Burns",
            description: "Tissue damage from heat, chemicals, electricity, or radiation. Severity by depth and area.",
            severity: "medium", contagious: false, icd11_code: "NE80.0",
            symptoms: vec![s("skin redness",0.8,true), s("blistering",0.7,true), s("pain",0.9,true), s("swelling",0.6,false), s("white or charred skin",0.5,false)],
            treatment: "Cool running water 20 min. Cover with clean dressing. Severe (>10% BSA or face/hands/genitals): hospital. IV fluids per Parkland formula.",
            first_aid: "Cool under running water for 20 minutes. Remove jewelry. Do NOT use ice, butter, or toothpaste. Cover loosely.",
            prevention: "Safe cooking practices, keep children away from fire/hot liquids, fire safety.",
        },
        DiseaseEntry {
            name: "Measles",
            description: "Highly contagious viral disease. Preventable by vaccination. Can cause encephalitis.",
            severity: "high", contagious: true, icd11_code: "1F03",
            symptoms: vec![s("high fever",0.9,true), s("rash",0.9,true), s("cough",0.7,false), s("runny nose",0.6,false), s("red eyes",0.7,true), s("koplik spots",0.8,false)],
            treatment: "Supportive: fluids, vitamin A supplementation (WHO). Manage complications. No specific antiviral.",
            first_aid: "Isolate, rest, fluids, vitamin A if available.",
            prevention: "MMR vaccination (2 doses). Herd immunity at 95% coverage.",
        },
        DiseaseEntry {
            name: "Tetanus",
            description: "Bacterial toxin (Clostridium tetani) causing severe muscle spasms. Enters through wounds.",
            severity: "high", contagious: false, icd11_code: "1C13",
            symptoms: vec![s("jaw stiffness",0.9,true), s("muscle spasms",0.9,true), s("difficulty swallowing",0.7,true), s("stiff neck",0.6,false), s("fever",0.4,false), s("sweating",0.4,false)],
            treatment: "Tetanus immunoglobulin, wound debridement, metronidazole, muscle relaxants, supportive ICU care.",
            first_aid: "Clean wound thoroughly. Seek immediate medical care.",
            prevention: "Tetanus vaccination (DPT series), booster every 10 years, clean wound care.",
        },
        DiseaseEntry {
            name: "Rabies",
            description: "Viral disease transmitted through animal bites. Nearly 100% fatal once symptoms appear.",
            severity: "high", contagious: false, icd11_code: "1C82",
            symptoms: vec![s("fever",0.6,false), s("headache",0.5,false), s("anxiety",0.6,true), s("confusion",0.7,true), s("hydrophobia",0.9,true), s("excessive salivation",0.8,false), s("paralysis",0.7,false)],
            treatment: "POST-BITE (before symptoms): Wash wound 15 min with soap. Rabies vaccine series + immunoglobulin. Once symptomatic: palliative care only.",
            first_aid: "Wash bite wound vigorously with soap and water for 15 minutes. Seek PEP immediately.",
            prevention: "Vaccinate dogs, avoid stray animals, pre-exposure vaccination for high-risk groups.",
        },
        DiseaseEntry {
            name: "Hepatitis B",
            description: "Viral liver infection. Can become chronic, leading to cirrhosis and liver cancer.",
            severity: "high", contagious: true, icd11_code: "1E50.1",
            symptoms: vec![s("jaundice",0.8,true), s("fatigue",0.7,true), s("abdominal pain",0.6,false), s("dark urine",0.7,true), s("nausea",0.6,false), s("vomiting",0.4,false), s("joint pain",0.4,false)],
            treatment: "Acute: supportive. Chronic: antiviral therapy (tenofovir/entecavir). Regular monitoring for liver damage.",
            first_aid: "Rest, avoid alcohol, seek medical testing.",
            prevention: "Hepatitis B vaccination (birth dose + series), safe injection practices, blood screening.",
        },
        DiseaseEntry {
            name: "Meningitis",
            description: "Inflammation of brain/spinal cord membranes. Bacterial form is a medical emergency.",
            severity: "high", contagious: true, icd11_code: "8A00",
            symptoms: vec![s("severe headache",0.9,true), s("stiff neck",0.9,true), s("high fever",0.8,true), s("sensitivity to light",0.7,false), s("nausea",0.5,false), s("vomiting",0.5,false), s("confusion",0.6,false), s("rash",0.5,false)],
            treatment: "EMERGENCY: IV antibiotics immediately (ceftriaxone). Dexamethasone before or with first antibiotic dose. ICU monitoring.",
            first_aid: "Seek emergency medical care immediately. Time is critical.",
            prevention: "Meningococcal vaccine, Hib vaccine, pneumococcal vaccine.",
        },
        DiseaseEntry {
            name: "Scabies",
            description: "Parasitic skin infestation by mites. Intensely itchy, spreads by skin contact.",
            severity: "low", contagious: true, icd11_code: "1G04",
            symptoms: vec![s("intense itching",0.9,true), s("rash",0.8,true), s("small blisters",0.6,false), s("burrow tracks on skin",0.8,true), s("itching worse at night",0.7,false)],
            treatment: "Permethrin 5% cream (apply neck down, wash off after 8-14h). Ivermectin for resistant cases. Treat all household contacts.",
            first_aid: "Keep skin clean, avoid scratching, wash bedding in hot water.",
            prevention: "Avoid prolonged skin contact with infected persons, treat all contacts simultaneously.",
        },
        DiseaseEntry {
            name: "Urinary Tract Infection",
            description: "Bacterial infection of the urinary system. Common, especially in women.",
            severity: "low", contagious: false, icd11_code: "GC08",
            symptoms: vec![s("painful urination",0.9,true), s("frequent urination",0.8,true), s("cloudy urine",0.7,false), s("blood in urine",0.6,false), s("lower abdominal pain",0.6,false), s("urgency",0.7,true)],
            treatment: "Antibiotics: nitrofurantoin or trimethoprim-sulfamethoxazole for uncomplicated. Fluids. If fever/flank pain: possible kidney infection — needs stronger treatment.",
            first_aid: "Drink plenty of water. Seek medical care for antibiotics.",
            prevention: "Adequate hydration, urinate after intercourse, wipe front to back.",
        },
        DiseaseEntry {
            name: "Asthma",
            description: "Chronic airway inflammation with reversible obstruction. Attacks can be life-threatening.",
            severity: "medium", contagious: false, icd11_code: "CA23",
            symptoms: vec![s("wheezing",0.9,true), s("shortness of breath",0.8,true), s("chest tightness",0.7,true), s("cough",0.7,false), s("difficulty breathing at night",0.6,false)],
            treatment: "Reliever: salbutamol inhaler. Controller: inhaled corticosteroids (beclomethasone). Severe attack: nebulized salbutamol + systemic steroids.",
            first_aid: "Sit upright. Use reliever inhaler (4 puffs, wait 4 min, repeat if needed). Call for help if no improvement.",
            prevention: "Avoid triggers (smoke, dust, allergens), regular controller medication, action plan.",
        },
        DiseaseEntry {
            name: "Anemia",
            description: "Low red blood cell count. Most commonly caused by iron deficiency.",
            severity: "medium", contagious: false, icd11_code: "3A00",
            symptoms: vec![s("fatigue",0.8,true), s("pale skin",0.7,true), s("weakness",0.7,true), s("dizziness",0.6,false), s("shortness of breath",0.5,false), s("cold hands and feet",0.4,false), s("rapid heartbeat",0.5,false)],
            treatment: "Iron deficiency: oral iron supplements (ferrous sulfate) + vitamin C. Severe: blood transfusion. Treat underlying cause.",
            first_aid: "Rest, iron-rich foods (meat, beans, dark leafy greens).",
            prevention: "Iron-rich diet, iron supplementation in pregnancy, deworming in endemic areas.",
        },
        DiseaseEntry {
            name: "Conjunctivitis",
            description: "Inflammation of the eye membrane (pink eye). Usually viral or bacterial.",
            severity: "low", contagious: true, icd11_code: "9A60",
            symptoms: vec![s("red eyes",0.9,true), s("eye discharge",0.8,true), s("itchy eyes",0.7,false), s("tearing",0.6,false), s("crusting of eyelids",0.6,false), s("gritty feeling in eye",0.5,false)],
            treatment: "Bacterial: antibiotic eye drops (chloramphenicol). Viral: self-limiting, supportive care. Allergic: antihistamine drops.",
            first_aid: "Clean discharge with warm water, avoid touching/rubbing eyes.",
            prevention: "Hand washing, avoid sharing towels, don't touch eyes with dirty hands.",
        },
        DiseaseEntry {
            name: "Gastroenteritis",
            description: "Inflammation of stomach and intestines. Usually viral (norovirus, rotavirus).",
            severity: "medium", contagious: true, icd11_code: "DA43",
            symptoms: vec![s("vomiting",0.9,true), s("diarrhea",0.9,true), s("nausea",0.8,false), s("abdominal cramps",0.7,false), s("fever",0.5,false), s("headache",0.3,false)],
            treatment: "ORS for hydration. Anti-emetics if needed. Zinc for children. Usually self-limiting (2-5 days).",
            first_aid: "Small frequent sips of ORS. BRAT diet when tolerated (bananas, rice, applesauce, toast).",
            prevention: "Hand washing, safe food handling, rotavirus vaccine for infants.",
        },
        DiseaseEntry {
            name: "Skin Abscess",
            description: "Localized collection of pus in the skin. Usually caused by Staphylococcus aureus.",
            severity: "low", contagious: false, icd11_code: "EE50",
            symptoms: vec![s("painful lump",0.9,true), s("swelling",0.8,true), s("redness",0.7,false), s("warmth",0.6,false), s("pus",0.8,true), s("fever",0.4,false)],
            treatment: "Incision and drainage. Antibiotics if cellulitis present or immunocompromised. Wound packing.",
            first_aid: "Warm compresses. Do NOT squeeze. Seek medical care for drainage.",
            prevention: "Good hygiene, clean wounds promptly, avoid sharing personal items.",
        },
        DiseaseEntry {
            name: "Otitis Media",
            description: "Middle ear infection. Extremely common in children. Pain is the primary symptom.",
            severity: "low", contagious: false, icd11_code: "AB10",
            symptoms: vec![s("ear pain",0.9,true), s("fever",0.6,false), s("hearing difficulty",0.6,true), s("ear discharge",0.5,false), s("irritability",0.5,false), s("pulling at ear",0.6,false)],
            treatment: "Most resolve spontaneously. Amoxicillin if bacterial/persistent. Pain relief: paracetamol/ibuprofen.",
            first_aid: "Pain relief, warm compress on ear. See doctor if fever persists >48h.",
            prevention: "Breastfeeding, pneumococcal vaccine, avoid secondhand smoke.",
        },
        DiseaseEntry {
            name: "Malaria (Cerebral)",
            description: "Severe complication of Plasmodium falciparum malaria with brain involvement. Medical emergency.",
            severity: "high", contagious: false, icd11_code: "1F40.1",
            symptoms: vec![s("high fever",0.9,true), s("seizures",0.8,true), s("confusion",0.8,true), s("coma",0.7,false), s("abnormal posturing",0.6,false), s("respiratory distress",0.6,false)],
            treatment: "EMERGENCY: IV artesunate immediately. Manage seizures (diazepam). Monitor glucose. ICU care.",
            first_aid: "Recovery position, protect airway, cool if febrile, transport to hospital immediately.",
            prevention: "Same as malaria: bed nets, prophylaxis, early treatment of uncomplicated malaria.",
        },
        DiseaseEntry {
            name: "Schistosomiasis",
            description: "Parasitic disease from freshwater snails. Affects 200+ million people worldwide.",
            severity: "medium", contagious: false, icd11_code: "1F80",
            symptoms: vec![s("blood in urine",0.8,true), s("abdominal pain",0.6,false), s("diarrhea",0.5,false), s("fatigue",0.5,false), s("rash",0.4,false), s("fever",0.4,false), s("liver enlargement",0.6,false)],
            treatment: "Praziquantel (single dose, 40mg/kg). Mass drug administration in endemic areas.",
            first_aid: "Seek medical testing if exposed to freshwater in endemic areas.",
            prevention: "Avoid swimming in freshwater in endemic areas, snail control, access to clean water.",
        },
        DiseaseEntry {
            name: "Trachoma",
            description: "Leading infectious cause of blindness. Caused by Chlamydia trachomatis.",
            severity: "medium", contagious: true, icd11_code: "9A70",
            symptoms: vec![s("eye irritation",0.7,true), s("eye discharge",0.7,true), s("swollen eyelids",0.6,false), s("sensitivity to light",0.5,false), s("vision problems",0.6,true), s("eyelash turning inward",0.7,false)],
            treatment: "SAFE strategy: Surgery for trichiasis, Antibiotics (azithromycin mass treatment), Face washing, Environmental improvement.",
            first_aid: "Keep face clean, avoid rubbing eyes.",
            prevention: "Face washing, improved sanitation, fly control, access to clean water.",
        },
        DiseaseEntry {
            name: "Hookworm Infection",
            description: "Intestinal parasite entering through bare feet. Causes anemia and malnutrition.",
            severity: "medium", contagious: false, icd11_code: "1F61",
            symptoms: vec![s("abdominal pain",0.6,true), s("diarrhea",0.5,false), s("fatigue",0.7,true), s("anemia",0.8,true), s("weight loss",0.5,false), s("itchy rash on feet",0.6,false)],
            treatment: "Albendazole 400mg single dose or mebendazole 500mg single dose. Iron supplementation for anemia.",
            first_aid: "Iron-rich foods, seek medical care for deworming.",
            prevention: "Wear shoes, improve sanitation, mass deworming programs in endemic areas.",
        },
        DiseaseEntry {
            name: "Yellow Fever",
            description: "Viral hemorrhagic disease transmitted by mosquitoes. Vaccine-preventable.",
            severity: "high", contagious: false, icd11_code: "1D40",
            symptoms: vec![s("fever",0.8,true), s("headache",0.7,false), s("jaundice",0.8,true), s("muscle pain",0.6,false), s("nausea",0.5,false), s("vomiting",0.5,false), s("bleeding",0.6,false), s("fatigue",0.5,false)],
            treatment: "No specific antiviral. Supportive care: fluids, fever management. Severe: ICU with organ support.",
            first_aid: "Rest, fluids, fever management. Seek medical care urgently.",
            prevention: "Yellow fever vaccination (single dose, lifelong protection), mosquito control.",
        },
        DiseaseEntry {
            name: "Leprosy",
            description: "Chronic bacterial infection (Mycobacterium leprae) affecting skin and nerves. Curable.",
            severity: "medium", contagious: true, icd11_code: "1B20",
            symptoms: vec![s("skin patches with numbness",0.9,true), s("nerve thickening",0.7,true), s("muscle weakness",0.6,false), s("numbness in hands/feet",0.7,true), s("skin nodules",0.5,false), s("eye problems",0.4,false)],
            treatment: "WHO multidrug therapy (MDT): rifampicin + dapsone (PB), add clofazimine (MB). Duration: 6-12 months.",
            first_aid: "Protect numb areas from injury. Seek medical care — leprosy is curable.",
            prevention: "Early detection and treatment, BCG vaccination provides partial protection.",
        },
        DiseaseEntry {
            name: "Epilepsy",
            description: "Neurological disorder causing recurrent seizures. Very common, treatable.",
            severity: "medium", contagious: false, icd11_code: "8A60",
            symptoms: vec![s("seizures",0.95,true), s("loss of consciousness",0.7,true), s("confusion",0.6,false), s("staring spells",0.6,false), s("muscle jerking",0.7,false), s("temporary confusion",0.5,false)],
            treatment: "Anti-seizure medication: carbamazepine, valproic acid, phenobarbital (resource-limited settings). Take daily, do not stop abruptly.",
            first_aid: "During seizure: clear area, turn on side, do NOT put anything in mouth, time the seizure. Call help if >5 min.",
            prevention: "Regular medication, adequate sleep, avoid known triggers, manage head injury risk.",
        },
        DiseaseEntry {
            name: "Snakebite Envenomation",
            description: "Venom injection from snake bite. A neglected tropical disease killing 100,000+ annually.",
            severity: "high", contagious: false, icd11_code: "NF04",
            symptoms: vec![s("bite wound",0.9,true), s("swelling",0.8,true), s("pain",0.8,false), s("bleeding",0.6,false), s("nausea",0.5,false), s("difficulty breathing",0.6,false), s("blurred vision",0.5,false), s("paralysis",0.5,false)],
            treatment: "Antivenom (polyvalent if species unknown). Keep calm, immobilize limb. Hospital: monitor for coagulopathy, neurotoxicity.",
            first_aid: "Keep still, immobilize bitten limb below heart level. Do NOT cut, suck, or tourniquet. Remove jewelry. Transport to hospital.",
            prevention: "Wear boots and long pants, use flashlight at night, sleep elevated off ground.",
        },
        DiseaseEntry {
            name: "Chickenpox",
            description: "Highly contagious viral infection (varicella-zoster). Usually mild in children, severe in adults.",
            severity: "low", contagious: true, icd11_code: "1E90",
            symptoms: vec![s("itchy rash",0.9,true), s("fever",0.6,false), s("fatigue",0.5,false), s("headache",0.4,false), s("loss of appetite",0.4,false), s("fluid-filled blisters",0.9,true)],
            treatment: "Supportive: calamine lotion, oatmeal baths, antihistamines for itch. Acyclovir for high-risk groups.",
            first_aid: "Keep cool, trim nails to prevent scratching, calamine lotion.",
            prevention: "Varicella vaccine (2 doses). Isolate infected persons.",
        },
        DiseaseEntry {
            name: "Pertussis",
            description: "Whooping cough — highly contagious bacterial respiratory disease. Dangerous for infants.",
            severity: "medium", contagious: true, icd11_code: "1C12",
            symptoms: vec![s("severe cough",0.9,true), s("whooping sound",0.8,true), s("vomiting after cough",0.7,false), s("runny nose",0.5,false), s("mild fever",0.4,false), s("exhaustion after cough",0.6,false)],
            treatment: "Azithromycin (first-line antibiotic). Most effective if started early. Supportive care, monitor infants for apnea.",
            first_aid: "Keep calm during coughing fits, small frequent meals, humidified air.",
            prevention: "DPT vaccination series, booster (Tdap) in pregnancy, cocooning strategy.",
        },
        DiseaseEntry {
            name: "Eczema",
            description: "Chronic inflammatory skin condition causing itchy, dry, cracked skin.",
            severity: "low", contagious: false, icd11_code: "EA80",
            symptoms: vec![s("itchy skin",0.9,true), s("dry skin",0.8,true), s("red patches",0.8,true), s("cracked skin",0.6,false), s("skin thickening",0.5,false), s("oozing",0.4,false)],
            treatment: "Emollients (moisturizers) liberally and frequently. Topical corticosteroids for flares. Avoid triggers.",
            first_aid: "Moisturize, cool compress for itch relief, avoid scratching.",
            prevention: "Regular moisturizing, avoid known triggers (soaps, allergens), gentle skin care.",
        },
        DiseaseEntry {
            name: "Typhus",
            description: "Rickettsial bacterial infection transmitted by lice, fleas, or mites.",
            severity: "high", contagious: false, icd11_code: "1C30",
            symptoms: vec![s("high fever",0.9,true), s("severe headache",0.8,true), s("rash",0.7,true), s("muscle pain",0.6,false), s("confusion",0.5,false), s("cough",0.3,false)],
            treatment: "Doxycycline (drug of choice). Start empirically if suspected. Usually rapid improvement within 48h.",
            first_aid: "Rest, fluids, seek medical care for antibiotic treatment.",
            prevention: "Louse/flea control, good hygiene, avoid overcrowding.",
        },
        DiseaseEntry {
            name: "Giardiasis",
            description: "Intestinal infection by Giardia parasite. Common waterborne disease worldwide.",
            severity: "low", contagious: true, icd11_code: "1A30",
            symptoms: vec![s("diarrhea",0.8,true), s("abdominal cramps",0.7,true), s("bloating",0.7,true), s("nausea",0.5,false), s("greasy stools",0.6,false), s("fatigue",0.4,false)],
            treatment: "Metronidazole or tinidazole. Adequate hydration.",
            first_aid: "Stay hydrated, ORS if needed.",
            prevention: "Drink clean/boiled water, hand washing, avoid swallowing water while swimming.",
        },
        DiseaseEntry {
            name: "Cellulitis",
            description: "Bacterial skin infection causing redness and swelling. Can spread rapidly.",
            severity: "medium", contagious: false, icd11_code: "EE00",
            symptoms: vec![s("skin redness",0.9,true), s("swelling",0.8,true), s("warmth",0.7,false), s("pain",0.8,true), s("fever",0.5,false), s("red streaks",0.6,false)],
            treatment: "Oral antibiotics: flucloxacillin or cefalexin. IV antibiotics if severe. Elevate affected area.",
            first_aid: "Mark the edge of redness with pen to monitor spread. Elevate limb. Seek medical care.",
            prevention: "Prompt wound care, moisturize dry/cracked skin, treat athlete's foot.",
        },
        DiseaseEntry {
            name: "Preeclampsia",
            description: "Pregnancy complication with high blood pressure and organ damage. Can be fatal.",
            severity: "high", contagious: false, icd11_code: "JA24",
            symptoms: vec![s("high blood pressure",0.9,true), s("headache",0.7,false), s("swelling of face/hands",0.7,true), s("vision problems",0.6,false), s("upper abdominal pain",0.6,true), s("nausea",0.5,false), s("rapid weight gain",0.5,false)],
            treatment: "Delivery is the cure. Magnesium sulfate for seizure prevention. Antihypertensives. Monitor closely.",
            first_aid: "Seek immediate medical care. Lie on left side. Monitor for seizures.",
            prevention: "Regular prenatal care, blood pressure monitoring, low-dose aspirin if high-risk.",
        },
        DiseaseEntry {
            name: "Neonatal Sepsis",
            description: "Life-threatening infection in newborns (<28 days). Leading cause of neonatal death.",
            severity: "high", contagious: false, icd11_code: "KA60",
            symptoms: vec![s("fever",0.7,true), s("poor feeding",0.8,true), s("lethargy",0.8,true), s("rapid breathing",0.7,false), s("jaundice",0.5,false), s("hypothermia",0.6,false), s("irritability",0.5,false)],
            treatment: "EMERGENCY: IV antibiotics immediately (ampicillin + gentamicin). Supportive care, monitor glucose, temperature.",
            first_aid: "Keep baby warm, try to feed, seek emergency medical care immediately.",
            prevention: "Clean delivery, hand hygiene, exclusive breastfeeding, cord care.",
        },
        DiseaseEntry {
            name: "Fracture",
            description: "Broken bone. Can be open (bone visible) or closed. Common from trauma.",
            severity: "medium", contagious: false, icd11_code: "NA70",
            symptoms: vec![s("severe pain",0.9,true), s("swelling",0.8,true), s("deformity",0.7,true), s("inability to move",0.8,false), s("bruising",0.6,false), s("crepitus",0.5,false)],
            treatment: "Immobilize, pain management, X-ray. Closed reduction or surgical fixation depending on type and location.",
            first_aid: "Immobilize above and below fracture. Do NOT try to straighten. Ice for swelling. Splint with available materials.",
            prevention: "Calcium-rich diet, fall prevention, protective equipment during activities.",
        },
        DiseaseEntry {
            name: "Postpartum Hemorrhage",
            description: "Excessive bleeding after childbirth. Leading cause of maternal death worldwide.",
            severity: "high", contagious: false, icd11_code: "JA43",
            symptoms: vec![s("heavy vaginal bleeding",0.95,true), s("rapid heartbeat",0.7,true), s("low blood pressure",0.8,true), s("dizziness",0.6,false), s("pale skin",0.6,false), s("weakness",0.5,false)],
            treatment: "Uterine massage, oxytocin 10 IU IM/IV. Misoprostol if oxytocin unavailable. IV fluids, blood transfusion if severe. Surgical intervention if medical management fails.",
            first_aid: "Massage uterus firmly through abdomen. Keep legs elevated. Get to hospital immediately.",
            prevention: "Active management of third stage of labor, skilled birth attendant, oxytocin after delivery.",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_seed_creates_diseases() {
        let conn = db::init_memory_database().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0)).unwrap();
        assert!(count >= 50, "Expected at least 50 diseases, got {count}");
    }

    #[test]
    fn test_seed_creates_symptoms() {
        let conn = db::init_memory_database().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM symptoms", [], |r| r.get(0)).unwrap();
        assert!(count >= 30, "Expected at least 30 symptoms, got {count}");
    }

    #[test]
    fn test_seed_creates_treatments() {
        let conn = db::init_memory_database().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM treatments", [], |r| r.get(0)).unwrap();
        assert!(count >= 50, "Expected at least 50 treatments, got {count}");
    }

    #[test]
    fn test_seed_idempotent() {
        let conn = db::init_memory_database().unwrap();
        let c1: i64 = conn.query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0)).unwrap();
        seed_if_empty(&conn).unwrap();
        let c2: i64 = conn.query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0)).unwrap();
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_disease_has_symptoms() {
        let conn = db::init_memory_database().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM disease_symptoms ds JOIN diseases d ON d.id = ds.disease_id WHERE d.name = 'Malaria'",
            [], |r| r.get(0),
        ).unwrap();
        assert!(count >= 5, "Malaria should have at least 5 symptoms");
    }

    #[test]
    fn test_metadata_seed_version() {
        let conn = db::init_memory_database().unwrap();
        let ver: String = conn.query_row(
            "SELECT value FROM metadata WHERE key = 'seed_version'",
            [], |r| r.get(0),
        ).unwrap();
        assert_eq!(ver, "1.0");
    }
}
