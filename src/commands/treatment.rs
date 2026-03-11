use crate::display;
use rusqlite::Connection;

pub fn run(conn: &Connection, name: &str) {
    display::print_banner();
    display::print_disclaimer();

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
            display::print_treatment(&dname, &protocol, &source, &first_aid, &prevention);
        }
        Err(_) => {
            println!("Treatment for '{}' not found in database.", name);
            println!("Try: openhealth treatment \"malaria\"");
        }
    }
}
