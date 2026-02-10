use rusqlite::{Connection, Result};

pub fn create_database() -> Result<Connection> {
    let conn = Connection::open("users.db")?;
    println!("Database abierta correctamente");
    Ok(conn)
}