use std::collections::HashMap;
use url::Url;

use crate::{error, Slug, Storage};

#[derive(Default)]
pub struct Memory(HashMap<Slug, Url>);

impl Storage for Memory {
    fn store(&mut self, url: Url, slug: &Slug) -> Result<(), error::Storage> {
        match self.0.insert(slug.clone(), url) {
            Some(_) => Err(error::Storage::Duplicate),
            None => Ok(()),
        }
    }

    fn load(&self, slug: &Slug) -> Result<Url, error::Load> {
        self.0.get(slug).cloned().ok_or(error::Load::NotFound)
    }
}
