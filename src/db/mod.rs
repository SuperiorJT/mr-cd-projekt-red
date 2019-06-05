
use crate::model::user::User;
use crate::Result;
use crate::Error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
pub struct Database {
    pub pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn open() -> Result<Self> {
        let manager = SqliteConnectionManager::file("users.db");
        let pool = Pool::new(manager).map_err(|why| Error::R2D2(why))?;
        let db = Database { pool };

        User::db_create_table(&db)?;

        Ok(db)
    }
}