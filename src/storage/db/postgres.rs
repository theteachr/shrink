use axum::http::Uri;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use tokio::task::block_in_place;

use crate::{error, Storage};

pub struct Postgres(Pool<PostgresConnectionManager<NoTls>>);

impl Postgres {
    pub async fn connect(config: &str) -> Result<Self, &'static str> {
        let config = config.parse().map_err(|_| "bad config")?;

        // NOTE: Forced to depend on `tokio::task::block_in_place`!
        // The synchronous implementation of postgres client depends on `tokio`
        // but all it does is some `block_on` wrapping on all calls.
        block_in_place(move || {
            let manager = PostgresConnectionManager::new(config, NoTls);
            let pool = Pool::new(manager).map_err(|_| "failed to create pool")?;

            pool.get()
                .map_err(|_| "failed to get a worker")?
                .batch_execute(include_str!("scripts/postgres/schema.sql"))
                .map_err(|_| "valid schema")?;

            Ok(Self(pool))
        })
    }
}

impl Storage for Postgres {
    fn store(&mut self, uri: Uri, code: &str) -> Result<(), error::Storage> {
        block_in_place(move || {
            self.0
                .get()
                .map_err(|e| error::Storage::Internal(e.to_string()))?
                .execute(
                    include_str!("scripts/postgres/insert.sql"),
                    &[&code, &uri.to_string()],
                )
                // TODO: Check if it's a unique constraint violation
                .map_err(|e| error::Storage::Internal(e.to_string()))?;

            Ok(())
        })
    }

    fn load(&self, code: String) -> Result<Uri, error::Load> {
        block_in_place(move || {
            let mut conn = self
                .0
                .get()
                .map_err(|e| error::Load::Internal(e.to_string()))?;

            conn.query(include_str!("scripts/postgres/select.sql"), &[&code])
                .map_err(|e| error::Load::Internal(e.to_string()))? // TODO: Check for unique constraint violation
                .iter()
                .filter_map(|row| row.get::<usize, String>(0).parse::<Uri>().ok())
                .next()
                .ok_or(error::Load::NotFound)
        })
    }
}
