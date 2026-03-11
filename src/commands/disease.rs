use crate::display;
use rusqlite::Connection;

pub fn run(conn: &Connection, name: &str) {
    display::print_banner();

    let disease = conn.query_row(
        "SELECT id, name, description, severity, contagious, icd11_code FROM diseases WHERE name LIKE ?1",
        [format!("%{name}%")],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i32>(4)? != 0,
                row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            ))
        },
    );

    match disease {
        Ok((id, dname, desc, severity, contagious, icd11)) => {
            let mut stmt = conn
                .prepare(
                    "SELECT s.name, ds.weight, ds.is_primary FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.disease_id = ?1 ORDER BY ds.weight DESC",
                )
                .unwrap();
            let symptoms: Vec<(String, f64, bool)> = stmt
                .query_map([id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get::<_, i32>(2)? != 0))
                })
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            display::print_disease_info(&dname, &desc, &severity, contagious, &icd11, &symptoms);
        }
        Err(_) => {
            println!("Disease '{}' not found in database.", name);
            println!("Try: openhealth disease \"malaria\"");
        }
    }
}
