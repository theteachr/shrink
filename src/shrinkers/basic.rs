use std::io::BufRead;

use crate::{
    error,
    generators::{Counter, RB62},
    storage::{Memory, Postgres, Sqlite},
    Generator, Shrinker, Storage,
};
use axum::http::Uri;

#[derive(Default)]
pub struct Basic<G, S> {
    uris: S,
    codes: G,
}

impl Basic<Counter, Memory> {
    pub fn from_file(path: &str) -> Result<Basic<Counter, Memory>, Box<dyn std::error::Error>> {
        let f = std::fs::File::open(path).map_err(|_| "unable to open file")?;
        let reader = std::io::BufReader::new(f);

        let mut codes = Counter::default();
        let mut uris = Memory::default();

        for line in reader.lines() {
            let uri = line
                .map_err(|_| "unable to read line")?
                .parse()
                .map_err(|_| "invalid uri")?;

            let code = codes.generate(&uri);
            uris.store(uri, &code)?;
        }

        Ok(Self { uris, codes })
    }
}

impl Basic<RB62, Sqlite> {
    pub fn open(path: &str) -> Result<Basic<RB62, Sqlite>, &'static str> {
        Ok(Self {
            uris: Sqlite::open(path)?,
            codes: RB62::default(),
        })
    }
}

impl Basic<RB62, Postgres> {
    pub async fn new() -> Self {
        Self {
            uris: Postgres::connect("host=localhost user=postgres password=secret")
                .await
                .unwrap(),
            codes: RB62::default(),
        }
    }
}

impl<G: Generator, S: Storage> Shrinker for Basic<G, S> {
    fn shrink(&mut self, uri: Uri) -> Result<String, error::Internal> {
        let code = self.codes.generate(&uri);
        self.uris.store(uri, &code)?;

        Ok(code)
    }

    fn expand(&self, code: String) -> Result<Uri, error::Load> {
        self.uris.load(code)
    }

    fn store_custom(&mut self, uri: Uri, code: String) -> Result<(), error::Storage> {
        self.uris.store(uri, &code)
    }
}
