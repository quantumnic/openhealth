use crate::engine::scorer::DiagnosisResult;
use crate::engine::severity::SeverityLevel;
use colored::*;

pub fn print_banner() {
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║           🏥  OpenHealth — Medical Diagnostics          ║".bright_cyan()
    );
    println!(
        "{}",
        "║        Healthcare for the 3.5 billion without access     ║".bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!();
}

pub fn print_disclaimer() {
    println!(
        "{}",
        "⚠️  DISCLAIMER: This tool provides general health information only."
            .yellow()
            .bold()
    );
    println!(
        "{}",
        "   It is NOT a substitute for professional medical advice.".yellow()
    );
    println!(
        "{}",
        "   Always consult a healthcare provider for diagnosis and treatment.".yellow()
    );
    println!();
}

pub fn print_diagnosis_results(results: &[DiagnosisResult], max_results: usize) {
    if results.is_empty() {
        println!(
            "{}",
            "No matching conditions found for the given symptoms.".yellow()
        );
        println!("Try describing your symptoms differently or add more details.");
        return;
    }

    let show = results.iter().take(max_results);

    println!("{}", "━━━ Possible Conditions ━━━".bold());
    println!();

    for (i, result) in show.enumerate() {
        let severity = SeverityLevel::from_str(&result.severity);
        let prob_bar = probability_bar(result.probability);

        println!(
            "{}. {} {} ({})",
            i + 1,
            severity.emoji(),
            result.disease_name.bold(),
            format!("{:.0}% match", result.probability).bright_white()
        );
        println!("   {prob_bar}");
        println!("   {}", result.description.dimmed());
        println!(
            "   Matched: {}",
            result.matched_symptoms.join(", ").green()
        );
        if !result.missing_key_symptoms.is_empty() {
            println!(
                "   Missing key symptoms: {}",
                result.missing_key_symptoms.join(", ").red()
            );
        }
        println!("   Severity: {} {}", severity.emoji(), severity.label());
        println!();
    }

    // Overall severity
    let severities: Vec<&str> = results.iter().take(3).map(|r| r.severity.as_str()).collect();
    let overall = crate::engine::severity::overall_severity(&severities);
    println!("{}", "━━━ Recommendation ━━━".bold());
    println!("{} {}", overall.emoji(), overall.advice());
    println!();
}

fn probability_bar(prob: f64) -> String {
    let filled = (prob / 5.0) as usize;
    let empty = 20_usize.saturating_sub(filled);
    format!(
        "[{}{}] {:.0}%",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed(),
        prob
    )
}

#[allow(clippy::too_many_arguments)]
pub fn print_disease_info(
    name: &str,
    description: &str,
    severity: &str,
    contagious: bool,
    icd11_code: &str,
    age_group: &str,
    category: &str,
    symptoms: &[(String, f64, bool)],
    risk_factors: &[(String, String)],
) {
    let sev = SeverityLevel::from_str(severity);
    println!("{}", "━━━ Disease Information ━━━".bold());
    println!("  Name:        {}", name.bold());
    println!("  Description: {}", description);
    println!("  Severity:    {} {}", sev.emoji(), sev.label());
    println!(
        "  Contagious:  {}",
        if contagious { "Yes ⚠️" } else { "No" }
    );
    println!("  ICD-11:      {}", icd11_code);
    println!("  Age group:   {}", age_group);
    println!("  Category:    {}", category.bright_cyan());
    println!();
    println!("  {}", "Symptoms:".underline());
    for (sym, weight, primary) in symptoms {
        let marker = if *primary {
            "★".yellow().to_string()
        } else {
            "•".to_string()
        };
        let w_pct = (weight * 100.0) as u32;
        println!("    {marker} {sym} (weight: {w_pct}%)");
    }
    if !risk_factors.is_empty() {
        println!();
        println!("  {}", "Risk Factors:".underline());
        for (factor, impact) in risk_factors {
            let impact_color = match impact.as_str() {
                "high" => format!("[{impact}]").red().to_string(),
                "moderate" => format!("[{impact}]").yellow().to_string(),
                _ => format!("[{impact}]").green().to_string(),
            };
            println!("    ⚡ {factor} {impact_color}");
        }
    }
    println!();
}

pub fn print_treatment(
    disease_name: &str,
    protocol: &str,
    source: &str,
    first_aid: &str,
    prevention: &str,
) {
    println!("{}", "━━━ Treatment Protocol ━━━".bold());
    println!("  Disease:    {}", disease_name.bold());
    println!("  Source:     {}", source.bright_cyan());
    println!();
    println!("  {}", "Treatment:".underline());
    println!("  {protocol}");
    println!();
    println!("  {}", "First Aid:".underline());
    println!("  {first_aid}");
    println!();
    println!("  {}", "Prevention:".underline());
    println!("  {prevention}");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_bar_100() {
        let bar = probability_bar(100.0);
        assert!(bar.contains("100%"));
    }

    #[test]
    fn test_probability_bar_0() {
        let bar = probability_bar(0.0);
        assert!(bar.contains("0%"));
    }
}
