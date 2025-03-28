use std::io::BufRead;
use std::{error::Error, sync::Arc};

use crate::storage::Redis;
use crate::validator::{Alnum, Validator};
use crate::{
    error,
    generators::{Counter, RB62},
    storage::{Cache, Cached, Memory, Postgres, Sqlite},
    validator::Code,
    Generator, Shrinker, Storage,
};
use tokio::sync::RwLock;
use url::Url;

pub struct App<G, S> {
    pub urls: S,
    codes: G,
}

impl App<Counter, Memory> {
    pub fn from_file(path: &str) -> Result<App<Counter, Memory>, Box<dyn Error>> {
        let f = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(f);

        let mut codes = Counter::default();
        let mut urls = Memory::default();

        for line in reader.lines() {
            let url = line?.parse()?;
            let code = codes.generate(&url);
            urls.store(url, &code)?;
        }

        Ok(Self { urls, codes })
    }
}

impl App<RB62, Sqlite> {
    pub fn open(path: &str) -> Result<App<RB62, Sqlite>, Box<dyn Error>> {
        Ok(Self {
            urls: Sqlite::open(path)?,
            codes: RB62,
        })
    }
}

impl App<RB62, Postgres> {
    pub async fn new() -> Self {
        let config = "host=localhost user=postgres password=secret";

        Self {
            urls: Postgres::connect(config).await.unwrap(),
            codes: RB62,
        }
    }
}

impl<G: Generator, S: Storage> Shrinker for App<G, S> {
    fn shrink(&mut self, url: Url) -> Result<Code, error::Internal> {
        let mut code = self.codes.generate(&url);
        // In case there is a collision, generate a new code until it's unique.
        while self.urls.load(&code).is_ok() {
            code = self.codes.generate(&url);
        }
        self.urls.store(url, &code)?;
        Ok(code)
    }

    fn expand(&self, code: &Code) -> Result<Url, error::Load> {
        self.urls.load(code)
    }
}

impl<S: Storage, G> App<G, S> {
    pub fn with_cache<C: Cache>(self, cache: C) -> App<G, Cached<C, S>> {
        App {
            urls: Cached {
                cache,
                storage: self.urls,
            },
            codes: self.codes,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub app: Arc<RwLock<App<RB62, Cached<Redis, Sqlite>>>>,
    pub base_url: Url,
    pub validator: Arc<Validator<Alnum>>,
}

impl AppState {
    pub fn shrink_response(&self, code: &Code) -> Option<Url> {
        self.base_url.join(code.as_str()).ok()
    }
}
