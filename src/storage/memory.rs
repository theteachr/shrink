use axum::http::Uri;
use std::collections::HashMap;

use crate::Storage;

#[derive(Default)]
pub struct Memory(HashMap<String, Uri>);

impl Storage for Memory {
    fn store(&mut self, uri: Uri, code: String) -> Result<(), &'static str> {
        self.0.insert(code, uri); // TODO: Handle when `code` is already present.
        Ok(())
    }

    fn load(&self, code: String) -> Result<Uri, &'static str> {
        self.0.get(&code).cloned().ok_or("not found")
    }
}
