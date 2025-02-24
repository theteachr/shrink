pub mod app;
pub mod error;
pub mod generators;
pub mod storage;
pub mod validator;

use url::Url;
use validator::Code;

// NOTE: Maybe consider going all async?

pub trait Shrinker {
    fn shrink(&mut self, url: Url) -> Result<Code, error::Internal>;
    fn expand(&self, code: &Code) -> Result<Url, error::Load>;
}

trait Generator {
    fn generate(&mut self, url: &Url) -> Code;
}

pub trait Storage {
    fn store(&mut self, url: Url, code: &Code) -> Result<(), error::Storage>;
    fn load(&self, code: &Code) -> Result<Url, error::Load>;
}
