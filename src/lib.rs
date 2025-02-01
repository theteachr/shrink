pub mod generators;
pub mod shrinkers;
pub mod storage;

// TODO: Create a type for URL / URI.
// This will remove dependency on axum.
use axum::http::Uri;

pub trait Shrinker {
    fn shrink(&mut self, uri: Uri) -> Result<String, &'static str>;
    fn expand(&self, code: String) -> Result<Uri, &'static str>;
    fn store_custom(&mut self, uri: Uri, code: String) -> Result<(), &'static str>;
}

trait Generator {
    fn generate(&mut self, uri: &Uri) -> Result<String, &'static str>;
}

trait Storage {
    fn store(&mut self, uri: Uri, code: String) -> Result<(), &'static str>;
    fn load(&self, code: String) -> Result<Uri, &'static str>;
}
