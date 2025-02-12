use url::Url;

use crate::{error, Slug, Storage};

pub trait Cache: Storage {
    fn get(&self, slug: &Slug) -> Result<Url, error::Load>;
    fn set(&self, url: &Url, slug: &Slug) -> Result<(), error::Storage>;
}

pub struct Cached<C: Cache, S: Storage> {
    pub cache: C,
    pub storage: S,
}

impl<C: Cache, S: Storage> Storage for Cached<C, S> {
    fn store(&mut self, url: Url, slug: &Slug) -> Result<(), error::Storage> {
        self.storage.store(url, slug)
    }

    fn load(&self, slug: &Slug) -> Result<Url, error::Load> {
        self.cache.load(slug).or_else(|_| {
            let url = self.storage.load(slug)?;

            if let Err(_) = self.cache.set(&url, slug) {
                eprintln!("Failed to store URL in cache");
            }

            Ok(url)
        })
    }
}
