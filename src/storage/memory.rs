use std::collections::HashMap;
use url::Url;

use crate::{error, Storage};

#[derive(Default)]
pub struct Memory(HashMap<String, Url>);

impl Storage for Memory {
    fn store(&mut self, url: Url, code: &str) -> Result<(), error::Storage> {
        match self.0.insert(code.to_owned(), url) {
            Some(_) => Err(error::Storage::Duplicate),
            None => Ok(()),
        }
    }

    fn load(&self, code: String) -> Result<Url, error::Load> {
        self.0.get(&code).cloned().ok_or(error::Load::NotFound)
    }
}
