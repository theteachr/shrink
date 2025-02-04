use std::error::Error;
use std::io::BufRead;

use crate::{
    error,
    generators::{Counter, RB62},
    storage::{Cache, Cached, Memory, Postgres, Sqlite},
    Generator, Shrinker, Slug, Storage,
};
use url::Url;

pub struct App<G, S> {
    pub urls: S,
    slugs: G,
}

impl App<Counter, Memory> {
    pub fn from_file(path: &str) -> Result<App<Counter, Memory>, Box<dyn Error>> {
        let f = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(f);

        let mut slugs = Counter::default();
        let mut urls = Memory::default();

        for line in reader.lines() {
            let url = line?.parse()?;
            let slug = slugs.generate(&url);
            urls.store(url, &slug)?;
        }

        Ok(Self { urls, slugs })
    }
}

impl App<RB62, Sqlite> {
    pub fn open(path: &str) -> Result<App<RB62, Sqlite>, Box<dyn Error>> {
        Ok(Self {
            urls: Sqlite::open(path)?,
            slugs: RB62::default(),
        })
    }
}

impl App<RB62, Postgres> {
    pub async fn new() -> Self {
        let config = "host=localhost user=postgres password=secret";

        Self {
            urls: Postgres::connect(config).await.unwrap(),
            slugs: RB62::default(),
        }
    }
}

impl<G: Generator, S: Storage> Shrinker for App<G, S> {
    fn shrink(&mut self, url: Url) -> Result<Slug, error::Internal> {
        let mut slug = self.slugs.generate(&url);
        // In case there is a collision, generate a new slug until it's unique.
        while let Ok(_) = self.urls.load(&slug) {
            slug = self.slugs.generate(&url);
        }
        self.urls.store(url, &slug)?;
        Ok(slug)
    }

    fn expand(&self, slug: &Slug) -> Result<Url, error::Load> {
        self.urls.load(slug)
    }
}

impl<S: Storage, G> App<G, S> {
    pub fn with_cache<C: Cache>(self, cache: C) -> App<G, Cached<C, S>> {
        App {
            urls: Cached {
                cache,
                storage: self.urls,
            },
            slugs: self.slugs,
        }
    }
}
