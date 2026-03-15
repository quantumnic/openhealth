use colored::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct IncubationInfo {
    disease: &'static str,
    min_days: f64,
    max_days: f64,
    typical: &'static str,
    notes: &'static str,
}

fn get_incubation_data() -> Vec<IncubationInfo> {
    vec![
        IncubationInfo { disease: "Malaria", min_days: 7.0, max_days: 30.0, typical: "10-15 days", notes: "P. vivax/ovale can relapse months later from liver hypnozoites" },
        IncubationInfo { disease: "COVID-19", min_days: 1.0, max_days: 14.0, typical: "3-5 days", notes: "Omicron variants tend toward shorter incubation (~3 days)" },
        IncubationInfo { disease: "Influenza", min_days: 1.0, max_days: 4.0, typical: "2 days", notes: "Contagious 1 day before symptoms" },
        IncubationInfo { disease: "Common Cold", min_days: 1.0, max_days: 3.0, typical: "1-2 days", notes: "Rhinovirus most common cause" },
        IncubationInfo { disease: "Chickenpox (Varicella)", min_days: 10.0, max_days: 21.0, typical: "14-16 days", notes: "Contagious 1-2 days before rash until all lesions crusted" },
        IncubationInfo { disease: "Measles", min_days: 7.0, max_days: 21.0, typical: "10-14 days", notes: "Highly contagious 4 days before to 4 days after rash" },
        IncubationInfo { disease: "Mumps", min_days: 12.0, max_days: 25.0, typical: "16-18 days", notes: "Up to 30% of infections are asymptomatic" },
        IncubationInfo { disease: "Rubella", min_days: 14.0, max_days: 21.0, typical: "14-17 days", notes: "Contagious 1 week before to 1 week after rash" },
        IncubationInfo { disease: "Hepatitis A", min_days: 15.0, max_days: 50.0, typical: "28-30 days", notes: "Most infectious 2 weeks before symptom onset" },
        IncubationInfo { disease: "Hepatitis B", min_days: 45.0, max_days: 180.0, typical: "60-90 days", notes: "Can be asymptomatic; chronic carrier state possible" },
        IncubationInfo { disease: "Hepatitis E", min_days: 15.0, max_days: 64.0, typical: "40 days", notes: "Dangerous in pregnancy (high mortality in 3rd trimester)" },
        IncubationInfo { disease: "Tuberculosis", min_days: 14.0, max_days: 365.0, typical: "2-12 weeks", notes: "Latent TB can reactivate years later" },
        IncubationInfo { disease: "Typhoid Fever", min_days: 6.0, max_days: 30.0, typical: "8-14 days", notes: "Depends on inoculum size; chronic carriers exist" },
        IncubationInfo { disease: "Cholera", min_days: 0.08, max_days: 5.0, typical: "1-3 days", notes: "Can be as short as 2 hours with large inoculum" },
        IncubationInfo { disease: "Dengue Fever", min_days: 3.0, max_days: 14.0, typical: "4-7 days", notes: "Second infection with different serotype increases DHF risk" },
        IncubationInfo { disease: "Ebola Virus Disease", min_days: 2.0, max_days: 21.0, typical: "8-10 days", notes: "Not contagious until symptomatic" },
        IncubationInfo { disease: "Rabies", min_days: 20.0, max_days: 365.0, typical: "1-3 months", notes: "Can be years in rare cases; once symptomatic, nearly 100% fatal" },
        IncubationInfo { disease: "Tetanus", min_days: 3.0, max_days: 21.0, typical: "7-10 days", notes: "Shorter incubation = worse prognosis" },
        IncubationInfo { disease: "Pertussis (Whooping Cough)", min_days: 4.0, max_days: 21.0, typical: "7-10 days", notes: "Catarrhal stage most contagious; cough can last 100 days" },
        IncubationInfo { disease: "Diphtheria", min_days: 2.0, max_days: 5.0, typical: "2-5 days", notes: "Carriers can transmit without symptoms" },
        IncubationInfo { disease: "Plague", min_days: 1.0, max_days: 7.0, typical: "2-6 days", notes: "Pneumonic plague: 1-3 days; most rapidly fatal form" },
        IncubationInfo { disease: "Anthrax", min_days: 1.0, max_days: 60.0, typical: "1-7 days (cutaneous)", notes: "Inhalational: up to 60 days; spores can persist in lungs" },
        IncubationInfo { disease: "Lyme Disease", min_days: 3.0, max_days: 30.0, typical: "7-14 days", notes: "Erythema migrans appears at bite site; tick must feed >36h" },
        IncubationInfo { disease: "Rocky Mountain Spotted Fever", min_days: 2.0, max_days: 14.0, typical: "5-7 days", notes: "Rash typically appears day 2-5; absence doesn't exclude diagnosis" },
        IncubationInfo { disease: "Chikungunya", min_days: 1.0, max_days: 12.0, typical: "3-7 days", notes: "Joint pain can persist for months to years" },
        IncubationInfo { disease: "Zika Virus", min_days: 3.0, max_days: 14.0, typical: "3-7 days", notes: "80% asymptomatic; teratogenic risk (microcephaly)" },
        IncubationInfo { disease: "Mpox (Monkeypox)", min_days: 5.0, max_days: 21.0, typical: "7-14 days", notes: "Rash evolves through stages over 2-4 weeks" },
        IncubationInfo { disease: "Norovirus Gastroenteritis", min_days: 0.5, max_days: 2.0, typical: "12-48 hours", notes: "Very low infectious dose (~18 viral particles)" },
        IncubationInfo { disease: "Rotavirus Gastroenteritis", min_days: 1.0, max_days: 3.0, typical: "2 days", notes: "Shedding can continue for 10 days after symptom resolution" },
        IncubationInfo { disease: "Hand, Foot, and Mouth Disease", min_days: 3.0, max_days: 7.0, typical: "3-5 days", notes: "Virus shed in stool for weeks after recovery" },
        IncubationInfo { disease: "Scarlet Fever", min_days: 1.0, max_days: 7.0, typical: "2-5 days", notes: "Usually follows streptococcal pharyngitis onset" },
        IncubationInfo { disease: "Meningitis (Bacterial)", min_days: 1.0, max_days: 10.0, typical: "2-4 days", notes: "N. meningitidis: 2-10 days; rapid deterioration possible" },
        IncubationInfo { disease: "Tuberculosis", min_days: 14.0, max_days: 365.0, typical: "4-12 weeks to primary infection", notes: "Latent TB: may reactivate decades later" },
        IncubationInfo { disease: "Leprosy (Hansen's Disease)", min_days: 365.0, max_days: 7300.0, typical: "3-5 years", notes: "One of longest incubation periods; range 1-20+ years" },
        IncubationInfo { disease: "Poliomyelitis", min_days: 3.0, max_days: 35.0, typical: "7-14 days", notes: "95% of infections are asymptomatic or mild" },
        IncubationInfo { disease: "Yellow Fever", min_days: 3.0, max_days: 6.0, typical: "3-6 days", notes: "Biphasic illness; toxic phase follows brief remission" },
        IncubationInfo { disease: "West Nile Virus", min_days: 2.0, max_days: 14.0, typical: "2-6 days", notes: "~80% of infections asymptomatic; <1% develop neuroinvasive disease" },
        IncubationInfo { disease: "Japanese Encephalitis", min_days: 5.0, max_days: 15.0, typical: "5-15 days", notes: "Most infections asymptomatic; case fatality 20-30%" },
        IncubationInfo { disease: "Chagas Disease", min_days: 5.0, max_days: 14.0, typical: "1-2 weeks (acute)", notes: "Chronic phase: decades of latency before cardiac/GI complications" },
        IncubationInfo { disease: "Toxoplasmosis", min_days: 5.0, max_days: 23.0, typical: "10-23 days", notes: "Usually asymptomatic in immunocompetent; dangerous in pregnancy" },
        IncubationInfo { disease: "Giardiasis", min_days: 7.0, max_days: 21.0, typical: "7-14 days", notes: "Can become chronic if untreated" },
        IncubationInfo { disease: "Schistosomiasis", min_days: 14.0, max_days: 84.0, typical: "4-6 weeks", notes: "Katayama fever (acute phase) at 4-8 weeks post-exposure" },
        IncubationInfo { disease: "Amoebic Dysentery", min_days: 2.0, max_days: 28.0, typical: "2-4 weeks", notes: "90% of infections are asymptomatic; cyst shedding continues" },
        IncubationInfo { disease: "Croup (Laryngotracheobronchitis)", min_days: 2.0, max_days: 6.0, typical: "2-4 days", notes: "Usually caused by parainfluenza virus" },
    ]
}

pub fn run(query: Option<&str>, json: bool) {
    let data = get_incubation_data();

    let filtered: Vec<&IncubationInfo> = if let Some(q) = query {
        let q_lower = q.to_lowercase();
        data.iter()
            .filter(|d| d.disease.to_lowercase().contains(&q_lower))
            .collect()
    } else {
        data.iter().collect()
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap_or_default());
        return;
    }

    if filtered.is_empty() {
        if let Some(q) = query {
            println!("{}", format!("No incubation data found for '{q}'.").red());
            println!("Try: openhealth incubation (without arguments to list all)");
        }
        return;
    }

    println!("{}", "═══ Incubation Period Reference ═══".cyan().bold());
    println!();

    for info in &filtered {
        println!("{}", info.disease.yellow().bold());
        println!(
            "  {} {} (range: {:.0}-{:.0} days)",
            "Typical:".white().bold(),
            info.typical.green(),
            info.min_days,
            info.max_days
        );
        println!(
            "  {} {}",
            "Note:".white().bold(),
            info.notes.white().dimmed()
        );
        println!();
    }

    println!(
        "{}",
        format!("Showing {} disease(s).", filtered.len()).dimmed()
    );
    println!(
        "{}",
        "⚕ Incubation periods are approximate and vary by individual.".dimmed()
    );
}
