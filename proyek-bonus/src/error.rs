use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt;

/// Membuat `AppError::BadRequest` dengan format string.
#[macro_export]
macro_rules! bad_request {
    ($($arg:tt)*) => {
        $crate::error::AppError::BadRequest(format!($($arg)*))
    };
}

/// Error aplikasi yang bisa dikirim sebagai response HTTP.
#[derive(Debug)]
pub enum AppError {
    /// Input pengguna tidak valid.
    BadRequest(String),
    /// Data yang diminta tidak ada.
    NotFound,
    /// Kode pendek sudah dipakai.
    Conflict(String),
    /// Link sudah tidak aktif.
    Gone,
    /// Kesalahan internal yang tidak boleh membocorkan detail ke client.
    Internal(String),
    /// Kesalahan database.
    Database(sqlx::Error),
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Gone => StatusCode::GONE,
            Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::BadRequest(message) | Self::Conflict(message) | Self::Internal(message) => {
                message.clone()
            }
            Self::NotFound => "data tidak ditemukan".to_string(),
            Self::Gone => "link sudah tidak aktif".to_string(),
            Self::Database(error) => {
                eprintln!("database error: {error}");
                "terjadi kesalahan internal".to_string()
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(message) => write!(formatter, "bad request: {message}"),
            Self::NotFound => write!(formatter, "not found"),
            Self::Conflict(message) => write!(formatter, "conflict: {message}"),
            Self::Gone => write!(formatter, "gone"),
            Self::Internal(message) => write!(formatter, "internal: {message}"),
            Self::Database(error) => write!(formatter, "database: {error}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = ErrorBody {
            error: self.message(),
        };
        (status, Json(body)).into_response()
    }
}
