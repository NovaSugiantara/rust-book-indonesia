use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use kasir_api::routes::router;
use kasir_api::state::{AppState, SharedState};
use serde_json::{Value, json};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tokio::sync::mpsc;
use tower::ServiceExt;

async fn test_state() -> SharedState {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("database test");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrasi test");

    let (notif_tx, _notif_rx) = mpsc::channel(16);
    Arc::new(AppState { pool, notif_tx })
}

async fn json_body(response: axum::response::Response) -> Value {
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body response");
    serde_json::from_slice(&bytes).expect("body JSON")
}

async fn post_json(app: &axum::Router, uri: &str, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

async fn patch_json(app: &axum::Router, uri: &str, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

async fn create_menu(app: &axum::Router, kode: &str, nama: &str, harga: u32) {
    let response = post_json(
        app,
        "/api/menu",
        json!({ "kode": kode, "nama": nama, "harga": harga }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);
}

async fn create_order(app: &axum::Router, pelanggan: &str, items: Value) -> i64 {
    let response = post_json(
        app,
        "/api/orders",
        json!({ "pelanggan": pelanggan, "items": items }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);
    json_body(response).await["id"].as_i64().unwrap()
}

#[tokio::test]
async fn health_dan_menu_crud() {
    let state = test_state().await;
    let app = router(state);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    create_menu(&app, "kopi", "Kopi", 12_000).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/menu")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body[0]["kode"], "KOPI");
    assert_eq!(body[0]["harga"], 12_000);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/menu/KOPI")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await["nama"], "Kopi");
}

#[tokio::test]
async fn alur_pesanan_bayar_status_dan_laporan() {
    let state = test_state().await;
    let app = router(state);

    create_menu(&app, "KOPI", "Kopi", 12_000).await;
    create_menu(&app, "ROTI", "Roti", 8_000).await;

    let id = create_order(
        &app,
        "Bu Sri",
        json!([{ "kode": "KOPI", "jumlah": 2 }, { "kode": "ROTI", "jumlah": 1 }]),
    )
    .await;

    let response = post_json(
        &app,
        &format!("/api/orders/{id}/pay"),
        json!({ "metode": "qris" }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["status"], "dibayar");
    assert_eq!(body["total"], 32_000);
    assert_eq!(body["metode"], "qris");

    let response = patch_json(
        &app,
        &format!("/api/orders/{id}/status"),
        json!({ "status": "diproses" }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await["status"], "diproses");

    let response = patch_json(
        &app,
        &format!("/api/orders/{id}/status"),
        json!({ "status": "selesai" }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await["status"], "selesai");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/reports/daily")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["total"], 32_000);
    assert_eq!(body["jumlah_transaksi"], 1);
    assert_eq!(body["per_metode"]["qris"], 32_000);
}

#[tokio::test]
async fn transisi_ilegal_menghasilkan_json_409() {
    let state = test_state().await;
    let app = router(state);

    create_menu(&app, "KOPI", "Kopi", 12_000).await;
    let id = create_order(&app, "Pak Budi", json!([{ "kode": "KOPI", "jumlah": 1 }])).await;

    let response = patch_json(
        &app,
        &format!("/api/orders/{id}/status"),
        json!({ "status": "selesai" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
    let body = json_body(response).await;
    assert!(body["error"].as_str().unwrap().contains("tidak diizinkan"));
}

#[tokio::test]
async fn metode_pembayaran_tidak_dikenal_ditolak() {
    let state = test_state().await;
    let app = router(state);

    create_menu(&app, "KOPI", "Kopi", 12_000).await;
    let id = create_order(&app, "Bu Ani", json!([{ "kode": "KOPI", "jumlah": 1 }])).await;

    let response = post_json(
        &app,
        &format!("/api/orders/{id}/pay"),
        json!({ "metode": "kartu" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(
        json_body(response).await["error"]
            .as_str()
            .unwrap()
            .contains("tidak dikenal")
    );
}

#[tokio::test]
async fn pesanan_tanpa_item_ditolak() {
    let state = test_state().await;
    let app = router(state);

    let response = post_json(
        &app,
        "/api/orders",
        json!({ "pelanggan": "Bu Ani", "items": [] }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(
        json_body(response).await["error"]
            .as_str()
            .unwrap()
            .contains("minimal satu item")
    );
}
