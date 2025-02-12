use crate::{Generator, Slug};

#[derive(Clone, Default)]
pub struct Counter(usize); // XXX: This is not `Send` (I think).

impl Generator for Counter {
    fn generate(&mut self, _: &url::Url) -> Slug {
        self.0 += 1;
        Slug::new(self.0.to_string())
    }
}
