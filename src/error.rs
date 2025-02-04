use std::error::Error;
use std::fmt::Display;

use axum::response::IntoResponse;
use r2d2_postgres::postgres;
use r2d2_sqlite::rusqlite;

#[derive(Debug)]
pub struct Duplicate;
#[derive(Debug)]
pub struct NotFound;
#[derive(Debug)]
pub struct Internal(String);
#[derive(Debug)]
pub struct BadAlias;

#[derive(Debug)]
pub enum Storage {
    Duplicate,
    BadAlias,
    Internal(String),
}

#[derive(Debug)]
pub enum Load {
    NotFound,
    BadAlias,
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
            Storage::BadAlias => write!(f, "bad alias"),
        }
    }
}

// TODO: Remove duplication (bad alias, internal error)

impl Display for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Load::NotFound => NotFound.fmt(f),
            Load::Internal(msg) => write!(f, "internal load error: {}", msg),
            Load::BadAlias => write!(f, "bad alias"),
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
            Storage::BadAlias => Internal("bad alias".to_string()),
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

impl IntoResponse for Storage {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            Storage::Duplicate => axum::http::Response::builder()
                .status(axum::http::StatusCode::CONFLICT)
                .body("code already used".into())
                .unwrap(),
            Storage::Internal(_) => axum::http::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("internal error".into())
                .unwrap(),
            Storage::BadAlias => axum::http::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("bad alias".into())
                .unwrap(),
        }
    }
}

impl IntoResponse for Internal {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(self.0.into())
            .unwrap()
    }
}

impl IntoResponse for Load {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            Load::NotFound => axum::http::Response::builder()
                .status(axum::http::StatusCode::NOT_FOUND)
                .body("shrunk code not found".into())
                .unwrap(),
            Load::Internal(_) => axum::http::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("internal error".into())
                .unwrap(),
            Load::BadAlias => axum::http::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("bad alias".into())
                .unwrap(),
        }
    }
}
