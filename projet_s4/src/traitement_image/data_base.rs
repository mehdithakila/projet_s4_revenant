use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Face {
    name: String,
    features: Vec<u8>,
}

pub fn create_db() -> Result<()> {
    let conn = Connection::open("faces.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS faces (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  features BLOB NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

pub fn save_face(name: &str, features: Vec<u8>) -> Result<()> {
    let conn = Connection::open("faces.db")?;
    conn.execute(
        "INSERT INTO faces (name, features) VALUES (?1, ?2)",
        params![name, features],
    )?;
    Ok(())
}

