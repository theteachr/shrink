use url::Url;

use crate::{error, Code, Storage};

pub trait Cache: Storage {
    fn get(&self, code: &Code) -> Result<Url, error::Load>;
    fn set(&self, url: &Url, code: &Code) -> Result<(), error::Storage>;
}

pub struct Cached<C: Cache, S: Storage> {
    pub cache: C,
    pub storage: S,
}

impl<C: Cache, S: Storage> Storage for Cached<C, S> {
    fn store(&mut self, url: Url, code: &Code) -> Result<(), error::Storage> {
        self.storage.store(url, code)
    }

    fn load(&self, code: &Code) -> Result<Url, error::Load> {
        self.cache.load(code).or_else(|_| {
            let url = self.storage.load(code)?;

            if let Err(_) = self.cache.set(&url, code) {
                eprintln!("Failed to store URL in cache");
            }

            Ok(url)
        })
    }
}
