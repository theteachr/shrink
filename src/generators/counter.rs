use crate::Generator;
use axum::http::Uri;

#[derive(Clone, Default)]
pub struct Counter(usize); // XXX: This is not `Send` (I think).

impl Generator for Counter {
    fn generate(&mut self, _: &Uri) -> Result<String, &'static str> {
        self.0 += 1;
        Ok(self.0.to_string())
    }
}
