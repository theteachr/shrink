pub mod error;
pub mod generators;
pub mod shrinkers;
pub mod storage;

use url::Url;

pub trait Shrinker {
    fn shrink(&mut self, url: Url) -> Result<String, error::Internal>;
    fn expand(&self, code: &str) -> Result<Url, error::Load>;
    fn store_custom(&mut self, uri: Url, code: &str) -> Result<(), error::Storage>;
}

trait Generator {
    fn generate(&mut self, uri: &Url) -> String;
}

trait Storage {
    fn store(&mut self, uri: Url, code: &str) -> Result<(), error::Storage>;
    fn load(&self, code: &str) -> Result<Url, error::Load>;
}
