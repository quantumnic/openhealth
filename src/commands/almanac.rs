use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct AlmanacResult {
    month: String,
    hemisphere: String,
    season: String,
    diseases: Vec<AlmanacDisease>,
    tips: Vec<String>,
}

#[derive(Serialize)]
struct AlmanacDisease {
    name: String,
    severity: String,
    reason: String,
}

struct SeasonalEntry {
    disease: &'static str,
    months: &'static [u32],   // 1-12
    reason: &'static str,
}

fn get_seasonal_data() -> Vec<SeasonalEntry> {
    vec![
        SeasonalEntry { disease: "Influenza", months: &[11,12,1,2,3], reason: "Peak flu season in temperate climates" },
        SeasonalEntry { disease: "Common Cold", months: &[9,10,11,12,1,2,3], reason: "Rhinovirus thrives in cool, dry air" },
        SeasonalEntry { disease: "Norovirus Gastroenteritis", months: &[11,12,1,2,3], reason: "Winter vomiting bug peaks in cold months" },
        SeasonalEntry { disease: "Dengue Fever", months: &[6,7,8,9,10], reason: "Rainy season increases Aedes mosquito breeding" },
        SeasonalEntry { disease: "Malaria", months: &[5,6,7,8,9,10], reason: "Warm wet season increases Anopheles mosquito activity" },
        SeasonalEntry { disease: "Heatstroke", months: &[6,7,8], reason: "Summer heat waves; highest risk during extreme temperatures" },
        SeasonalEntry { disease: "Lyme Disease", months: &[5,6,7,8], reason: "Tick nymphs are most active in late spring/summer" },
        SeasonalEntry { disease: "Asthma", months: &[3,4,5,9,10], reason: "Spring pollen and fall allergens trigger exacerbations" },
        SeasonalEntry { disease: "Chickenpox", months: &[3,4,5,6], reason: "Historically peaks in spring before widespread vaccination" },
        SeasonalEntry { disease: "Hand, Foot, and Mouth Disease", months: &[6,7,8,9], reason: "Enterovirus transmission peaks in summer/early fall" },
        SeasonalEntry { disease: "Measles", months: &[2,3,4,5], reason: "Late winter/spring peaks due to indoor crowding" },
        SeasonalEntry { disease: "Rotavirus Gastroenteritis", months: &[1,2,3,4,5], reason: "Peaks in winter/spring in temperate climates" },
        SeasonalEntry { disease: "Cholera", months: &[6,7,8,9,10], reason: "Rainy season and flooding contaminate water supplies" },
        SeasonalEntry { disease: "Yellow Fever", months: &[7,8,9,10], reason: "Peak mosquito season in endemic areas" },
        SeasonalEntry { disease: "Depression", months: &[11,12,1,2], reason: "Seasonal affective disorder (SAD) peaks with reduced daylight" },
        SeasonalEntry { disease: "Pneumonia", months: &[12,1,2,3], reason: "Cold weather and respiratory virus circulation increase risk" },
        SeasonalEntry { disease: "Bronchiolitis", months: &[11,12,1,2], reason: "RSV season peaks in winter" },
        SeasonalEntry { disease: "Tinea Versicolor", months: &[5,6,7,8,9], reason: "Hot humid weather promotes Malassezia overgrowth" },
        SeasonalEntry { disease: "Scabies", months: &[10,11,12,1,2], reason: "Increased close contact and indoor crowding in winter" },
        SeasonalEntry { disease: "Croup", months: &[10,11,12], reason: "Parainfluenza virus circulation peaks in fall" },
        SeasonalEntry { disease: "COVID-19", months: &[11,12,1,2,3], reason: "Respiratory virus with winter surge pattern" },
        SeasonalEntry { disease: "Pertussis", months: &[6,7,8,9], reason: "Late summer/early fall peaks in many regions" },
        SeasonalEntry { disease: "Plantar Fasciitis", months: &[3,4,5], reason: "Spring exercise ramp-up after winter inactivity" },
        SeasonalEntry { disease: "Kidney Stones", months: &[6,7,8], reason: "Dehydration in summer heat increases stone formation risk" },
        SeasonalEntry { disease: "Gout", months: &[3,4,5,6], reason: "Dietary changes and dehydration in warmer months" },
    ]
}

fn season_for_month(month: u32) -> &'static str {
    match month {
        3..=5 => "Spring",
        6..=8 => "Summer",
        9..=11 => "Autumn",
        _ => "Winter",
    }
}

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown",
    }
}

fn tips_for_season(season: &str) -> Vec<String> {
    match season {
        "Winter" => vec![
            "Get annual flu and COVID-19 vaccinations".into(),
            "Wash hands frequently to prevent respiratory infections".into(),
            "Ensure adequate vitamin D intake with reduced sunlight".into(),
            "Monitor for signs of seasonal depression".into(),
            "Keep indoor humidity 30-50% to reduce virus transmission".into(),
        ],
        "Spring" => vec![
            "Check tick precautions if spending time outdoors".into(),
            "Manage allergy symptoms early with antihistamines".into(),
            "Gradually increase exercise intensity to avoid injuries".into(),
            "Update vaccinations — spring is peak measles season".into(),
        ],
        "Summer" => vec![
            "Stay hydrated — dehydration increases risk of kidney stones and UTIs".into(),
            "Use insect repellent in mosquito-endemic areas".into(),
            "Avoid peak sun hours (10am-4pm) to prevent heatstroke".into(),
            "Practice safe food handling to prevent foodborne illness".into(),
            "Wear breathable clothing to prevent fungal skin infections".into(),
        ],
        "Autumn" => vec![
            "Get flu vaccination before peak season".into(),
            "Check for ticks after outdoor activities through October".into(),
            "Prepare for cold and flu season with good hand hygiene".into(),
            "Manage fall allergies (ragweed, mold spores)".into(),
        ],
        _ => vec![],
    }
}

pub fn run(conn: &Connection, month_input: Option<u32>, json: bool) {
    let current_month = month_input.unwrap_or_else(|| {
        chrono::Local::now().month()
    });

    if !(1..=12).contains(&current_month) {
        if json {
            println!("{{\"error\": \"Month must be 1-12\"}}");
        } else {
            println!("{} Month must be 1-12", "✗".red());
        }
        return;
    }

    let season = season_for_month(current_month);
    let data = get_seasonal_data();
    let tips = tips_for_season(season);

    // Find diseases active this month and verify they exist in the database
    let mut active: Vec<AlmanacDisease> = Vec::new();
    for entry in &data {
        if !entry.months.contains(&current_month) {
            continue;
        }
        // Look up severity from database
        let severity: Option<String> = conn.query_row(
            "SELECT severity FROM diseases WHERE name = ?1",
            [entry.disease],
            |row| row.get(0),
        ).ok();

        active.push(AlmanacDisease {
            name: entry.disease.to_string(),
            severity: severity.unwrap_or_else(|| "medium".into()),
            reason: entry.reason.to_string(),
        });
    }

    // Sort by severity
    active.sort_by(|a, b| {
        let sev_order = |s: &str| match s { "high" => 0, "medium" => 1, _ => 2 };
        sev_order(&a.severity).cmp(&sev_order(&b.severity))
    });

    if json {
        let result = AlmanacResult {
            month: month_name(current_month).to_string(),
            hemisphere: "Northern".into(),
            season: season.to_string(),
            diseases: active,
            tips,
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".into()));
        return;
    }

    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", format!("║         📅  Health Almanac — {} ({})             ║", month_name(current_month), season).bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_cyan());
    println!();

    if active.is_empty() {
        println!("  No specific seasonal risks tracked for this month.");
    } else {
        println!("{}", "Diseases with increased risk this month:".bold());
        println!();
        for d in &active {
            let sev_emoji = match d.severity.as_str() {
                "high" => "🔴",
                "medium" => "🟡",
                _ => "🟢",
            };
            println!("  {} {} — {}", sev_emoji, d.name.bold(), d.reason.dimmed());
        }
    }

    println!();
    println!("{}", "━━━ Seasonal Health Tips ━━━".bold());
    for tip in &tips {
        println!("  💡 {tip}");
    }
    println!();

    // Suppress unused conn warning for the look-up
    let _ = conn;
}

use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_season_mapping() {
        assert_eq!(season_for_month(1), "Winter");
        assert_eq!(season_for_month(4), "Spring");
        assert_eq!(season_for_month(7), "Summer");
        assert_eq!(season_for_month(10), "Autumn");
    }

    #[test]
    fn test_tips_not_empty() {
        assert!(!tips_for_season("Winter").is_empty());
        assert!(!tips_for_season("Summer").is_empty());
    }

    #[test]
    fn test_seasonal_data_valid_months() {
        for entry in get_seasonal_data() {
            for m in entry.months {
                assert!((1..=12).contains(m), "Invalid month in {}: {}", entry.disease, m);
            }
        }
    }
}
