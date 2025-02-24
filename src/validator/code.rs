#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Code(pub(crate) String);

impl Code {
    pub(crate) fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
