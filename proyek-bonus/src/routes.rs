use crate::error::AppError;
use crate::model::Link;
use crate::shortcode::ShortCode;
use crate::state::AppState;
use crate::stats;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLink {
    pub target: String,
    pub custom_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub short_url: String,
    pub code: String,
    pub target: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/links", get(list_links).post(create_link))
        .route("/api/stats", get(all_stats))
        .route("/api/stats/{code}", get(one_stats))
        .route("/api/audit", get(audit))
        .route("/{code}", get(redirect))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn create_link(
    State(state): State<AppState>,
    Json(request): Json<CreateLink>,
) -> Result<(StatusCode, Json<CreateResponse>), AppError> {
    let target = normalize_target(&request.target, is_http_url)?;

    let code = if let Some(value) = request.custom_code.as_deref() {
        ShortCode::parse(value)?
    } else {
        ShortCode::generate()
    };

    let link = Link::new(code.clone(), target.clone());
    state.store.insert_link(link).await?;

    let response = CreateResponse {
        short_url: state.short_url(&code),
        code: code.to_string(),
        target,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn list_links(State(state): State<AppState>) -> Result<Json<Vec<Link>>, AppError> {
    Ok(Json(state.store.list_links().await?))
}

async fn redirect(
    State(state): State<AppState>,
    Path(value): Path<String>,
) -> Result<Redirect, AppError> {
    let code = ShortCode::parse(&value)?;
    let Some(link) = state.store.get_link(&code).await? else {
        return Err(AppError::NotFound);
    };

    if !link.can_redirect() {
        return Err(AppError::Gone);
    }

    state
        .click_tx
        .send(code.to_string())
        .await
        .map_err(|_| AppError::Internal("worker klik berhenti".to_string()))?;
    let _ = state.audit_tx.try_send(code.to_string());

    Ok(Redirect::temporary(&link.target))
}

async fn all_stats(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    let links = state.store.list_links().await?;
    let summary = stats::summarize(&links);
    let value = serde_json::to_value(summary)
        .map_err(|error| AppError::Internal(format!("gagal membuat statistik: {error}")))?;
    Ok(Json(value))
}

async fn one_stats(
    State(state): State<AppState>,
    Path(value): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let code = ShortCode::parse(&value)?;
    let Some(link) = state.store.get_link(&code).await? else {
        return Err(AppError::NotFound);
    };

    let view = stats::stats_for(&link);
    let value = serde_json::to_value(view)
        .map_err(|error| AppError::Internal(format!("gagal membuat statistik: {error}")))?;
    Ok(Json(value))
}

async fn audit(State(state): State<AppState>) -> Result<Json<Vec<String>>, AppError> {
    let entries = state
        .recent_clicks
        .lock()
        .map_err(|_| AppError::Internal("lock audit rusak".to_string()))?
        .clone();

    Ok(Json(entries))
}

pub fn normalize_target(target: &str, validator: fn(&str) -> bool) -> Result<String, AppError> {
    let value = target.trim();

    if !validator(value) {
        return Err(AppError::BadRequest(
            "target harus diawali http:// atau https://".to_string(),
        ));
    }

    Ok(value.to_string())
}

pub fn is_http_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_and_https() {
        assert!(normalize_target(" https://example.com ", is_http_url).is_ok());
        assert!(normalize_target("http://example.com", is_http_url).is_ok());
    }

    #[test]
    fn rejects_other_schemes() {
        assert!(normalize_target("ftp://example.com", is_http_url).is_err());
    }
}
