use crate::Generator;
use axum::http::Uri;

#[derive(Clone)]
pub struct Counter(usize); // XXX: This is not `Send` (I think).

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
