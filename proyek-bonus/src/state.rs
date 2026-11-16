use crate::error::AppError;
use crate::shortcode::ShortCode;
use crate::store::Store;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// State yang dibagikan ke semua handler.
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn Store<Error = AppError>>,
    pub click_tx: mpsc::Sender<String>,
    pub audit_tx: mpsc::Sender<String>,
    pub recent_clicks: Arc<Mutex<Vec<String>>>,
    pub base_url: String,
}

impl AppState {
    pub fn new(
        store: Arc<dyn Store<Error = AppError>>,
        base_url: String,
        click_tx: mpsc::Sender<String>,
        audit_tx: mpsc::Sender<String>,
        recent_clicks: Arc<Mutex<Vec<String>>>,
    ) -> Self {
        Self {
            store,
            click_tx,
            audit_tx,
            recent_clicks,
            base_url,
        }
    }

    pub fn short_url(&self, code: &ShortCode) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), code)
    }
}
