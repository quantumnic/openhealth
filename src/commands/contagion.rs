use colored::*;
use rusqlite::Connection;

/// Transmission route and precaution reference for contagious diseases.
pub fn run(conn: &Connection, query: Option<&str>, json: bool) {
    let routes = get_transmission_data();

    let filtered: Vec<&TransmissionInfo> = match query {
        Some(q) => {
            let q_lower = q.to_lowercase();
            routes
                .iter()
                .filter(|r| {
                    r.disease.to_lowercase().contains(&q_lower)
                        || r.route.to_lowercase().contains(&q_lower)
                })
                .collect()
        }
        None => {
            // Show only diseases that exist in the database
            let all_diseases: Vec<String> = conn
                .prepare("SELECT name FROM diseases WHERE contagious = 1")
                .unwrap()
                .query_map([], |row| row.get::<_, String>(0))
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();
            routes
                .iter()
                .filter(|r| {
                    all_diseases
                        .iter()
                        .any(|d| d.to_lowercase() == r.disease.to_lowercase())
                })
                .collect()
        }
    };

    if json {
        let items: Vec<serde_json::Value> = filtered
            .iter()
            .map(|r| {
                serde_json::json!({
                    "disease": r.disease,
                    "route": r.route,
                    "incubation": r.incubation,
                    "infectious_period": r.infectious_period,
                    "precautions": r.precautions,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&items).unwrap());
        return;
    }

    if filtered.is_empty() {
        println!(
            "{}",
            "No transmission data found for that query.".yellow()
        );
        println!("Try: openhealth contagion malaria");
        return;
    }

    println!(
        "{}",
        format!("🦠 Transmission & Precautions — {} result(s)", filtered.len())
            .bold()
            .cyan()
    );
    println!();

    for info in &filtered {
        println!("  {} {}", "Disease:".bold(), info.disease.bold().white());
        println!(
            "  {} {}",
            "Route:".bold(),
            info.route.yellow()
        );
        println!(
            "  {} {}",
            "Incubation:".bold(),
            info.incubation
        );
        println!(
            "  {} {}",
            "Infectious period:".bold(),
            info.infectious_period
        );
        println!("  {} {}", "Precautions:".bold(), info.precautions.green());
        println!();
    }
}

struct TransmissionInfo {
    disease: &'static str,
    route: &'static str,
    incubation: &'static str,
    infectious_period: &'static str,
    precautions: &'static str,
}

fn get_transmission_data() -> Vec<TransmissionInfo> {
    vec![
        TransmissionInfo { disease: "Malaria", route: "Vector-borne (Anopheles mosquito bite)", incubation: "7-30 days", infectious_period: "Not directly contagious person-to-person", precautions: "Bed nets, insect repellent, antimalarial prophylaxis" },
        TransmissionInfo { disease: "COVID-19", route: "Airborne / respiratory droplets / fomites", incubation: "1-14 days (median 5)", infectious_period: "2 days before to 10 days after symptom onset", precautions: "Masks, ventilation, hand hygiene, vaccination, isolation" },
        TransmissionInfo { disease: "Influenza", route: "Respiratory droplets / fomites", incubation: "1-4 days", infectious_period: "1 day before to 5-7 days after symptom onset", precautions: "Vaccination, hand hygiene, respiratory etiquette, isolation when symptomatic" },
        TransmissionInfo { disease: "Tuberculosis", route: "Airborne (aerosolized droplet nuclei)", incubation: "2-12 weeks (may remain latent years)", infectious_period: "While smear-positive; typically first 2 weeks of treatment", precautions: "N95 respirators, negative-pressure rooms, DOT therapy, BCG vaccination" },
        TransmissionInfo { disease: "Cholera", route: "Fecal-oral (contaminated water/food)", incubation: "Hours to 5 days", infectious_period: "While symptomatic and up to 2 weeks after", precautions: "Safe water, sanitation, oral cholera vaccine, hand hygiene" },
        TransmissionInfo { disease: "Measles", route: "Airborne (highly contagious, R0 12-18)", incubation: "7-21 days", infectious_period: "4 days before to 4 days after rash onset", precautions: "MMR vaccination, airborne isolation, post-exposure prophylaxis" },
        TransmissionInfo { disease: "Chickenpox (Varicella)", route: "Airborne / direct contact with vesicle fluid", incubation: "10-21 days", infectious_period: "1-2 days before rash until all lesions crusted", precautions: "Varicella vaccine, airborne + contact isolation" },
        TransmissionInfo { disease: "Mumps", route: "Respiratory droplets / direct contact with saliva", incubation: "12-25 days", infectious_period: "2 days before to 5 days after parotid swelling", precautions: "MMR vaccination, droplet isolation" },
        TransmissionInfo { disease: "Pertussis (Whooping Cough)", route: "Respiratory droplets", incubation: "5-21 days", infectious_period: "Catarrhal phase through 3 weeks (or 5 days of antibiotics)", precautions: "DTaP/Tdap vaccination, post-exposure prophylaxis (azithromycin), droplet isolation" },
        TransmissionInfo { disease: "Scarlet Fever", route: "Respiratory droplets / direct contact", incubation: "1-7 days", infectious_period: "While symptomatic until 24h of antibiotics", precautions: "Hand hygiene, avoid sharing utensils, prompt antibiotic treatment" },
        TransmissionInfo { disease: "Diphtheria", route: "Respiratory droplets / skin contact (cutaneous)", incubation: "2-5 days", infectious_period: "Until 2-4 negative cultures post-antibiotics", precautions: "DPT/Td vaccination, droplet isolation, antitoxin + antibiotics" },
        TransmissionInfo { disease: "Dengue Fever", route: "Vector-borne (Aedes mosquito)", incubation: "4-10 days", infectious_period: "Not directly person-to-person; viremic phase 2 days before to 5 days after fever", precautions: "Mosquito control, repellent, Dengvaxia vaccine (seropositive)" },
        TransmissionInfo { disease: "Ebola Virus Disease", route: "Direct contact with blood/body fluids of infected", incubation: "2-21 days", infectious_period: "While symptomatic; virus persists in semen months", precautions: "Full PPE, strict contact isolation, safe burial practices, rVSV-ZEBOV vaccine" },
        TransmissionInfo { disease: "Rabies", route: "Bite / scratch from infected animal (saliva to wound/mucosa)", incubation: "1-3 months (range: days to years)", infectious_period: "Not person-to-person (rare exceptions)", precautions: "Pre/post-exposure vaccination, wound washing, avoid stray animals" },
        TransmissionInfo { disease: "Tetanus", route: "Environmental (spore entry through wound; NOT contagious)", incubation: "3-21 days", infectious_period: "Not contagious", precautions: "DPT/Td vaccination, wound care, tetanus booster every 10 years" },
        TransmissionInfo { disease: "Hepatitis A", route: "Fecal-oral (contaminated food/water)", incubation: "15-50 days", infectious_period: "2 weeks before to 1 week after jaundice onset", precautions: "Hepatitis A vaccine, hand hygiene, safe food/water" },
        TransmissionInfo { disease: "Hepatitis E", route: "Fecal-oral (contaminated water)", incubation: "15-64 days", infectious_period: "1 week before to 2 weeks after jaundice onset", precautions: "Safe water, sanitation, hand hygiene" },
        TransmissionInfo { disease: "Norovirus Gastroenteritis", route: "Fecal-oral / vomit aerosol / fomites (highly contagious)", incubation: "12-48 hours", infectious_period: "While symptomatic to 48h after recovery", precautions: "Hand washing (alcohol gel less effective), surface disinfection, isolation" },
        TransmissionInfo { disease: "Rotavirus Gastroenteritis", route: "Fecal-oral", incubation: "1-3 days", infectious_period: "Before symptoms to 10 days after", precautions: "Rotavirus vaccine, hand hygiene, surface disinfection" },
        TransmissionInfo { disease: "Hand, Foot, and Mouth Disease", route: "Fecal-oral / respiratory droplets / vesicle fluid", incubation: "3-6 days", infectious_period: "First week of illness (virus shed in stool for weeks)", precautions: "Hand hygiene, avoid sharing utensils, disinfect surfaces" },
        TransmissionInfo { disease: "Plague", route: "Flea bite (bubonic) / respiratory droplets (pneumonic)", incubation: "1-7 days", infectious_period: "Pneumonic: while symptomatic", precautions: "Flea control, droplet isolation (pneumonic), post-exposure prophylaxis" },
        TransmissionInfo { disease: "Mpox (Monkeypox)", route: "Direct contact with lesions / respiratory droplets / fomites", incubation: "5-21 days", infectious_period: "From prodrome until all lesions crusted and fallen off", precautions: "Isolation, contact + droplet precautions, MVA-BN vaccine (JYNNEOS)" },
        TransmissionInfo { disease: "Mycoplasma Pneumonia", route: "Respiratory droplets (close contact)", incubation: "1-4 weeks", infectious_period: "While symptomatic (weeks if untreated)", precautions: "Hand hygiene, respiratory etiquette, avoid crowded dormitories when symptomatic" },
        TransmissionInfo { disease: "Coxsackievirus Myocarditis", route: "Fecal-oral / respiratory droplets", incubation: "3-5 days (enterovirus)", infectious_period: "First 1-2 weeks; virus shed in stool for weeks", precautions: "Hand hygiene, avoid sharing food/utensils, disinfect surfaces" },
        TransmissionInfo { disease: "Tonsillitis", route: "Respiratory droplets / direct contact", incubation: "2-5 days", infectious_period: "While symptomatic until 24h of antibiotics (strep)", precautions: "Hand hygiene, respiratory etiquette, avoid sharing utensils" },
        TransmissionInfo { disease: "Croup (Laryngotracheobronchitis)", route: "Respiratory droplets / fomites", incubation: "2-6 days", infectious_period: "First few days of illness", precautions: "Hand hygiene, respiratory etiquette, avoid exposure to symptomatic children" },
        TransmissionInfo { disease: "Impetigo", route: "Direct skin contact / fomites (towels, clothing)", incubation: "1-10 days", infectious_period: "Until 24h after starting antibiotics or sores heal", precautions: "Hand hygiene, avoid touching sores, do not share personal items, cover lesions" },
    ]
}
