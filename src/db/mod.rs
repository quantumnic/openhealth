pub mod schema;
pub mod seed;

use rusqlite::Connection;
use std::path::Path;

/// Initialize the database: create schema and seed if empty.
pub fn init_database(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    schema::create_tables(&conn)?;
    seed::seed_if_empty(&conn)?;
    Ok(conn)
}

/// Initialize an in-memory database for testing.
#[cfg(test)]
pub fn init_memory_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    schema::create_tables(&conn)?;
    seed::seed_if_empty(&conn)?;
    Ok(conn)
}
