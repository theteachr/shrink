pub mod error;
pub mod generators;
pub mod shrinkers;
pub mod storage;

// TODO: Create a type for URL / URI.
// This will remove dependency on axum.
use axum::http::Uri;

pub trait Shrinker {
    fn shrink(&mut self, uri: Uri) -> Result<String, error::Internal>;
    fn expand(&self, code: String) -> Result<Uri, error::Load>;
    fn store_custom(&mut self, uri: Uri, code: String) -> Result<(), error::Storage>;
}

trait Generator {
    fn generate(&mut self, uri: &Uri) -> String;
}

trait Storage {
    fn store(&mut self, uri: Uri, code: &str) -> Result<(), error::Storage>;
    fn load(&self, code: String) -> Result<Uri, error::Load>;
}
