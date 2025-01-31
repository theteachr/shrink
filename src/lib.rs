use axum::http::Uri;
use std::collections::HashMap;

pub trait Shrinker {
    fn shrink(&mut self, uri: Uri) -> Result<String, &'static str>;
    fn expand(&self, code: String) -> Result<Uri, NotFound>;
}

trait Generator {
    fn generate(&mut self, uri: &Uri) -> Result<String, &'static str>;
}

#[derive(Clone)]
struct Counter(usize); // XXX: This is not `Send` (I think).

impl Default for Counter {
    fn default() -> Self {
        Counter(0)
    }
}

impl Generator for Counter {
    fn generate(&mut self, _: &Uri) -> Result<String, &'static str> {
        self.0 += 1;
        Ok(self.0.to_string())
    }
}

#[derive(Default, Clone)]
pub struct Basic {
    uris: HashMap<String, Uri>,
    counter: Counter,
}

impl Shrinker for Basic {
    fn shrink(&mut self, uri: Uri) -> Result<String, &'static str> {
        let code = self.counter.generate(&uri)?;
        self.uris.insert(code.clone(), uri);

        Ok(code)
    }

    fn expand(&self, code: String) -> Result<Uri, NotFound> {
        self.uris.get(&code).cloned().ok_or(NotFound)
    }
}

#[derive(Debug)]
pub struct NotFound;

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not found :<")
    }
}

impl std::error::Error for NotFound {}
