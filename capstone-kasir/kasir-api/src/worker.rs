use crate::state::Notifikasi;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Worker latar belakang mencatat notifikasi pembayaran ke tabel events.
pub fn spawn_worker(pool: SqlitePool, mut rx: mpsc::Receiver<Notifikasi>) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(notifikasi) = rx.recv().await {
            tracing::info!(
                order_id = notifikasi.order_id,
                pesan = %notifikasi.pesan,
                "notifikasi pembayaran"
            );

            if let Err(error) = sqlx::query(
                "INSERT INTO events (order_id, jenis, pesan) VALUES (?, 'pembayaran', ?)",
            )
            .bind(notifikasi.order_id)
            .bind(&notifikasi.pesan)
            .execute(&pool)
            .await
            {
                tracing::error!(error = %error, "gagal menyimpan event pembayaran");
            }
        }
    })
}
