use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct RegionResult {
    region: String,
    diseases: Vec<RegionDisease>,
}

#[derive(Serialize)]
struct RegionDisease {
    name: String,
    severity: String,
    description: String,
    key_symptoms: Vec<String>,
}

/// Maps body regions to symptom keywords that indicate that region.
fn get_region_keywords() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("head", vec!["headache", "head", "scalp", "skull", "migraine", "brain", "confusion", "seizure", "dizziness", "vertigo"]),
        ("eyes", vec!["eye", "vision", "blurred", "blind", "floater", "retinal", "pupil", "corneal", "glaucoma", "eyelid", "conjunctiv"]),
        ("ears", vec!["ear", "hearing", "tinnitus", "deaf", "otitis", "auditory", "vertigo"]),
        ("mouth", vec!["mouth", "oral", "tongue", "tooth", "dental", "gum", "jaw", "throat", "swallow", "tonsil"]),
        ("neck", vec!["neck", "stiff neck", "cervical", "thyroid", "lymph node"]),
        ("chest", vec!["chest", "heart", "cardiac", "lung", "breath", "cough", "wheez", "palpitation", "rib"]),
        ("abdomen", vec!["abdomen", "abdominal", "stomach", "liver", "pancrea", "gallbladder", "spleen", "intestin", "bowel", "nausea", "vomit", "diarrhea"]),
        ("pelvis", vec!["pelvis", "pelvic", "bladder", "urin", "kidney", "groin", "genital", "uterus", "ovary", "prostate", "testicular"]),
        ("skin", vec!["skin", "rash", "itch", "blister", "lesion", "sore", "wart", "mole", "hive", "eczema", "psoriasis", "acne", "dermat"]),
        ("extremities", vec!["arm", "leg", "hand", "foot", "finger", "toe", "joint", "knee", "ankle", "wrist", "elbow", "shoulder", "hip", "heel"]),
        ("back", vec!["back pain", "spine", "spinal", "lumbar", "sciatic"]),
        ("whole body", vec!["fever", "fatigue", "weight loss", "chills", "sweating", "malaise"]),
    ]
}

pub fn run(conn: &Connection, region_input: Option<&str>, json: bool) {
    let regions = get_region_keywords();

    if region_input.is_none() {
        // List available regions
        if json {
            let region_names: Vec<&str> = regions.iter().map(|(r, _)| *r).collect();
            println!("{}", serde_json::to_string_pretty(&region_names).unwrap_or_else(|_| "[]".into()));
        } else {
            println!("{}", "━━━ Body Regions ━━━".bold());
            println!();
            for (region, keywords) in &regions {
                println!("  🏷️  {} — keywords: {}", region.bold(), keywords[..3.min(keywords.len())].join(", ").dimmed());
            }
            println!();
            println!("Usage: {} <region>", "openhealth region".bright_cyan());
            println!("Example: {} chest", "openhealth region".bright_cyan());
        }
        return;
    }

    let query = region_input.unwrap().to_lowercase();
    let matched_region = regions.iter().find(|(r, _)| r.contains(query.as_str()) || query.contains(*r));

    let (region_name, keywords) = match matched_region {
        Some((name, kw)) => (*name, kw.clone()),
        None => {
            if json {
                println!("{{\"error\": \"Unknown body region: {query}\"}}");
            } else {
                println!("{} Unknown body region: '{query}'", "✗".red());
                println!("Available regions: {}", regions.iter().map(|(r, _)| *r).collect::<Vec<_>>().join(", "));
            }
            return;
        }
    };

    // Find diseases whose symptoms match this region's keywords
    let mut disease_matches: Vec<RegionDisease> = Vec::new();

    let mut stmt = conn.prepare(
        "SELECT d.id, d.name, d.severity, d.description FROM diseases d ORDER BY d.name"
    ).unwrap();

    let diseases: Vec<(i64, String, String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    for (disease_id, name, severity, description) in &diseases {
        let mut sym_stmt = conn.prepare(
            "SELECT s.name FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.disease_id = ?1"
        ).unwrap();

        let symptoms: Vec<String> = sym_stmt
            .query_map([disease_id], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        let matching_symptoms: Vec<String> = symptoms.iter()
            .filter(|sym| {
                let sym_lower = sym.to_lowercase();
                keywords.iter().any(|kw| sym_lower.contains(kw))
            })
            .cloned()
            .collect();

        if !matching_symptoms.is_empty() {
            disease_matches.push(RegionDisease {
                name: name.clone(),
                severity: severity.clone(),
                description: description.clone(),
                key_symptoms: matching_symptoms,
            });
        }
    }

    // Sort by severity (high first)
    disease_matches.sort_by(|a, b| {
        let sev_order = |s: &str| match s { "high" => 0, "medium" => 1, _ => 2 };
        sev_order(&a.severity).cmp(&sev_order(&b.severity))
    });

    if json {
        let result = RegionResult {
            region: region_name.to_string(),
            diseases: disease_matches,
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".into()));
        return;
    }

    println!();
    println!("{} {} ({} conditions found)", "━━━".bold(), format!("Body Region: {}", region_name.to_uppercase()).bold(), disease_matches.len());
    println!();

    if disease_matches.is_empty() {
        println!("  No diseases found for this region.");
        return;
    }

    for d in disease_matches.iter().take(20) {
        let sev_emoji = match d.severity.as_str() {
            "high" => "🔴",
            "medium" => "🟡",
            _ => "🟢",
        };
        println!("  {} {} — {}", sev_emoji, d.name.bold(), d.description.dimmed());
        println!("    Related symptoms: {}", d.key_symptoms.join(", ").green());
        println!();
    }

    if disease_matches.len() > 20 {
        println!("  ... and {} more. Use --json for complete list.", disease_matches.len() - 20);
    }
}
