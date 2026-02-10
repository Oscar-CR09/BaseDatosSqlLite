use rusqlite::{params, Connection, Result};
use crate::models::user::User;

pub struct UserRepository<'a> {
    conn: &'a Connection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(&self, name: &str) -> Result<User> {
        self.conn.execute(
            "INSERT INTO users (name) VALUES (?1)",
            params![name],
        )?;

        let id = self.conn.last_insert_rowid() as i32;

        Ok(User {
            id,
            name: name.to_string(),
        })
    }

    pub fn find_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name FROM users WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![user_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(User {
                id: row.get(0)?,
                name: row.get(1)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn find_all(&self) -> Result<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name FROM users ORDER BY id",
        )?;

        let users_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut users = Vec::new();
        for user in users_iter {
            users.push(user?);
        }

        Ok(users)
    }

    pub fn update_name(&self, user_id: i32, new_name: &str) -> Result<bool> {
        let rows = self.conn.execute(
            "UPDATE users SET name = ?1 WHERE id = ?2",
            params![new_name, user_id],
        )?;

        Ok(rows > 0)
    }

    pub fn delete(&self, user_id: i32) -> Result<bool> {
        let rows = self.conn.execute(
            "DELETE FROM users WHERE id = ?1",
            params![user_id],
        )?;

        Ok(rows > 0)
    }
}