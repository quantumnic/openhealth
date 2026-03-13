use colored::Colorize;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Prognosis {
    disease: String,
    severity: String,
    recovery_outlook: String,
    typical_duration: String,
    complications: Vec<String>,
    when_to_seek_help: Vec<String>,
    lifestyle_impact: String,
}

pub fn run(conn: &Connection, name: &str, json: bool) {
    let result: Result<(String, String, String), _> = conn.query_row(
        "SELECT name, severity, description FROM diseases WHERE name LIKE ?1",
        [format!("%{name}%")],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    );

    let (disease_name, severity, _description) = match result {
        Ok(d) => d,
        Err(_) => {
            if json {
                println!("{{\"error\": \"Disease not found: {name}\"}}");
            } else {
                eprintln!("{} Disease not found: {name}", "✗".red());
                eprintln!("  Try: openhealth search {name}");
            }
            return;
        }
    };

    // Get treatment info
    let treatment: Option<(String, String)> = conn
        .query_row(
            "SELECT protocol, prevention FROM treatments t JOIN diseases d ON d.id = t.disease_id WHERE d.name = ?1",
            [&disease_name],
            |row| Ok((row.get(0)?, row.get::<_, Option<String>>(1)?.unwrap_or_default())),
        )
        .ok();

    // Get risk factors
    let mut rf_stmt = conn
        .prepare(
            "SELECT factor, impact FROM risk_factors rf JOIN diseases d ON d.id = rf.disease_id WHERE d.name = ?1",
        )
        .unwrap();
    let risk_factors: Vec<(String, String)> = rf_stmt
        .query_map([&disease_name], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // Build prognosis based on severity and available data
    let recovery_outlook = match severity.as_str() {
        "high" => "Requires prompt medical intervention. Outcomes depend heavily on speed of treatment.",
        "medium" => "Generally treatable with appropriate medical care. Most patients recover well.",
        _ => "Usually self-limiting or manageable with simple treatment. Good prognosis.",
    };

    let typical_duration = match severity.as_str() {
        "high" => "Days to weeks for acute phase; may require ongoing management",
        "medium" => "1-4 weeks with treatment; some conditions are chronic",
        _ => "3-14 days for most cases; chronic conditions need ongoing care",
    };

    let complications = build_complications(&disease_name, &severity);
    let when_to_seek = build_warning_signs(&disease_name, &severity);
    let lifestyle_impact = build_lifestyle_impact(&severity, &risk_factors);

    let prognosis = Prognosis {
        disease: disease_name.clone(),
        severity: severity.clone(),
        recovery_outlook: recovery_outlook.to_string(),
        typical_duration: typical_duration.to_string(),
        complications,
        when_to_seek_help: when_to_seek,
        lifestyle_impact,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&prognosis).unwrap());
        return;
    }

    // Display
    println!();
    println!(
        "{}",
        format!("🔮 Prognosis: {disease_name}").bold().cyan()
    );
    println!("{}", "─".repeat(50));

    let sev_display = match severity.as_str() {
        "high" => "🔴 High".red().bold(),
        "medium" => "🟡 Medium".yellow().bold(),
        _ => "🟢 Low".green().bold(),
    };
    println!("  {} {sev_display}", "Severity:".bold());
    println!("  {} {recovery_outlook}", "Outlook:".bold());
    println!(
        "  {} {typical_duration}",
        "Typical Duration:".bold()
    );

    if !prognosis.complications.is_empty() {
        println!();
        println!("  {}", "⚠️  Possible Complications:".bold().yellow());
        for c in &prognosis.complications {
            println!("    • {c}");
        }
    }

    if !prognosis.when_to_seek_help.is_empty() {
        println!();
        println!("  {}", "🚨 Seek Help If:".bold().red());
        for w in &prognosis.when_to_seek_help {
            println!("    • {w}");
        }
    }

    if let Some((protocol, prevention)) = &treatment {
        println!();
        println!("  {}", "💊 Treatment Summary:".bold());
        // Show first sentence of protocol
        let first = protocol.split('.').next().unwrap_or(protocol);
        println!("    {first}.");
        if !prevention.is_empty() {
            let prev_first = prevention.split('.').next().unwrap_or(prevention);
            println!("  {}", "🛡️  Prevention:".bold());
            println!("    {prev_first}.");
        }
    }

    if !risk_factors.is_empty() {
        println!();
        println!("  {}", "📊 Key Risk Factors:".bold());
        for (factor, impact) in risk_factors.iter().take(5) {
            let icon = match impact.as_str() {
                "high" => "🔴",
                "moderate" => "🟡",
                _ => "🟢",
            };
            println!("    {icon} {factor} ({impact})");
        }
    }

    println!();
    println!(
        "  {}",
        "⚕️  This is informational only — consult a healthcare provider."
            .dimmed()
    );
    println!(
        "  {}",
        format!("  Use: openhealth treatment \"{disease_name}\" for full protocol").dimmed()
    );
    println!();
}

fn build_complications(disease: &str, severity: &str) -> Vec<String> {
    let mut complications = Vec::new();

    // Disease-specific complications
    match disease {
        "Malaria" => {
            complications.push("Cerebral malaria".into());
            complications.push("Severe anemia".into());
            complications.push("Organ failure".into());
        }
        "Pneumonia" => {
            complications.push("Pleural effusion".into());
            complications.push("Sepsis".into());
            complications.push("Lung abscess".into());
        }
        "Heart Attack" => {
            complications.push("Heart failure".into());
            complications.push("Arrhythmias".into());
            complications.push("Cardiogenic shock".into());
        }
        "Stroke" => {
            complications.push("Permanent disability".into());
            complications.push("Aspiration pneumonia".into());
            complications.push("Recurrent stroke".into());
        }
        "Diabetes Type 2" => {
            complications.push("Diabetic neuropathy".into());
            complications.push("Kidney disease".into());
            complications.push("Retinopathy".into());
        }
        _ => {
            // Generic severity-based complications
            match severity {
                "high" => {
                    complications.push("Organ damage if untreated".into());
                    complications.push("Secondary infections".into());
                    complications.push("Long-term sequelae possible".into());
                }
                "medium" => {
                    complications.push("Chronic progression if untreated".into());
                    complications.push("Recurrence possible".into());
                }
                _ => {
                    complications.push("Rare complications with proper care".into());
                }
            }
        }
    }

    complications
}

fn build_warning_signs(_disease: &str, severity: &str) -> Vec<String> {
    let mut signs = Vec::new();

    // Universal warning signs
    signs.push("Symptoms worsening despite treatment".into());

    match severity {
        "high" => {
            signs.push("Difficulty breathing or shortness of breath".into());
            signs.push("Confusion or altered consciousness".into());
            signs.push("Persistent high fever (>39°C / 102°F)".into());
            signs.push("Severe chest or abdominal pain".into());
        }
        "medium" => {
            signs.push("Fever not improving after 48 hours".into());
            signs.push("New or unusual symptoms appearing".into());
            signs.push("Unable to keep fluids down".into());
        }
        _ => {
            signs.push("Symptoms lasting more than 7-10 days".into());
            signs.push("Fever developing in a previously afebrile illness".into());
        }
    }

    signs
}

fn build_lifestyle_impact(severity: &str, risk_factors: &[(String, String)]) -> String {
    let modifiable: Vec<&str> = risk_factors
        .iter()
        .filter(|(_, impact)| impact == "high" || impact == "moderate")
        .filter_map(|(factor, _)| {
            let f = factor.as_str();
            if f.contains("smoking")
                || f.contains("obesity")
                || f.contains("diet")
                || f.contains("alcohol")
                || f.contains("exercise")
                || f.contains("hygiene")
                || f.contains("sedentary")
            {
                Some(f)
            } else {
                None
            }
        })
        .collect();

    if !modifiable.is_empty() {
        format!(
            "Modifiable risk factors identified: {}. Addressing these can significantly improve outcomes.",
            modifiable.join(", ")
        )
    } else {
        match severity {
            "high" => "May require significant lifestyle adjustments during and after treatment.".into(),
            "medium" => "Moderate lifestyle adjustments may help recovery and prevention.".into(),
            _ => "Minimal long-term lifestyle impact expected with proper management.".into(),
        }
    }
}
