use colored::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BmiResult {
    pub weight_kg: f64,
    pub height_m: f64,
    pub bmi: f64,
    pub category: String,
    pub healthy_weight_range_kg: (f64, f64),
    pub health_risks: Vec<String>,
    pub recommendations: Vec<String>,
}

fn classify_bmi(bmi: f64) -> (&'static str, &'static str, Vec<&'static str>, Vec<&'static str>) {
    if bmi < 16.0 {
        (
            "Severe Underweight",
            "🔴",
            vec!["Severe malnutrition risk", "Immune dysfunction", "Organ failure risk", "Osteoporosis"],
            vec!["Seek immediate medical evaluation", "Nutritional rehabilitation program", "Monitor cardiac function"],
        )
    } else if bmi < 17.0 {
        (
            "Moderate Underweight",
            "🟡",
            vec!["Malnutrition risk", "Reduced immune function", "Bone density loss", "Anemia"],
            vec!["Consult healthcare provider", "Increase caloric intake gradually", "Consider nutritional supplements"],
        )
    } else if bmi < 18.5 {
        (
            "Mild Underweight",
            "🟡",
            vec!["Nutrient deficiency risk", "Reduced muscle mass", "Fatigue"],
            vec!["Balanced diet with adequate protein", "Strength training", "Regular health check-ups"],
        )
    } else if bmi < 25.0 {
        (
            "Normal Weight",
            "🟢",
            vec!["Lowest overall health risk"],
            vec!["Maintain balanced diet and regular exercise", "Annual health check-ups"],
        )
    } else if bmi < 30.0 {
        (
            "Overweight",
            "🟡",
            vec!["Increased cardiovascular risk", "Type 2 diabetes risk", "Joint stress", "Sleep apnea risk"],
            vec!["Aim for 150 min/week moderate exercise", "Reduce processed food intake", "Monitor blood pressure and glucose"],
        )
    } else if bmi < 35.0 {
        (
            "Obese (Class I)",
            "🔴",
            vec!["High cardiovascular risk", "Type 2 diabetes", "Hypertension", "Joint disease", "Sleep apnea"],
            vec!["Consult healthcare provider for weight management plan", "Structured exercise program", "Dietary changes with professional guidance"],
        )
    } else if bmi < 40.0 {
        (
            "Obese (Class II)",
            "🔴",
            vec!["Very high cardiovascular risk", "Metabolic syndrome", "Fatty liver disease", "Cancer risk increase"],
            vec!["Medical weight management program", "Consider pharmacotherapy", "Regular monitoring of metabolic markers"],
        )
    } else {
        (
            "Obese (Class III)",
            "🔴",
            vec!["Extreme cardiovascular risk", "Severe metabolic complications", "Reduced life expectancy", "Mobility impairment"],
            vec!["Comprehensive medical evaluation", "Consider bariatric surgery referral", "Multidisciplinary weight management team"],
        )
    }
}

pub fn run(input: &str, json: bool) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let (weight_kg, height_m) = match parse_input(&parts) {
        Some(v) => v,
        None => {
            eprintln!("Usage: openhealth bmi <weight_kg> <height_cm>");
            eprintln!("  Example: openhealth bmi 75 180");
            eprintln!("  (weight in kg, height in cm)");
            return;
        }
    };

    if weight_kg <= 0.0 || weight_kg > 500.0 || height_m <= 0.0 || height_m > 3.0 {
        eprintln!("Invalid values. Weight: 1-500 kg, Height: 30-300 cm.");
        return;
    }

    let bmi = weight_kg / (height_m * height_m);
    let (category, emoji, risks, recommendations) = classify_bmi(bmi);

    let healthy_low = 18.5 * height_m * height_m;
    let healthy_high = 24.9 * height_m * height_m;

    if json {
        let result = BmiResult {
            weight_kg,
            height_m,
            bmi: (bmi * 10.0).round() / 10.0,
            category: category.to_string(),
            healthy_weight_range_kg: (
                (healthy_low * 10.0).round() / 10.0,
                (healthy_high * 10.0).round() / 10.0,
            ),
            health_risks: risks.iter().map(|s| s.to_string()).collect(),
            recommendations: recommendations.iter().map(|s| s.to_string()).collect(),
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return;
    }

    println!();
    println!("{}", "═══ BMI Calculator ═══".bold().cyan());
    println!();
    println!("  Weight:  {:.1} kg", weight_kg);
    println!("  Height:  {:.0} cm ({:.2} m)", height_m * 100.0, height_m);
    println!("  BMI:     {:.1}", bmi);
    println!();
    println!(
        "  {} {} {}",
        emoji,
        "Classification:".bold(),
        category.bold()
    );
    println!();

    // Visual BMI scale
    print_bmi_scale(bmi);
    println!();

    println!(
        "  Healthy weight range: {:.1} – {:.1} kg",
        healthy_low, healthy_high
    );

    if bmi < 18.5 {
        let deficit = healthy_low - weight_kg;
        println!(
            "  {} {:.1} kg below healthy range",
            "↑".yellow(),
            deficit
        );
    } else if bmi >= 25.0 {
        let excess = weight_kg - healthy_high;
        println!(
            "  {} {:.1} kg above healthy range",
            "↓".yellow(),
            excess
        );
    }

    println!();
    println!("  {}", "Health Risks:".bold());
    for risk in &risks {
        let color = if emoji == "🟢" {
            risk.green().to_string()
        } else if emoji == "🟡" {
            risk.yellow().to_string()
        } else {
            risk.red().to_string()
        };
        println!("    • {color}");
    }

    println!();
    println!("  {}", "Recommendations:".bold());
    for rec in &recommendations {
        println!("    → {rec}");
    }

    println!();
    println!(
        "  {}",
        "⚠  BMI is a screening tool, not a diagnostic measure.".dimmed()
    );
    println!(
        "  {}",
        "   It does not account for muscle mass, bone density, or body composition.".dimmed()
    );
    println!(
        "  {}",
        "   Consult a healthcare professional for personalized assessment.".dimmed()
    );
    println!();
}

fn parse_input(parts: &[&str]) -> Option<(f64, f64)> {
    if parts.len() < 2 {
        return None;
    }
    let weight: f64 = parts[0].parse().ok()?;
    let height_cm: f64 = parts[1].parse().ok()?;
    let height_m = height_cm / 100.0;
    Some((weight, height_m))
}

fn print_bmi_scale(bmi: f64) {
    let segments = vec![
        (16.0, "Severe UW", "red"),
        (17.0, "Mod UW", "red"),
        (18.5, "Mild UW", "yellow"),
        (25.0, "Normal", "green"),
        (30.0, "Overweight", "yellow"),
        (35.0, "Obese I", "red"),
        (40.0, "Obese II", "red"),
        (50.0, "Obese III", "red"),
    ];

    print!("  ");
    for (_, label, color) in &segments {
        let colored_label = match *color {
            "red" => format!("{}", label.red()),
            "yellow" => format!("{}", label.yellow()),
            "green" => format!("{}", label.green()),
            _ => label.to_string(),
        };
        print!("│{colored_label:<10}");
    }
    println!("│");

    // Pointer line
    let total_width = segments.len() * 11;
    let min_bmi = 14.0_f64;
    let max_bmi = 50.0_f64;
    let clamped = bmi.clamp(min_bmi, max_bmi);
    let pos = ((clamped - min_bmi) / (max_bmi - min_bmi) * total_width as f64) as usize;
    let pos = pos.min(total_width - 1);
    print!("  ");
    for i in 0..total_width {
        if i == pos {
            print!("{}", "▲".bold().cyan());
        } else {
            print!("─");
        }
    }
    println!();
    print!("  ");
    for _ in 0..pos {
        print!(" ");
    }
    println!("{}", format!("{:.1}", bmi).bold().cyan());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmi_normal() {
        let bmi = 75.0 / (1.8 * 1.8);
        let (cat, _, _, _) = classify_bmi(bmi);
        assert_eq!(cat, "Normal Weight");
    }

    #[test]
    fn test_bmi_underweight() {
        let bmi = 58.0 / (1.8 * 1.8);
        let (cat, _, _, _) = classify_bmi(bmi);
        assert_eq!(cat, "Mild Underweight");
    }

    #[test]
    fn test_bmi_overweight() {
        let bmi = 90.0 / (1.8 * 1.8);
        let (cat, _, _, _) = classify_bmi(bmi);
        assert_eq!(cat, "Overweight");
    }

    #[test]
    fn test_bmi_obese() {
        let bmi = 120.0 / (1.8 * 1.8);
        let (cat, _, _, _) = classify_bmi(bmi);
        assert_eq!(cat, "Obese (Class II)");
    }

    #[test]
    fn test_bmi_severe_underweight() {
        let bmi = 35.0 / (1.7 * 1.7);
        let (cat, _, _, _) = classify_bmi(bmi);
        assert_eq!(cat, "Severe Underweight");
    }

    #[test]
    fn test_parse_input() {
        let parts = vec!["75", "180"];
        let (w, h) = parse_input(&parts).unwrap();
        assert!((w - 75.0).abs() < 0.01);
        assert!((h - 1.8).abs() < 0.01);
    }

    #[test]
    fn test_parse_input_invalid() {
        let parts = vec!["abc"];
        assert!(parse_input(&parts).is_none());
    }

    #[test]
    fn test_bmi_json_output() {
        // Just verify it doesn't panic
        run("75 180", true);
    }

    #[test]
    fn test_healthy_range() {
        let height_m = 1.75;
        let low = 18.5 * height_m * height_m;
        let high = 24.9 * height_m * height_m;
        assert!(low > 50.0 && low < 60.0);
        assert!(high > 70.0 && high < 80.0);
    }
}
