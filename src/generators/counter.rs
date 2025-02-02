use crate::Generator;

#[derive(Clone, Default)]
pub struct Counter(usize); // XXX: This is not `Send` (I think).

impl Generator for Counter {
    fn generate(&mut self, _: &url::Url) -> String {
        self.0 += 1;
        self.0.to_string()
    }
}
