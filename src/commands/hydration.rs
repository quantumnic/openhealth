use colored::*;
use serde::Serialize;

#[derive(Serialize)]
struct HydrationResult {
    weight_kg: f64,
    base_ml: f64,
    activity_ml: f64,
    climate_ml: f64,
    total_ml: f64,
    total_liters: f64,
    glasses_250ml: u32,
    category: String,
    tips: Vec<String>,
}

/// Calculate recommended daily water intake based on weight, activity level, and climate.
///
/// Formula: Base = 35 mL/kg body weight (WHO guideline range)
/// Activity bonus: +500 mL (moderate), +1000 mL (intense)
/// Climate bonus: +500 mL (hot/humid)
pub fn run(input: &str, json: bool) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        eprintln!("Usage: openhealth hydration <weight_kg> [activity] [climate]");
        eprintln!("  weight_kg: body weight in kilograms");
        eprintln!("  activity:  sedentary | moderate | intense  (default: sedentary)");
        eprintln!("  climate:   temperate | hot                  (default: temperate)");
        eprintln!();
        eprintln!("Example: openhealth hydration 70 moderate hot");
        return;
    }

    let weight: f64 = match parts[0].parse() {
        Ok(w) if w > 0.0 && w <= 500.0 => w,
        _ => {
            eprintln!("Invalid weight. Please enter a number between 1 and 500 kg.");
            return;
        }
    };

    let activity = parts.get(1).copied().unwrap_or("sedentary");
    let climate = parts.get(2).copied().unwrap_or("temperate");

    let base_ml = weight * 35.0;

    let (activity_ml, activity_label) = match activity.to_lowercase().as_str() {
        "moderate" | "mod" => (500.0, "moderate"),
        "intense" | "high" | "heavy" => (1000.0, "intense"),
        _ => (0.0, "sedentary"),
    };

    let (climate_ml, climate_label) = match climate.to_lowercase().as_str() {
        "hot" | "humid" | "tropical" => (500.0, "hot/humid"),
        _ => (0.0, "temperate"),
    };

    let total_ml = base_ml + activity_ml + climate_ml;
    let total_liters = total_ml / 1000.0;
    let glasses = (total_ml / 250.0).ceil() as u32;

    let category = if total_ml < 2000.0 {
        "Low intake — ensure adequate hydration"
    } else if total_ml < 3000.0 {
        "Normal range"
    } else if total_ml < 4000.0 {
        "Elevated — active/hot climate needs"
    } else {
        "High — monitor electrolyte balance"
    };

    let mut tips = vec![
        "Drink water before you feel thirsty — thirst indicates early dehydration".to_string(),
        "Monitor urine color — pale yellow indicates good hydration".to_string(),
    ];

    if activity_label == "intense" {
        tips.push("Replace electrolytes during prolonged exercise (>60 min)".to_string());
        tips.push("Drink 150-350 mL every 15-20 minutes during intense exercise".to_string());
    }

    if climate_label == "hot/humid" {
        tips.push("Increase intake in hot weather — sweat losses can exceed 1L/hour".to_string());
    }

    if weight > 100.0 {
        tips.push("Larger body mass requires proportionally more water — spread intake throughout the day".to_string());
    }

    tips.push("Fruits and vegetables contribute ~20% of daily water intake".to_string());
    tips.push("Caffeinated beverages have mild diuretic effect — compensate with extra water".to_string());

    let result = HydrationResult {
        weight_kg: weight,
        base_ml,
        activity_ml,
        climate_ml,
        total_ml,
        total_liters,
        glasses_250ml: glasses,
        category: category.to_string(),
        tips: tips.clone(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return;
    }

    println!();
    println!("{}", "💧 Daily Water Intake Calculator".bold().cyan());
    println!("{}", "═".repeat(42).cyan());
    println!();
    println!("  {} {} kg", "Weight:".bold(), weight);
    println!("  {} {}", "Activity:".bold(), activity_label);
    println!("  {} {}", "Climate:".bold(), climate_label);
    println!();
    println!("{}", "── Calculation ──".dimmed());
    println!("  Base (35 mL/kg):     {:>7.0} mL", base_ml);
    if activity_ml > 0.0 {
        println!("  Activity bonus:      {:>7.0} mL", activity_ml);
    }
    if climate_ml > 0.0 {
        println!("  Climate bonus:       {:>7.0} mL", climate_ml);
    }
    println!("  {}", "─".repeat(30));
    println!(
        "  {}      {:.1} L  ({} glasses)",
        "TOTAL:".bold().green(),
        total_liters,
        glasses
    );
    println!();
    println!("  📊 {}", category.yellow());
    println!();
    println!("{}", "── Hydration Tips ──".dimmed());
    for tip in &tips {
        println!("  💡 {}", tip);
    }
    println!();
    println!(
        "{}",
        "⚕️  Based on WHO/EFSA guidelines. Individual needs may vary."
            .dimmed()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydration_basic() {
        // Just ensure it doesn't panic
        run("70", false);
    }

    #[test]
    fn test_hydration_with_activity() {
        run("80 moderate", false);
    }

    #[test]
    fn test_hydration_full() {
        run("75 intense hot", false);
    }

    #[test]
    fn test_hydration_json() {
        run("70 sedentary temperate", true);
    }

    #[test]
    fn test_hydration_empty() {
        run("", false);
    }

    #[test]
    fn test_hydration_invalid_weight() {
        run("-5", false);
    }
}
