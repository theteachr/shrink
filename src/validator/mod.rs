mod alnum;
mod code;
mod default;

pub use alnum::Alnum;
pub use code::Code;
pub use default::DefaultValidator;

pub trait Validate {
    fn validate(&self, code: &str) -> bool;
}

pub struct Validator<T: Validate>(T);

impl<T: Validate> Validator<T> {
    pub fn new(validator: T) -> Self {
        Self(validator)
    }
}

impl<T: Validate> Validator<T> {
    pub fn validate(&self, s: String) -> Option<Code> {
        self.0.validate(&s).then(|| Code(s))
    }
}
