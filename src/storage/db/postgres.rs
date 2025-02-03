use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use tokio::task::block_in_place;
use url::Url;

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
    fn store(&mut self, url: Url, code: &str) -> Result<(), error::Storage> {
        block_in_place(move || {
            self.0
                .get()
                .map_err(|e| error::Storage::Internal(e.to_string()))?
                .execute(
                    include_str!("scripts/postgres/insert.sql"),
                    &[&code, &url.to_string()],
                )?;

            Ok(())
        })
    }

    fn load(&mut self, code: &str) -> Result<Url, error::Load> {
        block_in_place(move || {
            let mut conn = self
                .0
                .get()
                .map_err(|e| error::Load::Internal(e.to_string()))?;

            conn.query(include_str!("scripts/postgres/select.sql"), &[&code])
                .map_err(|e| error::Load::Internal(e.to_string()))?
                .iter()
                .filter_map(|row| row.get::<usize, String>(0).parse::<Url>().ok())
                .next()
                .ok_or(error::Load::NotFound)
        })
    }
}
