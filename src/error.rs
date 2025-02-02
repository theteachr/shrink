use std::error::Error;
use std::fmt::Display;

use r2d2_postgres::postgres;
use r2d2_sqlite::rusqlite;

#[derive(Debug)]
pub struct Duplicate;
#[derive(Debug)]
pub struct NotFound;
#[derive(Debug)]
pub struct Internal(String);

#[derive(Debug)]
pub enum Storage {
    Duplicate,
    Internal(String),
}

#[derive(Debug)]
pub enum Load {
    NotFound,
    Internal(String),
}

impl Display for Duplicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "duplicate entry")
    }
}

impl Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not found")
    }
}

impl Display for Internal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal error: {}", self.0)
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Storage::Duplicate => NotFound.fmt(f),
            Storage::Internal(msg) => write!(f, "internal storage error: {}", msg),
        }
    }
}

impl Display for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Load::NotFound => NotFound.fmt(f),
            Load::Internal(msg) => write!(f, "internal load error: {}", msg),
        }
    }
}

impl Error for Duplicate {}
impl Error for NotFound {}
impl Error for Internal {}
impl Error for Storage {}
impl Error for Load {}

impl From<Storage> for Internal {
    fn from(err: Storage) -> Self {
        match err {
            Storage::Duplicate => Internal("duplicate entry".to_string()),
            Storage::Internal(msg) => Internal(msg),
        }
    }
}

impl From<postgres::Error> for Storage {
    fn from(err: postgres::Error) -> Self {
        match err.code().cloned() {
            Some(postgres::error::SqlState::UNIQUE_VIOLATION) => Storage::Duplicate,
            _ => Storage::Internal(err.to_string()),
        }
    }
}

impl From<rusqlite::Error> for Storage {
    fn from(err: rusqlite::Error) -> Self {
        match err.sqlite_error().map(|e| e.code) {
            Some(rusqlite::ErrorCode::ConstraintViolation) => Storage::Duplicate,
            _ => Storage::Internal(err.to_string()),
        }
    }
}
