use crate::error::AppError;
use crate::shortcode::ShortCode;
use crate::store::Store;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tokio::sync::mpsc;

/// Worker async yang menulis klik ke store.
pub fn spawn_click_worker(
    store: Arc<dyn Store<Error = AppError>>,
    mut receiver: mpsc::Receiver<String>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(value) = receiver.recv().await {
            let Ok(code) = ShortCode::parse(&value) else {
                continue;
            };

            if let Err(error) = store.increment_clicks(&code).await {
                eprintln!("gagal menyimpan klik {code}: {error}");
            }
        }
    })
}

/// Worker blocking dengan std::thread dan buffer bersama.
pub struct AuditGuard {
    sender: Option<mpsc::Sender<String>>,
    handle: Option<JoinHandle<()>>,
    buffer: Arc<Mutex<Vec<String>>>,
}

impl AuditGuard {
    pub fn spawn(
        sender: mpsc::Sender<String>,
        mut receiver: mpsc::Receiver<String>,
        buffer: Arc<Mutex<Vec<String>>>,
    ) -> Self {
        let worker_buffer = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            while let Some(code) = receiver.blocking_recv() {
                if let Ok(mut entries) = worker_buffer.lock() {
                    entries.push(code);
                    if entries.len() > 20 {
                        entries.remove(0);
                    }
                }
            }
        });

        Self {
            sender: Some(sender),
            handle: Some(handle),
            buffer,
        }
    }

    pub fn buffer(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.buffer)
    }
}

impl Drop for AuditGuard {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        if let Ok(entries) = self.buffer.lock() {
            eprintln!("audit: {} klik terakhir tercatat", entries.len());
        }
    }
}
