use std::collections::HashMap;
use url::Url;

use crate::{error, Code, Storage};

#[derive(Default)]
pub struct Memory(HashMap<Code, Url>);

impl Storage for Memory {
    fn store(&mut self, url: Url, code: &Code) -> Result<(), error::Storage> {
        match self.0.insert(code.clone(), url) {
            Some(_) => Err(error::Storage::Duplicate),
            None => Ok(()),
        }
    }

    fn load(&self, code: &Code) -> Result<Url, error::Load> {
        self.0.get(code).cloned().ok_or(error::Load::NotFound)
    }
}
