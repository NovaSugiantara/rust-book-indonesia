use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Notifikasi {
    pub order_id: i64,
    pub pesan: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub notif_tx: mpsc::Sender<Notifikasi>,
}

pub type SharedState = Arc<AppState>;
