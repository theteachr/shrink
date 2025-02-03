use url::Url;

use crate::{error, Storage};

pub struct Cached<C: Storage, S: Storage> {
    pub cache: C,
    pub storage: S,
}

impl<C: Storage, S: Storage> Storage for Cached<C, S> {
    fn store(&mut self, url: Url, code: &str) -> Result<(), error::Storage> {
        self.storage.store(url, code)
    }

    fn load(&mut self, code: &str) -> Result<Url, error::Load> {
        self.cache.load(code).or_else(|_| {
            let url = self.storage.load(code)?;

            if let Err(_) = self.cache.store(url.clone(), code) {
                eprintln!("Failed to store URL in cache");
            }

            Ok(url)
        })
    }
}
