use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS diseases (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE,
            description TEXT NOT NULL,
            severity TEXT NOT NULL CHECK(severity IN ('low','medium','high')),
            contagious INTEGER NOT NULL DEFAULT 0,
            prevalence TEXT,
            icd11_code TEXT,
            age_group TEXT DEFAULT 'all',
            category TEXT DEFAULT 'general'
        );

        CREATE TABLE IF NOT EXISTS symptoms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE,
            description TEXT
        );

        CREATE TABLE IF NOT EXISTS disease_symptoms (
            disease_id INTEGER NOT NULL,
            symptom_id INTEGER NOT NULL,
            weight REAL NOT NULL DEFAULT 0.5,
            is_primary INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (disease_id, symptom_id),
            FOREIGN KEY (disease_id) REFERENCES diseases(id),
            FOREIGN KEY (symptom_id) REFERENCES symptoms(id)
        );

        CREATE TABLE IF NOT EXISTS treatments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            disease_id INTEGER NOT NULL,
            protocol TEXT NOT NULL,
            source TEXT DEFAULT 'WHO',
            first_aid TEXT,
            prevention TEXT,
            FOREIGN KEY (disease_id) REFERENCES diseases(id)
        );

        CREATE TABLE IF NOT EXISTS risk_factors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            disease_id INTEGER NOT NULL,
            factor TEXT NOT NULL,
            impact TEXT DEFAULT 'moderate',
            FOREIGN KEY (disease_id) REFERENCES diseases(id)
        );

        CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_create_tables_twice() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        create_tables(&conn).unwrap(); // idempotent
    }
}
