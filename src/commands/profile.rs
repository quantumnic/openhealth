use colored::*;
use rusqlite::Connection;

pub fn run(conn: &Connection, age: Option<u8>, sex: Option<&str>, show: bool, clear: bool, json: bool) {
    if clear {
        conn.execute("DELETE FROM metadata WHERE key IN ('profile_age', 'profile_sex')", []).ok();
        if json {
            println!("{{\"status\": \"cleared\"}}");
        } else {
            println!("{}", "✅ Profile cleared.".green());
        }
        return;
    }

    if let Some(a) = age {
        conn.execute(
            "INSERT OR REPLACE INTO metadata (key, value) VALUES ('profile_age', ?1)",
            [a.to_string()],
        ).ok();
    }

    if let Some(s) = sex {
        let s = s.to_lowercase();
        if s != "male" && s != "female" {
            if json {
                println!("{{\"error\": \"sex must be 'male' or 'female'\"}}");
            } else {
                eprintln!("{}", "Sex must be 'male' or 'female'.".red());
            }
            return;
        }
        conn.execute(
            "INSERT OR REPLACE INTO metadata (key, value) VALUES ('profile_sex', ?1)",
            [&s],
        ).ok();
    }

    if show || (age.is_none() && sex.is_none()) {
        let current_age = get_profile_age(conn);
        let current_sex = get_profile_sex(conn);

        if json {
            let obj = serde_json::json!({
                "age": current_age,
                "sex": current_sex,
            });
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        } else {
            println!("{}", "━━━ User Profile ━━━".bold());
            println!(
                "  Age: {}",
                current_age.map_or("not set".dimmed().to_string(), |a| a.to_string())
            );
            println!(
                "  Sex: {}",
                current_sex.as_deref().unwrap_or(&"not set".dimmed().to_string())
            );
            println!();
            println!("  Set with: openhealth profile --age 35 --sex male");
        }
        return;
    }

    if json {
        println!("{{\"status\": \"updated\"}}");
    } else {
        println!("{}", "✅ Profile updated.".green());
    }
}

pub fn get_profile_age(conn: &Connection) -> Option<u8> {
    conn.query_row(
        "SELECT value FROM metadata WHERE key = 'profile_age'",
        [],
        |row| row.get::<_, String>(0),
    )
    .ok()
    .and_then(|v| v.parse().ok())
}

pub fn get_profile_sex(conn: &Connection) -> Option<String> {
    conn.query_row(
        "SELECT value FROM metadata WHERE key = 'profile_sex'",
        [],
        |row| row.get::<_, String>(0),
    )
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_profile_set_and_get() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, Some(30), Some("male"), false, false, false);
        assert_eq!(get_profile_age(&conn), Some(30));
        assert_eq!(get_profile_sex(&conn), Some("male".to_string()));
    }

    #[test]
    fn test_profile_clear() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, Some(25), Some("female"), false, false, false);
        run(&conn, None, None, false, true, false);
        assert_eq!(get_profile_age(&conn), None);
        assert_eq!(get_profile_sex(&conn), None);
    }

    #[test]
    fn test_profile_show_empty() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, None, None, true, false, false);
    }

    #[test]
    fn test_profile_json() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, Some(40), Some("female"), false, false, true);
        run(&conn, None, None, true, false, true);
    }

    #[test]
    fn test_profile_invalid_sex() {
        let conn = db::init_memory_database().unwrap();
        run(&conn, None, Some("other"), false, false, false);
    }
}
