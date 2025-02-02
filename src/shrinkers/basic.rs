use std::io::BufRead;

use crate::{
    error,
    generators::{Counter, RB62},
    storage::{Memory, Postgres, Sqlite},
    Generator, Shrinker, Storage,
};
use url::Url;

#[derive(Default)]
pub struct Basic<G, S> {
    urls: S,
    codes: G,
}

impl Basic<Counter, Memory> {
    pub fn from_file(path: &str) -> Result<Basic<Counter, Memory>, Box<dyn std::error::Error>> {
        let f = std::fs::File::open(path).map_err(|_| "unable to open file")?;
        let reader = std::io::BufReader::new(f);

        let mut codes = Counter::default();
        let mut urls = Memory::default();

        for line in reader.lines() {
            let url = line
                .map_err(|_| "unable to read line")?
                .parse()
                .map_err(|_| "invalid url")?;

            let code = codes.generate(&url);
            urls.store(url, &code)?;
        }

        Ok(Self { urls, codes })
    }
}

impl Basic<RB62, Sqlite> {
    pub fn open(path: &str) -> Result<Basic<RB62, Sqlite>, &'static str> {
        Ok(Self {
            urls: Sqlite::open(path)?,
            codes: RB62::default(),
        })
    }
}

impl Basic<RB62, Postgres> {
    pub async fn new() -> Self {
        Self {
            urls: Postgres::connect("host=localhost user=postgres password=secret")
                .await
                .unwrap(),
            codes: RB62::default(),
        }
    }
}

impl<G: Generator, S: Storage> Shrinker for Basic<G, S> {
    fn shrink(&mut self, url: Url) -> Result<String, error::Internal> {
        let code = self.codes.generate(&url);
        self.urls.store(url, &code)?;

        Ok(code)
    }

    fn expand(&self, code: String) -> Result<Url, error::Load> {
        self.urls.load(code)
    }

    fn store_custom(&mut self, url: Url, code: String) -> Result<(), error::Storage> {
        self.urls.store(url, &code)
    }
}
