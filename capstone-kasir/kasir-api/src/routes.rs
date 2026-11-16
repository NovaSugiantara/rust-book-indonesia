use crate::error::AppError;
use crate::models::{
    CreateMenuRequest, CreateOrderRequest, EventResponse, MenuResponse, OrderResponse, PayRequest,
    ReportResponse, StatusRequest, UpdateMenuRequest,
};
use crate::state::{Notifikasi, SharedState};
use crate::store;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use kasir_core::StatusPesanan;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub tanggal: Option<String>,
}

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/menu", get(list_menu).post(create_menu))
        .route(
            "/api/menu/{kode}",
            get(get_menu).put(update_menu).delete(delete_menu),
        )
        .route("/api/orders", post(create_order))
        .route("/api/orders/{id}", get(get_order))
        .route("/api/orders/{id}/pay", post(pay_order))
        .route("/api/orders/{id}/status", patch(update_status))
        .route("/api/reports/daily", get(daily_report))
        .route("/api/events", get(list_events))
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "kasir-warung" }))
}

async fn list_menu(State(state): State<SharedState>) -> Result<Json<Vec<MenuResponse>>, AppError> {
    Ok(Json(store::list_menu(&state.pool).await?))
}

async fn get_menu(
    State(state): State<SharedState>,
    Path(kode): Path<String>,
) -> Result<Json<MenuResponse>, AppError> {
    Ok(Json(store::get_menu(&state.pool, &kode).await?))
}

async fn create_menu(
    State(state): State<SharedState>,
    Json(request): Json<CreateMenuRequest>,
) -> Result<(StatusCode, Json<MenuResponse>), AppError> {
    let menu = store::create_menu(&state.pool, request).await?;
    Ok((StatusCode::CREATED, Json(menu)))
}

async fn update_menu(
    State(state): State<SharedState>,
    Path(kode): Path<String>,
    Json(request): Json<UpdateMenuRequest>,
) -> Result<Json<MenuResponse>, AppError> {
    Ok(Json(store::update_menu(&state.pool, &kode, request).await?))
}

async fn delete_menu(
    State(state): State<SharedState>,
    Path(kode): Path<String>,
) -> Result<StatusCode, AppError> {
    store::delete_menu(&state.pool, &kode).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_order(
    State(state): State<SharedState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    let order = store::create_order(&state.pool, request).await?;
    Ok((StatusCode::CREATED, Json(order)))
}

async fn get_order(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<OrderResponse>, AppError> {
    Ok(Json(store::get_order(&state.pool, id).await?))
}

async fn pay_order(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(request): Json<PayRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = store::pay_order(&state.pool, id, &request.metode).await?;

    let notifikasi = Notifikasi {
        order_id: id,
        pesan: format!(
            "Pesanan {} dibayar dengan {} sebesar Rp{}",
            id,
            order.metode.as_deref().unwrap_or("tidak diketahui"),
            order.total
        ),
    };

    if state.notif_tx.send(notifikasi).await.is_err() {
        tracing::warn!(order_id = id, "worker notifikasi tidak tersedia");
    }

    Ok(Json(order))
}

async fn update_status(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(request): Json<StatusRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    let target = StatusPesanan::parse(&request.status)?;
    Ok(Json(store::update_status(&state.pool, id, target).await?))
}

async fn daily_report(
    State(state): State<SharedState>,
    Query(query): Query<ReportQuery>,
) -> Result<Json<ReportResponse>, AppError> {
    Ok(Json(
        store::daily_report(&state.pool, query.tanggal.as_deref()).await?,
    ))
}

async fn list_events(
    State(state): State<SharedState>,
) -> Result<Json<Vec<EventResponse>>, AppError> {
    Ok(Json(store::list_events(&state.pool).await?))
}
