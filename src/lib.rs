pub mod app;
pub mod error;
pub mod generators;
mod slug;
pub mod storage;

pub use slug::Slug;
pub use slug::Validator;

use url::Url;

// NOTE: Maybe consider going all async?

pub trait Shrinker {
    fn shrink(&mut self, url: Url) -> Result<Slug, error::Internal>;
    fn expand(&self, slug: &Slug) -> Result<Url, error::Load>;
}

trait Generator {
    fn generate(&mut self, url: &Url) -> Slug;
}

pub trait Storage {
    fn store(&mut self, url: Url, slug: &Slug) -> Result<(), error::Storage>;
    fn load(&self, slug: &Slug) -> Result<Url, error::Load>;
}
