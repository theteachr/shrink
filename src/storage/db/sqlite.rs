use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use url::Url;

use crate::{error, Storage};

pub struct Sqlite(Pool<SqliteConnectionManager>);

impl Sqlite {
    pub fn open(path: &str) -> Result<Self, &'static str> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager).map_err(|_| "failed to create pool")?;

        pool.get()
            .map_err(|_| "failed to get a worker")?
            .execute(include_str!("scripts/sqlite/schema.sql"), ())
            .expect("valid schema");

        Ok(Self(pool))
    }
}

impl Default for Sqlite {
    fn default() -> Self {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).unwrap();

        pool.get()
            .unwrap()
            .execute(include_str!("scripts/sqlite/schema.sql"), ())
            .expect("valid schema");

        Self(pool)
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
            )
            .map_err(|e| error::Storage::Internal(e.to_string()))?; // TODO: Check for whether the error is unique constraint violation

        Ok(())
    }

    fn load(&self, code: String) -> Result<Url, error::Load> {
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
