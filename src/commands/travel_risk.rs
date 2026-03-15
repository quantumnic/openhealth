use colored::Colorize;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct TravelRisk {
    region: String,
    diseases: Vec<TravelDisease>,
    general_advice: Vec<String>,
}

#[derive(Debug, Serialize)]
struct TravelDisease {
    name: String,
    severity: String,
    vaccine_available: bool,
    prophylaxis_note: String,
}

struct RegionData {
    name: &'static str,
    aliases: &'static [&'static str],
    disease_patterns: &'static [(&'static str, bool, &'static str)], // (disease, vaccine_available, prophylaxis_note)
    advice: &'static [&'static str],
}

fn get_regions() -> Vec<RegionData> {
    vec![
        RegionData {
            name: "Sub-Saharan Africa",
            aliases: &["africa", "sub-saharan", "subsaharan", "west africa", "east africa", "central africa", "southern africa"],
            disease_patterns: &[
                ("Malaria", false, "Chemoprophylaxis required (atovaquone-proguanil, doxycycline, or mefloquine)"),
                ("Yellow Fever", true, "Vaccination required for entry to many countries; single dose gives lifelong immunity"),
                ("Cholera", true, "Oral vaccine available; safe water practices essential"),
                ("Typhoid Fever", true, "Vaccination recommended; watch food/water hygiene"),
                ("Dengue Fever", false, "Mosquito bite prevention; no widely available prophylaxis"),
                ("Schistosomiasis", false, "Avoid freshwater swimming/wading in endemic lakes and rivers"),
                ("African Trypanosomiasis (Sleeping Sickness)", false, "Avoid tsetse fly bites; wear neutral-colored clothing"),
                ("Meningitis", true, "Meningococcal vaccine for meningitis belt (Sahel); seasonal risk Dec-Jun"),
                ("Ebola", false, "Vaccine available for outbreaks; avoid contact with sick individuals and bushmeat"),
                ("Rabies", true, "Pre-exposure vaccination for extended stays; avoid stray animals"),
                ("Leishmaniasis (Visceral)", false, "Sandfly bite prevention; bed nets"),
                ("Hookworm Infection", false, "Wear shoes; avoid walking barefoot on soil"),
                ("Lymphatic Filariasis (Elephantiasis)", false, "Mosquito bite prevention"),
                ("Buruli Ulcer", false, "Avoid contact with stagnant water; cover skin wounds"),
            ],
            advice: &[
                "⚕️  Visit travel clinic 4-6 weeks before departure",
                "🦟 Use DEET-based repellent and sleep under insecticide-treated bed nets",
                "💧 Drink only bottled or boiled water; avoid ice from unknown sources",
                "🍖 Eat thoroughly cooked food; avoid raw vegetables washed in local water",
                "💊 Carry antimalarial prophylaxis and start before travel as directed",
                "🩺 Carry basic first-aid kit with ORS packets, antibiotics (traveler's diarrhea), and antimalarials",
                "📋 Ensure routine vaccinations (MMR, DPT, polio) are current",
            ],
        },
        RegionData {
            name: "South & Southeast Asia",
            aliases: &["asia", "southeast asia", "south asia", "india", "thailand", "vietnam", "cambodia", "myanmar", "laos", "indonesia", "philippines", "bangladesh", "nepal", "sri lanka"],
            disease_patterns: &[
                ("Malaria", false, "Prophylaxis needed for rural areas; not typically needed for major cities"),
                ("Dengue Fever", false, "Major risk; mosquito bite prevention essential (daytime-biting Aedes)"),
                ("Japanese Encephalitis", true, "Vaccination recommended for extended rural stays; mosquito prevention"),
                ("Typhoid Fever", true, "Vaccination recommended; food/water hygiene critical"),
                ("Cholera", true, "Oral vaccine available; strict water/food hygiene"),
                ("Chikungunya", false, "Mosquito prevention; no vaccine or prophylaxis"),
                ("Leptospirosis", false, "Avoid wading in floodwater; doxycycline prophylaxis for high-risk exposure"),
                ("Rabies", true, "Pre-exposure vaccination recommended; stray dogs common"),
                ("Tuberculosis", true, "BCG vaccination; avoid crowded indoor spaces with poor ventilation"),
                ("Melioidosis", false, "Avoid soil/water contact especially during monsoon; cover wounds"),
                ("Strongyloidiasis", false, "Wear shoes; avoid skin contact with contaminated soil"),
                ("Scrub Typhus", false, "Doxycycline prophylaxis for jungle exposure; repellent on skin"),
            ],
            advice: &[
                "⚕️  Visit travel clinic 4-6 weeks before departure",
                "🦟 Use repellent day AND night (dengue mosquitoes bite during daytime)",
                "💧 Drink only sealed bottled water; avoid street ice",
                "🍜 Eat freshly cooked hot food; be cautious with street food",
                "🐕 Avoid contact with stray dogs and monkeys (rabies risk)",
                "🌧️ Avoid wading in floodwater during monsoon season",
                "💊 Carry ORS, ciprofloxacin (traveler's diarrhea), and any prescribed prophylaxis",
            ],
        },
        RegionData {
            name: "Central & South America",
            aliases: &["latin america", "south america", "central america", "caribbean", "brazil", "mexico", "peru", "colombia", "argentina", "chile", "ecuador", "bolivia"],
            disease_patterns: &[
                ("Dengue Fever", false, "Major risk in tropical lowlands; mosquito bite prevention"),
                ("Zika Virus Disease", false, "Mosquito prevention; pregnancy advisory — avoid travel or strict prevention"),
                ("Malaria", false, "Prophylaxis for Amazon basin and select rural areas"),
                ("Yellow Fever", true, "Vaccination required for Amazon/jungle regions"),
                ("Chagas Disease", false, "Avoid triatomine bug bites; don't sleep in adobe/thatch housing without nets"),
                ("Coccidioidomycosis (Valley Fever)", false, "Avoid dust inhalation in arid areas (Mexico, Central America)"),
                ("Histoplasmosis", false, "Avoid cave exploration and chicken coops without N95 mask"),
                ("Typhoid Fever", true, "Vaccination recommended; food/water hygiene"),
                ("Rabies", true, "Pre-exposure vaccination for adventure travelers"),
                ("Leptospirosis", false, "Avoid contact with floodwater; prophylaxis for rafting/caving"),
                ("Leishmaniasis (Visceral)", false, "Sandfly bite prevention in endemic areas"),
                ("Neurocysticercosis", false, "Avoid undercooked pork; strict hand/food hygiene"),
            ],
            advice: &[
                "⚕️  Visit travel clinic 4-6 weeks before departure",
                "🦟 Mosquito protection critical — use DEET, wear long sleeves, use air-conditioned lodging",
                "🤰 Pregnant women or those planning pregnancy: consult about Zika risk before travel",
                "💧 Safe water practices; avoid unpasteurized juices",
                "🍖 Cook meat thoroughly, especially pork",
                "🏔️ Altitude sickness prevention for Andean travel (acclimatize, acetazolamide)",
                "🐍 Watch for venomous snakes in jungle/rural areas; wear boots",
            ],
        },
        RegionData {
            name: "Middle East & North Africa",
            aliases: &["middle east", "north africa", "mena", "egypt", "morocco", "saudi arabia", "uae", "jordan", "iran", "iraq", "turkey"],
            disease_patterns: &[
                ("Typhoid Fever", true, "Vaccination recommended; food/water hygiene"),
                ("Hepatitis A", true, "Vaccination recommended before travel"),
                ("Leishmaniasis (Visceral)", false, "Sandfly bite prevention; cover skin at dusk"),
                ("Brucellosis", false, "Avoid unpasteurized dairy products and raw meat"),
                ("Q Fever", false, "Avoid contact with livestock birthing products"),
                ("Rabies", true, "Pre-exposure vaccination for extended stays"),
                ("Schistosomiasis", false, "Avoid freshwater swimming in Egypt and other endemic areas"),
                ("Tuberculosis", true, "BCG vaccination; avoid overcrowded settings"),
            ],
            advice: &[
                "⚕️  Visit travel clinic before departure",
                "💧 Drink bottled water; avoid tap water and ice",
                "🥛 Avoid unpasteurized dairy products (brucellosis, listeriosis risk)",
                "☀️ Heat-related illness prevention: hydrate, avoid midday sun, acclimatize",
                "🧴 Use sunscreen and stay hydrated in desert climates",
                "📋 Ensure hepatitis A and routine vaccinations are current",
            ],
        },
        RegionData {
            name: "Oceania & Pacific Islands",
            aliases: &["oceania", "pacific", "pacific islands", "fiji", "papua new guinea", "samoa", "tonga", "solomon islands", "vanuatu", "micronesia"],
            disease_patterns: &[
                ("Dengue Fever", false, "Mosquito bite prevention; outbreaks common"),
                ("Malaria", false, "Prophylaxis for Papua New Guinea, Solomon Islands, Vanuatu"),
                ("Typhoid Fever", true, "Vaccination recommended for some islands"),
                ("Leptospirosis", false, "Avoid freshwater exposure after heavy rains"),
                ("Lymphatic Filariasis (Elephantiasis)", false, "Mosquito bite prevention"),
                ("Ciguatera Fish Poisoning", false, "Avoid large reef fish (barracuda, moray eel, large grouper)"),
                ("Tuberculosis", true, "BCG vaccination; varies by island"),
            ],
            advice: &[
                "⚕️  Visit travel clinic before departure",
                "🦟 Mosquito prevention essential (dengue, malaria on some islands)",
                "🐟 Ask locals about ciguatera risk before eating reef fish",
                "☀️ Strong UV — use high-SPF sunscreen, hat, and protective clothing",
                "💧 Safe water practices on remote islands",
                "🏊 Be cautious of coral cuts (slow to heal, infection risk) and marine stings",
            ],
        },
    ]
}

pub fn run(conn: &Connection, region: Option<&str>, json: bool) {
    if let Some(query) = region {
        let query_lower = query.to_lowercase();
        let regions = get_regions();
        let matched = regions.iter().find(|r| {
            r.name.to_lowercase().contains(&query_lower)
                || r.aliases.iter().any(|a| a.contains(&query_lower) || query_lower.contains(a))
        });

        if let Some(region_data) = matched {
            if json {
                print_json(conn, region_data);
            } else {
                print_region(conn, region_data);
            }
        } else if json {
            println!("{{\"error\": \"Region not found. Available: Sub-Saharan Africa, South & Southeast Asia, Central & South America, Middle East & North Africa, Oceania & Pacific Islands\"}}");
        } else {
            println!("{}", "Region not found.".red());
            println!();
            println!("{}", "Available regions:".bold());
            for r in &regions {
                println!("  • {} (aliases: {})", r.name, r.aliases.join(", "));
            }
        }
    } else {
        let regions = get_regions();
        if json {
            let names: Vec<&str> = regions.iter().map(|r| r.name).collect();
            println!("{}", serde_json::to_string_pretty(&names).unwrap_or_default());
        } else {
            println!("{}", "╔══════════════════════════════════════════╗".cyan());
            println!("{}", "║    🌍 Travel Health Risk Regions         ║".cyan().bold());
            println!("{}", "╚══════════════════════════════════════════╝".cyan());
            println!();
            for r in &regions {
                let disease_count = r.disease_patterns.len();
                let vaccine_count = r.disease_patterns.iter().filter(|d| d.1).count();
                println!("  {} {} ({} risks, {} vaccine-preventable)",
                    "▸".cyan(),
                    r.name.bold(),
                    disease_count,
                    vaccine_count,
                );
            }
            println!();
            println!("{}", "Usage: openhealth travel-risk <region>".dimmed());
            println!("{}", "  e.g. openhealth travel-risk africa".dimmed());
        }
    }
}

fn print_region(conn: &Connection, region: &RegionData) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════╗".to_string().cyan());
    println!("{}", format!("║  🌍 Travel Health Risks: {:<23} ║", region.name).cyan().bold());
    println!("{}", "╚══════════════════════════════════════════════════╝".to_string().cyan());
    println!();

    for (disease_name, vaccine, note) in region.disease_patterns {
        // Look up severity from database
        let severity = conn.query_row(
            "SELECT severity FROM diseases WHERE name = ?1",
            [disease_name],
            |row| row.get::<_, String>(0),
        ).unwrap_or_else(|_| "unknown".to_string());

        let severity_icon = match severity.as_str() {
            "high" => "🔴",
            "medium" => "🟡",
            _ => "🟢",
        };

        let vaccine_icon = if *vaccine { "💉" } else { "  " };

        println!("  {} {} {} {}",
            severity_icon,
            vaccine_icon,
            disease_name.bold(),
            if *vaccine { "(vaccine available)".green().to_string() } else { String::new() },
        );
        println!("      {}", note.dimmed());
        println!();
    }

    println!("{}", "─── General Travel Advice ───".cyan().bold());
    println!();
    for advice in region.advice {
        println!("  {advice}");
    }
    println!();
    println!("{}", "⚠️  This is informational only. Consult a travel medicine specialist for personalized advice.".yellow());
}

fn print_json(conn: &Connection, region: &RegionData) {
    let diseases: Vec<TravelDisease> = region.disease_patterns.iter().map(|(name, vaccine, note)| {
        let severity = conn.query_row(
            "SELECT severity FROM diseases WHERE name = ?1",
            [name],
            |row| row.get::<_, String>(0),
        ).unwrap_or_else(|_| "unknown".to_string());

        TravelDisease {
            name: name.to_string(),
            severity,
            vaccine_available: *vaccine,
            prophylaxis_note: note.to_string(),
        }
    }).collect();

    let result = TravelRisk {
        region: region.name.to_string(),
        diseases,
        general_advice: region.advice.iter().map(|a| a.to_string()).collect(),
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default());
}
