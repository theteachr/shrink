use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct AlreadyExists;
#[derive(Debug)]
pub struct NotFound;
#[derive(Debug)]
pub struct InternalError(pub String);

impl Display for AlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the code already exists")
    }
}

impl Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the code was not found")
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal error: {}", self.0)
    }
}

impl Error for AlreadyExists {}
impl Error for NotFound {}
impl Error for InternalError {}
