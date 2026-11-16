use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use serde_json::{Value, json};
use shortener::error::AppError;
use shortener::routes;
use shortener::shortcode::ShortCode;
use shortener::state::AppState;
use shortener::store::{MemoryStore, Store};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tower::ServiceExt;

fn test_state() -> (AppState, mpsc::Receiver<String>, mpsc::Receiver<String>) {
    let store: Arc<dyn Store<Error = AppError>> = Arc::new(MemoryStore::new());
    let (click_tx, click_rx) = mpsc::channel(8);
    let (audit_tx, audit_rx) = mpsc::channel(8);

    (
        AppState::new(
            store,
            "http://localhost:3000".to_string(),
            click_tx,
            audit_tx,
            Arc::new(Mutex::new(Vec::new())),
        ),
        click_rx,
        audit_rx,
    )
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn create_redirect_and_read_stats() {
    let (state, _click_rx, _audit_rx) = test_state();
    let app = routes::router(state.clone());

    let request = Request::builder()
        .method("POST")
        .uri("/api/links")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "target": "https://example.com/promo",
                "custom_code": "promo"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = body_json(response).await;
    assert_eq!(body["short_url"], "http://localhost:3000/promo");

    let request = Request::builder()
        .uri("/promo")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers()["location"].to_str().unwrap(),
        "https://example.com/promo"
    );

    let code = ShortCode::parse("promo").unwrap();
    state.store.increment_clicks(&code).await.unwrap();

    let request = Request::builder()
        .uri("/api/stats/promo")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_json(response).await;
    assert_eq!(body["clicks"], 1);
    assert_eq!(body["code"], "promo");
}

#[tokio::test]
async fn rejects_invalid_target() {
    let (state, _click_rx, _audit_rx) = test_state();
    let app = routes::router(state);

    let request = Request::builder()
        .method("POST")
        .uri("/api/links")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "target": "ftp://example.com" }).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        body_json(response).await["error"],
        "target harus diawali http:// atau https://"
    );
}
