use colored::*;
use serde::Serialize;
use rusqlite::Connection;

#[derive(Serialize)]
struct SeverityStats {
    level: String,
    emoji: String,
    description: String,
    count: usize,
    examples: Vec<String>,
}

#[derive(Serialize)]
struct SeverityGuide {
    levels: Vec<SeverityStats>,
    total_diseases: usize,
}

/// Display severity classification guide with database statistics.
pub fn run(conn: &Connection, json: bool) {
    let mut levels = Vec::new();

    for (level, emoji, desc) in [
        ("low", "🟢", "Monitor at home — mild, self-care appropriate"),
        ("medium", "🟡", "See a doctor soon — needs medical attention"),
        ("high", "🔴", "Emergency — seek immediate medical help"),
    ] {
        let mut stmt = conn
            .prepare("SELECT name FROM diseases WHERE severity = ?1 ORDER BY name")
            .unwrap();
        let names: Vec<String> = stmt
            .query_map([level], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        let examples: Vec<String> = names.iter().take(5).cloned().collect();

        levels.push(SeverityStats {
            level: level.to_string(),
            emoji: emoji.to_string(),
            description: desc.to_string(),
            count: names.len(),
            examples,
        });
    }

    let total: usize = levels.iter().map(|l| l.count).sum();

    let guide = SeverityGuide {
        levels,
        total_diseases: total,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&guide).unwrap());
        return;
    }

    println!();
    println!("{}", "⚕️  Severity Classification Guide".bold().cyan());
    println!("{}", "═".repeat(50).cyan());
    println!();

    for lvl in &guide.levels {
        let pct = if total > 0 {
            (lvl.count as f64 / total as f64 * 100.0) as u32
        } else {
            0
        };
        let bar_len = (pct as usize) / 3;
        let bar = "█".repeat(bar_len);

        let colored_level = match lvl.level.as_str() {
            "high" => format!("{} HIGH", lvl.emoji).red().bold().to_string(),
            "medium" => format!("{} MEDIUM", lvl.emoji).yellow().bold().to_string(),
            _ => format!("{} LOW", lvl.emoji).green().bold().to_string(),
        };

        println!("  {}", colored_level);
        println!("  {}", lvl.description.dimmed());
        println!("  {} diseases ({}%)  {}", lvl.count, pct, bar);
        println!(
            "  Examples: {}",
            lvl.examples.join(", ").dimmed()
        );
        println!();
    }

    println!(
        "  Total diseases in database: {}",
        total.to_string().bold()
    );
    println!();
    println!(
        "{}",
        "  ⚠️  Severity ratings are general guidance. Always seek professional"
            .dimmed()
    );
    println!(
        "{}",
        "     medical advice for any health concern.".dimmed()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_severity_guide() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, false);
    }

    #[test]
    fn test_severity_guide_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, true);
    }
}
