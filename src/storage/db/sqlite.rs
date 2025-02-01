use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::Storage;

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
    fn store(&mut self, uri: axum::http::Uri, code: &str) -> std::result::Result<(), &'static str> {
        self.0
            .get()
            .map_err(|_| "failed to get a worker")?
            .execute(
                include_str!("scripts/sqlite/insert.sql"),
                (&code, &uri.to_string()),
            )
            .map_err(|_| "could not insert into sqlite")?;

        Ok(())
    }

    fn load(&self, code: String) -> std::result::Result<axum::http::Uri, &'static str> {
        let conn = self.0.get().map_err(|_| "failed to get a worker")?;

        let mut stmt = conn
            .prepare(include_str!("scripts/sqlite/select.sql"))
            .map_err(|_| "failed to prepare statement")?;

        let mut uris = stmt
            .query_map([code], |row| {
                row.get::<usize, String>(0)?
                    .parse()
                    .map_err(|_| rusqlite::Error::InvalidQuery)
            })
            .map_err(|_| "failed to run select (sqlite)")?;

        uris.next()
            .ok_or("no uri found")?
            .map_err(|_| "failed to get uri")
    }
}
