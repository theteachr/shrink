use std::io::BufRead;

use crate::{generators::Counter, storage::Memory, Generator, Shrinker, Storage};
use axum::http::Uri;

#[derive(Default)]
pub struct Basic<G> {
    uris: Memory,
    codes: G,
}

impl Basic<Counter> {
    pub fn from_file(path: &str) -> Result<Basic<Counter>, &'static str> {
        let f = std::fs::File::open(path).map_err(|_| "unable to open file")?;
        let reader = std::io::BufReader::new(f);
        let mut codes = Counter::default();
        let mut uris = Memory::default();

        for line in reader.lines() {
            let uri = line
                .map_err(|_| "unable to read line")?
                .parse()
                .map_err(|_| "invalid uri")?;

            let code = codes.generate(&uri)?;
            uris.store(uri, code)?;
        }

        Ok(Self { uris, codes })
    }
}

impl<G: Generator> Shrinker for Basic<G> {
    fn shrink(&mut self, uri: Uri) -> Result<String, &'static str> {
        let code = self.codes.generate(&uri)?;
        self.uris.store(uri, code.clone())?;

        Ok(code)
    }

    fn expand(&self, code: String) -> Result<Uri, &'static str> {
        self.uris.load(code)
    }
}
