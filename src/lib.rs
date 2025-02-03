pub mod app;
pub mod error;
pub mod generators;
pub mod storage;

use url::Url;

// NOTE: Maybe consider going all async?

pub trait Shrinker {
    fn shrink(&mut self, url: Url) -> Result<String, error::Internal>;
    fn expand(&self, code: &str) -> Result<Url, error::Load>;
    fn store_custom(&mut self, url: Url, code: &str) -> Result<(), error::Storage>;
}

trait Generator {
    fn generate(&mut self, url: &Url) -> String;
}

trait Storage {
    fn store(&mut self, url: Url, code: &str) -> Result<(), error::Storage>;
    fn load(&self, code: &str) -> Result<Url, error::Load>;
}
