use r2d2::Pool;
use redis::{Client, Commands};
use std::time::Duration;
use url::Url;

use crate::{error, Storage};

use super::Cache;

pub struct Redis {
    pool: Pool<Client>,
    expire: Duration,
}

impl Redis {
    pub fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.pool
            .get()?
            .set_ex::<_, _, ()>(key, value, self.expire.as_secs())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Url, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;

        let url: String = conn.get(key)?;
        let url: Url = url.parse()?;

        Ok(url)
    }
}

impl Cache for Redis {
    fn get(&self, code: &str) -> Result<Url, error::Load> {
        self.get(code).map_err(|_| error::Load::NotFound)
    }

    fn set(&self, url: &Url, code: &str) -> Result<(), error::Storage> {
        self.set(code, url.as_str())
            .map_err(|_| crate::error::Storage::Duplicate)
    }
}

impl Storage for Redis {
    fn store(&mut self, url: Url, code: &str) -> Result<(), error::Storage> {
        match self.get(code) {
            Ok(_) => Err(crate::error::Storage::Duplicate),
            Err(_) => {
                self.set(code, url.as_str())
                    .map_err(|e| error::Storage::Internal(e.to_string()))?;
                Ok(())
            }
        }
    }

    fn load(&self, code: &str) -> Result<Url, error::Load> {
        self.get(code).map_err(|_| error::Load::NotFound)
    }
}

impl Default for Redis {
    fn default() -> Self {
        let client = Client::open("redis://127.0.0.1/").expect("Invalid Redis URL");

        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(5))
            .build(client)
            .expect("Failed to create Redis pool");

        Self {
            pool,
            expire: Duration::from_secs(300),
        }
    }
}
