use rusqlite::Connection;

use crate::Storage;

pub struct Sqlite(Connection);

impl Sqlite {
    pub fn from_file(path: &str) -> Result<Self, &'static str> {
        let conn = Connection::open(path).map_err(|_| "sqlite to always open")?;

        conn.execute(include_str!("scripts/schema.sql"), ())
            .expect("valid schema");

        Ok(Self(conn))
    }
}

impl Default for Sqlite {
    fn default() -> Self {
        let conn = Connection::open_in_memory().expect("sqlite to always open in memory");

        conn.execute(include_str!("scripts/schema.sql"), ())
            .expect("valid schema");

        Self(conn)
    }
}

impl Storage for Sqlite {
    fn store(
        &mut self,
        uri: axum::http::Uri,
        code: String,
    ) -> std::result::Result<(), &'static str> {
        self.0
            .execute(
                include_str!("scripts/insert.sql"),
                (&uri.to_string(), &code),
            )
            .map_err(|_| "could not insert into sqlite")?;

        Ok(())
    }

    fn load(&self, code: String) -> std::result::Result<axum::http::Uri, &'static str> {
        let mut stmt = self
            .0
            .prepare(include_str!("scripts/select.sql"))
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
