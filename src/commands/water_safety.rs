use colored::Colorize;

struct WaterMethod {
    name: &'static str,
    effectiveness: &'static str,
    instructions: &'static str,
    pros: &'static str,
    cons: &'static str,
    kills: &'static str,
}

fn get_methods() -> Vec<WaterMethod> {
    vec![
        WaterMethod {
            name: "Boiling",
            effectiveness: "Very High",
            instructions: "Bring water to a rolling boil for at least 1 minute (3 minutes above 2000m altitude). Let cool naturally. Do not add ice.",
            pros: "Most reliable method. Kills all pathogens. No special equipment needed.",
            cons: "Requires fuel. Slow. Does not remove chemical contaminants or turbidity.",
            kills: "Bacteria ✓ | Viruses ✓ | Parasites ✓ | Cysts ✓",
        },
        WaterMethod {
            name: "Solar Disinfection (SODIS)",
            effectiveness: "High",
            instructions: "Fill clean PET plastic bottles (1-2L). If water is cloudy, filter through cloth first. Place in direct sunlight for 6+ hours (or 2 days if cloudy sky). Lay bottles on reflective surface (metal roof).",
            pros: "Free. No chemicals needed. Uses only sunlight and plastic bottles.",
            cons: "Slow (6-48 hours). Needs clear bottles. Less effective in cloudy weather. Small volume per batch.",
            kills: "Bacteria ✓ | Viruses ✓ | Parasites ✓ (with sufficient exposure)",
        },
        WaterMethod {
            name: "Chlorination",
            effectiveness: "High",
            instructions: "Add 2 drops of unscented household bleach (5-6% sodium hypochlorite) per liter. Stir and wait 30 minutes. Water should have slight chlorine smell. Double dose for cloudy water.",
            pros: "Fast. Cheap. Provides residual protection. Widely available.",
            cons: "Does not kill Cryptosporidium. Bad taste. Requires correct dosing. Less effective in turbid water.",
            kills: "Bacteria ✓ | Viruses ✓ | Parasites (partial) | Cysts ✗",
        },
        WaterMethod {
            name: "Ceramic/Pot Filtration",
            effectiveness: "High",
            instructions: "Pour water through ceramic filter pot. Water passes through micropores. Collect filtered water in clean container below. Clean filter regularly by scrubbing under clean water.",
            pros: "Reusable. No chemicals. Good for household use. Some have silver coating for extra disinfection.",
            cons: "Slow flow rate. Fragile. Does not remove all viruses. Needs regular cleaning.",
            kills: "Bacteria ✓ | Viruses (partial) | Parasites ✓ | Cysts ✓",
        },
        WaterMethod {
            name: "Cloth Filtration",
            effectiveness: "Moderate",
            instructions: "Fold clean cotton cloth (like sari cloth) 4-8 times. Pour water through it into clean container. This removes larger particles, copepods, and significantly reduces cholera risk.",
            pros: "Free. Available everywhere. Proven to reduce cholera by ~50%. Immediate.",
            cons: "Does not remove bacteria, viruses, or chemicals. Only a pre-treatment step.",
            kills: "Bacteria ✗ | Viruses ✗ | Parasites (partial) | Reduces cholera vectors ✓",
        },
        WaterMethod {
            name: "Water Purification Tablets",
            effectiveness: "High",
            instructions: "Follow package directions (typically 1 tablet per liter). Wait 30-60 minutes before drinking. For chlorine dioxide tablets, wait 4 hours for Cryptosporidium.",
            pros: "Lightweight. Portable. Long shelf life. Good for emergencies and travel.",
            cons: "Costs money. Bad taste. Some tablets slow-acting. Less effective in cold/turbid water.",
            kills: "Bacteria ✓ | Viruses ✓ | Parasites ✓ (chlorine dioxide only) | Cysts ✓ (chlorine dioxide)",
        },
    ]
}

pub fn run(method: Option<&str>, as_json: bool) {
    let methods = get_methods();

    if as_json {
        let json: Vec<serde_json::Value> = methods
            .iter()
            .filter(|m| {
                method.is_none()
                    || m.name
                        .to_lowercase()
                        .contains(&method.unwrap_or("").to_lowercase())
            })
            .map(|m| {
                serde_json::json!({
                    "name": m.name,
                    "effectiveness": m.effectiveness,
                    "instructions": m.instructions,
                    "pros": m.pros,
                    "cons": m.cons,
                    "kills": m.kills,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        return;
    }

    println!("\n{}", "💧 Water Safety & Purification Guide".bold().cyan());
    println!("   {}\n", "Making water safe to drink — methods for any situation".dimmed());

    // WHO ORS recipe
    println!("{}", "━".repeat(60).dimmed());
    println!("  {} {}", "🧂".bold(), "WHO Oral Rehydration Solution (ORS) Recipe".bold().yellow());
    println!("  For 1 liter of clean water:");
    println!("    • 6 level teaspoons of sugar");
    println!("    • ½ level teaspoon of salt");
    println!("  Stir until dissolved. Sip frequently. Saves lives during diarrheal illness.");
    println!("{}\n", "━".repeat(60).dimmed());

    let filtered: Vec<&WaterMethod> = if let Some(q) = method {
        let q_lower = q.to_lowercase();
        methods
            .iter()
            .filter(|m| m.name.to_lowercase().contains(&q_lower))
            .collect()
    } else {
        methods.iter().collect()
    };

    if filtered.is_empty() {
        println!(
            "{} No method found matching '{}'. Try: boiling, sodis, chlorination, ceramic, cloth, tablets",
            "⚠".yellow(),
            method.unwrap_or("")
        );
        return;
    }

    for m in &filtered {
        let eff_colored = match m.effectiveness {
            "Very High" => m.effectiveness.green().bold().to_string(),
            "High" => m.effectiveness.green().to_string(),
            "Moderate" => m.effectiveness.yellow().to_string(),
            _ => m.effectiveness.to_string(),
        };
        println!("  {} {} [{}]", "▸".cyan(), m.name.bold(), eff_colored);
        println!("    {} {}", "📋".bold(), "How:".underline());
        println!("    {}", m.instructions);
        println!("    {} {}", "✓".green(), m.pros);
        println!("    {} {}", "✗".red(), m.cons);
        println!("    🦠 {}", m.kills);
        println!();
    }

    println!("{}", "⚠ Important:".bold().yellow());
    println!("  • Always treat water if you're unsure of the source");
    println!("  • Turbid (cloudy) water should be pre-filtered through cloth before treatment");
    println!("  • Store treated water in clean, covered containers");
    println!("  • When in doubt, BOIL — it's the most reliable method");
    println!();
}
