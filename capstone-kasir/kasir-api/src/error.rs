use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use kasir_core::KasirError;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Payment(String),
    Database(sqlx::Error),
    Internal(String),
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Payment(_) => StatusCode::BAD_GATEWAY,
            Self::Database(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BadRequest(pesan) | Self::NotFound(pesan) | Self::Conflict(pesan) => {
                pesan.clone()
            }
            Self::Payment(pesan) => pesan.clone(),
            Self::Internal(_) => "terjadi kesalahan internal".to_string(),
            Self::Database(error) => {
                tracing::error!(error = %error, "database error");
                "terjadi kesalahan database".to_string()
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(pesan) => write!(f, "bad request: {pesan}"),
            Self::NotFound(pesan) => write!(f, "not found: {pesan}"),
            Self::Conflict(pesan) => write!(f, "conflict: {pesan}"),
            Self::Payment(pesan) => write!(f, "payment: {pesan}"),
            Self::Database(error) => write!(f, "database: {error}"),
            Self::Internal(pesan) => write!(f, "internal: {pesan}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<KasirError> for AppError {
    fn from(error: KasirError) -> Self {
        match error {
            KasirError::InputTidakValid(pesan) => Self::BadRequest(pesan),
            KasirError::ItemTidakDitemukan(pesan) => Self::NotFound(pesan),
            KasirError::StatusTidakValid(pesan) => Self::Conflict(pesan),
            KasirError::PembayaranGagal(pesan) => Self::Payment(pesan),
            KasirError::Internal(pesan) => Self::Internal(pesan),
        }
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
