
use crate::db::sqlite::Database;
use crate::{Result};
use rusqlite::{NO_PARAMS};
use serenity::model::prelude::UserId;
#[derive(Debug)]
pub struct User {
    /// The discord id of the user. Use this to access discord data.
    // NOTE: This is stored in the db as a string due to sqlite not supporting u64's.
    // It must be formatted to string when inserted and parsed when fetched
    pub discord_id: UserId,
}

impl User {
    pub fn new(discord_id: UserId) -> Self {
        Self { discord_id }
    }

    pub fn db_create_table(db: &Database) -> Result<usize> {
        let conn = db.pool.get()?;
        let usize = conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                discord_id TEXT PRIMARY KEY
            )",
            NO_PARAMS,
        )?;
        Ok(usize)
    }

    pub fn db_create(&self, db: &Database) -> Result<usize> {
        let conn = db.pool.get()?;
        let usize = conn.execute(
            "INSERT INTO users (discord_id) values (?1)",
            // Format to string since this u64's must be stored as strings
            &[&self.discord_id.to_string()],
        )?;
        Ok(usize)
    }

    pub fn db_read_by_id(db: &Database, user_id: UserId) -> Result<User> {
        let conn = db.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM users WHERE discord_id = (?1)")?;
        let mut user_iter = stmt.query_map(&[&user_id.to_string()], |row| {
            // Parse the value to a u64 since we have it stored as a string
            let discord_id_string: String = row.get(0)?;
            let discord_id = discord_id_string
                .parse::<u64>()
                .expect("Failed to parse discord_id");
            Ok(User {
                discord_id: UserId(discord_id),
            })
        })?;
        let user = match user_iter.next() {
            Some(user) => user?,
            None => Err(rusqlite::Error::QueryReturnedNoRows)?,
        };
        Ok(user)
    }
}