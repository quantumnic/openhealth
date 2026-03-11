use crate::display;
use rusqlite::Connection;

pub fn run(conn: &Connection, name: &str, json: bool) {
    let result = conn.query_row(
        "SELECT d.name, t.protocol, t.source, t.first_aid, t.prevention 
         FROM treatments t 
         JOIN diseases d ON d.id = t.disease_id 
         WHERE d.name LIKE ?1",
        [format!("%{name}%")],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?.unwrap_or_else(|| "WHO".to_string()),
                row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            ))
        },
    );

    match result {
        Ok((dname, protocol, source, first_aid, prevention)) => {
            if json {
                let obj = serde_json::json!({
                    "disease": dname,
                    "protocol": protocol,
                    "source": source,
                    "first_aid": first_aid,
                    "prevention": prevention,
                });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
                return;
            }
            display::print_banner();
            display::print_disclaimer();
            display::print_treatment(&dname, &protocol, &source, &first_aid, &prevention);
        }
        Err(_) => {
            if json {
                println!("null");
            } else {
                println!("Treatment for '{}' not found in database.", name);
                println!("Try: openhealth treatment \"malaria\"");
            }
        }
    }
}
