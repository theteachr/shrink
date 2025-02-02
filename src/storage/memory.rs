use axum::http::Uri;
use std::collections::HashMap;

use crate::{error, Storage};

#[derive(Default)]
pub struct Memory(HashMap<String, Uri>);

impl Storage for Memory {
    fn store(&mut self, uri: Uri, code: &str) -> Result<(), error::Storage> {
        match self.0.insert(code.to_owned(), uri) {
            Some(_) => Err(error::Storage::Duplicate),
            None => Ok(()),
        }
    }

    fn load(&self, code: String) -> Result<Uri, error::Load> {
        self.0.get(&code).cloned().ok_or(error::Load::NotFound)
    }
}
