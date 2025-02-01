use std::io::BufRead;

use crate::{
    generators::{Counter, RB62},
    storage::{Memory, Sqlite},
    Generator, Shrinker, Storage,
};
use axum::http::Uri;

#[derive(Default)]
pub struct Basic<G, S> {
    uris: S,
    codes: G,
}

impl Basic<Counter, Memory> {
    pub fn from_file(path: &str) -> Result<Basic<Counter, Memory>, &'static str> {
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

impl Basic<RB62, Sqlite> {
    pub fn open(path: &str) -> Result<Basic<RB62, Sqlite>, &'static str> {
        Ok(Self {
            uris: Sqlite::from_file(path)?,
            codes: RB62::default(),
        })
    }
}

impl<G: Generator, S: Storage> Shrinker for Basic<G, S> {
    fn shrink(&mut self, uri: Uri) -> Result<String, &'static str> {
        let code = self.codes.generate(&uri)?;
        self.uris.store(uri, code.clone())?;

        Ok(code)
    }

    fn expand(&self, code: String) -> Result<Uri, &'static str> {
        self.uris.load(code)
    }
}
