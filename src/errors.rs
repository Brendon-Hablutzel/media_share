use std::fmt::Display;

use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AppError {
    MultipartParseError(MultipartError),
    DatabaseError(sqlx::Error),
    IOError(std::io::Error),
    NotFound(String),
}

impl From<MultipartError> for AppError {
    fn from(value: MultipartError) -> Self {
        Self::MultipartParseError(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(inner) => write!(f, "database error: {inner}"),
            Self::MultipartParseError(inner) => write!(f, "bad multipart request: {inner}"),
            Self::IOError(inner) => write!(f, "IO error: {inner}"),
            Self::NotFound(details) => write!(f, "not found: {details}"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MultipartParseError(_) => StatusCode::BAD_REQUEST,
            Self::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        };

        let text = self.to_string();

        (status_code, text).into_response()
    }
}

impl std::error::Error for AppError {}
