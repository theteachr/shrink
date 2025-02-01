use axum::http::Uri;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

use crate::Storage;

pub struct Postgres(Pool<PostgresConnectionManager<NoTls>>);

impl Postgres {
    pub fn connect(config: &str) -> Result<Self, &'static str> {
        let manager =
            PostgresConnectionManager::new(config.parse().map_err(|_| "bad config")?, NoTls);
        let pool = Pool::new(manager).map_err(|_| "failed to create pool")?;

        pool.get()
            .map_err(|_| "failed to get a worker")?
            .batch_execute(include_str!("scripts/schema.sql"))
            .expect("valid schema");

        Ok(Self(pool))
    }
}

impl Storage for Postgres {
    fn store(&mut self, uri: axum::http::Uri, code: &str) -> std::result::Result<(), &'static str> {
        self.0
            .get()
            .map_err(|_| "failed to get a worker")?
            .execute(
                include_str!("scripts/insert.sql"),
                &[&code, &uri.to_string()],
            )
            .map_err(|_| "could not insert into sqlite")?;

        Ok(())
    }

    fn load(&self, code: String) -> std::result::Result<axum::http::Uri, &'static str> {
        let mut conn = self.0.get().map_err(|_| "failed to get a worker")?;

        conn.query(include_str!("scripts/select.sql"), &[&code])
            .map_err(|_| "failed to query postgres")?
            .iter()
            .filter_map(|row| row.get::<usize, String>(0).parse::<Uri>().ok())
            .next()
            .ok_or("could not find uri")
    }
}

impl Default for Postgres {
    fn default() -> Self {
        match Postgres::connect("host=localhost user=postgres password=secret") {
            Ok(client) => client,
            Err(e) => panic!("{}", e),
        }
    }
}
