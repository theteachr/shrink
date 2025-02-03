use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::error::Error;
use url::Url;

use crate::{error, Storage};

pub struct Sqlite(Pool<SqliteConnectionManager>);

impl Sqlite {
    fn with_pool(manager: SqliteConnectionManager) -> Result<Self, Box<dyn Error>> {
        let pool = Pool::new(manager).map_err(|_| "failed to create pool")?;

        pool.get()?
            .execute(include_str!("scripts/schema.sql"), ())?;

        Ok(Self(pool))
    }

    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        Self::with_pool(SqliteConnectionManager::file(path))
    }
}

impl Default for Sqlite {
    fn default() -> Self {
        Self::with_pool(SqliteConnectionManager::memory()).expect("failed to create in-memory pool")
    }
}

impl Storage for Sqlite {
    fn store(&mut self, url: Url, code: &str) -> Result<(), error::Storage> {
        self.0
            .get()
            .map_err(|e| error::Storage::Internal(e.to_string()))?
            .execute(
                include_str!("scripts/sqlite/insert.sql"),
                (&code, &url.to_string()),
            )?;

        Ok(())
    }

    fn load(&mut self, code: &str) -> Result<Url, error::Load> {
        let conn = self
            .0
            .get()
            .map_err(|e| error::Load::Internal(e.to_string()))?; // FIXME

        let mut stmt = conn
            .prepare(include_str!("scripts/sqlite/select.sql"))
            .map_err(|e| error::Load::Internal(e.to_string()))?;

        let mut urls = stmt
            .query_map([code], |row| {
                row.get::<usize, String>(0)?
                    .parse()
                    .map_err(|_| rusqlite::Error::InvalidQuery)
            })
            .map_err(|e| error::Load::Internal(e.to_string()))?;

        urls.next()
            .ok_or(error::Load::NotFound)?
            .map_err(|e| error::Load::Internal(e.to_string()))
    }
}
