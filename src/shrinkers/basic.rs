use std::io::BufRead;

use crate::{generators::Counter, storage::Memory, Generator, Shrinker, Storage};
use axum::http::Uri;

#[derive(Default)]
pub struct Basic {
    uris: Memory,
    counter: Counter,
}

impl Basic {
    pub fn from_file(path: &str) -> Result<Self, &'static str> {
        let f = std::fs::File::open(path).map_err(|_| "unable to open file")?;
        let reader = std::io::BufReader::new(f);
        let mut counter = Counter::default();
        let mut uris = Memory::default();

        for line in reader.lines() {
            let uri = line
                .map_err(|_| "unable to read line")?
                .parse()
                .map_err(|_| "invalid uri")?;

            let code = counter.generate(&uri)?;
            uris.store(uri, code)?;
        }

        Ok(Self { uris, counter })
    }
}

impl Shrinker for Basic {
    fn shrink(&mut self, uri: Uri) -> Result<String, &'static str> {
        let code = self.counter.generate(&uri)?;
        self.uris.store(uri, code.clone())?;

        Ok(code)
    }

    fn expand(&self, code: String) -> Result<Uri, &'static str> {
        self.uris.load(code)
    }
}
