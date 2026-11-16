use shortener::config::Config;
use shortener::error::AppError;
use shortener::state::AppState;
use shortener::store::{MemoryStore, SqliteStore, Store};
use shortener::worker::{AuditGuard, spawn_click_worker};
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if Config::wants_help() {
        eprintln!("{}", Config::help());
        return Ok(());
    }

    let config = Config::from_env()?;

    let boxed_store: Box<dyn Store<Error = AppError>> = if config.use_memory {
        Box::new(MemoryStore::new())
    } else {
        Box::new(SqliteStore::connect(&config.database_url).await?)
    };
    let store: Arc<dyn Store<Error = AppError>> = Arc::from(boxed_store);

    let (click_tx, click_rx) = mpsc::channel(128);
    let (audit_tx, audit_rx) = mpsc::channel(64);
    let recent_clicks = Arc::new(Mutex::new(Vec::new()));

    let click_worker = spawn_click_worker(Arc::clone(&store), click_rx);
    let audit_guard = AuditGuard::spawn(audit_tx.clone(), audit_rx, Arc::clone(&recent_clicks));

    let state = AppState::new(
        store,
        config.base_url.clone(),
        click_tx,
        audit_tx,
        recent_clicks,
    );
    assert_send_sync::<AppState>();

    let app = shortener::routes::router(state.clone());
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;

    println!("Shortener berjalan di http://{}", config.bind_addr);
    println!(
        "Store: {}",
        if config.use_memory {
            "memory"
        } else {
            "sqlite"
        }
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    drop(state);
    click_worker.abort();
    drop(audit_guard);

    Ok(())
}

fn assert_send_sync<T: Send + Sync>() {}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("gagal memasang signal shutdown: {error}");
    }
}
