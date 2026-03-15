use colored::*;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HealthSummary {
    total_diseases: i64,
    total_symptoms: i64,
    categories: Vec<CategorySummary>,
    severity_breakdown: SeverityBreakdown,
    top_risk_factors: Vec<RiskFactorSummary>,
}

#[derive(Debug, Serialize)]
struct CategorySummary {
    category: String,
    count: i64,
}

#[derive(Debug, Serialize)]
struct SeverityBreakdown {
    high: i64,
    medium: i64,
    low: i64,
}

#[derive(Debug, Serialize)]
struct RiskFactorSummary {
    factor: String,
    disease_count: i64,
}

pub fn run(conn: &Connection, json: bool) {
    let total_diseases: i64 = conn
        .query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0))
        .unwrap_or(0);

    let total_symptoms: i64 = conn
        .query_row("SELECT COUNT(*) FROM symptoms", [], |r| r.get(0))
        .unwrap_or(0);

    // Category breakdown
    let mut cat_stmt = conn
        .prepare(
            "SELECT COALESCE(category, 'general') as cat, COUNT(*) as cnt
             FROM diseases GROUP BY cat ORDER BY cnt DESC",
        )
        .unwrap();
    let categories: Vec<CategorySummary> = cat_stmt
        .query_map([], |row| {
            Ok(CategorySummary {
                category: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    // Severity breakdown
    let high: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM diseases WHERE severity = 'high'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let medium: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM diseases WHERE severity = 'medium'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let low: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM diseases WHERE severity = 'low'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let severity_breakdown = SeverityBreakdown { high, medium, low };

    // Top risk factors (most common across diseases)
    let mut rf_stmt = conn
        .prepare(
            "SELECT LOWER(factor), COUNT(DISTINCT disease_id) as cnt
             FROM risk_factors
             GROUP BY LOWER(factor)
             ORDER BY cnt DESC
             LIMIT 15",
        )
        .unwrap();
    let top_risk_factors: Vec<RiskFactorSummary> = rf_stmt
        .query_map([], |row| {
            Ok(RiskFactorSummary {
                factor: row.get(0)?,
                disease_count: row.get(1)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    if json {
        let summary = HealthSummary {
            total_diseases,
            total_symptoms,
            categories,
            severity_breakdown,
            top_risk_factors,
        };
        println!("{}", serde_json::to_string_pretty(&summary).unwrap());
        return;
    }

    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗"
            .bright_cyan()
    );
    println!(
        "{}",
        "║          🏥  OpenHealth Database Summary  🏥           ║"
            .bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝"
            .bright_cyan()
    );
    println!();

    println!(
        "  📊 {} diseases  •  {} symptoms tracked",
        total_diseases.to_string().bright_white().bold(),
        total_symptoms.to_string().bright_white().bold()
    );
    println!();

    // Severity breakdown
    println!("  {}", "Severity Distribution:".bold());
    let total = (high + medium + low) as f64;
    if total > 0.0 {
        let h_pct = (high as f64 / total * 100.0) as u32;
        let m_pct = (medium as f64 / total * 100.0) as u32;
        let l_pct = (low as f64 / total * 100.0) as u32;
        println!(
            "    🔴 High:   {:>3} ({:>2}%)  {}",
            high,
            h_pct,
            "█".repeat((h_pct as usize) / 2).red()
        );
        println!(
            "    🟡 Medium: {:>3} ({:>2}%)  {}",
            medium,
            m_pct,
            "█".repeat((m_pct as usize) / 2).yellow()
        );
        println!(
            "    🟢 Low:    {:>3} ({:>2}%)  {}",
            low,
            l_pct,
            "█".repeat((l_pct as usize) / 2).green()
        );
    }
    println!();

    // Top categories
    println!("  {}", "Top Categories:".bold());
    for (i, cat) in categories.iter().take(10).enumerate() {
        let bar_len = (cat.count as usize).min(30);
        println!(
            "    {:>2}. {:<25} {:>3}  {}",
            i + 1,
            cat.category.bright_white(),
            cat.count,
            "▓".repeat(bar_len).bright_blue()
        );
    }
    if categories.len() > 10 {
        println!(
            "        ... and {} more categories",
            categories.len() - 10
        );
    }
    println!();

    // Top risk factors
    println!("  {}", "Most Common Risk Factors:".bold());
    for rf in top_risk_factors.iter().take(10) {
        println!(
            "    ⚠️  {:<40} ({} diseases)",
            rf.factor.bright_yellow(),
            rf.disease_count
        );
    }
    println!();
    println!(
        "  {}",
        "Use 'openhealth stats' for detailed statistics.".dimmed()
    );
    println!();
}
