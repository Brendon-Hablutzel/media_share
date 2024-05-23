use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse};

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

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            Self::MultipartParseError(inner) => (
                StatusCode::BAD_REQUEST,
                format!("bad multipart request: {inner}"),
            ),
            Self::DatabaseError(inner) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {inner}"),
            ),
            Self::IOError(inner) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("io error: {inner}"),
            ),
            Self::NotFound(details) => (StatusCode::NOT_FOUND, format!("not found: {details}")),
        };

        response.into_response()
    }
}
