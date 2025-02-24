use crate::{validator::Code, Generator};

#[derive(Clone, Default)]
pub struct Counter(usize); // XXX: This is not `Send` (I think).

impl Generator for Counter {
    fn generate(&mut self, _: &url::Url) -> Code {
        self.0 += 1;
        Code::new(self.0.to_string())
    }
}
