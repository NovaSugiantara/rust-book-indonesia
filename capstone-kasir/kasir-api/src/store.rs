use crate::error::AppError;
use crate::models::{
    CreateMenuRequest, CreateOrderRequest, EventResponse, ItemResponse, MenuResponse,
    OrderResponse, ReportResponse, UpdateMenuRequest,
};
use kasir_core::{ItemPesanan, StatusPesanan, Transaksi, metode_pembayaran, rekap_per_metode};
use sqlx::SqlitePool;
use std::collections::HashMap;

pub async fn list_menu(pool: &SqlitePool) -> Result<Vec<MenuResponse>, AppError> {
    let rows = sqlx::query_as::<_, (String, String, i64, i64)>(
        "SELECT kode, nama, harga, aktif FROM menu ORDER BY kode",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(kode, nama, harga, aktif)| MenuResponse {
            kode,
            nama,
            harga: harga as u32,
            aktif: aktif != 0,
        })
        .collect())
}

pub async fn get_menu(pool: &SqlitePool, kode: &str) -> Result<MenuResponse, AppError> {
    let kode = normalize_kode(kode);
    let row = sqlx::query_as::<_, (String, String, i64, i64)>(
        "SELECT kode, nama, harga, aktif FROM menu WHERE kode = ?",
    )
    .bind(&kode)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("menu {kode} tidak ditemukan")))?;

    Ok(MenuResponse {
        kode: row.0,
        nama: row.1,
        harga: row.2 as u32,
        aktif: row.3 != 0,
    })
}

pub async fn create_menu(
    pool: &SqlitePool,
    request: CreateMenuRequest,
) -> Result<MenuResponse, AppError> {
    let kode = normalize_kode(&request.kode);
    let nama = request.nama.trim();

    if kode.is_empty() {
        return Err(AppError::BadRequest(
            "kode menu tidak boleh kosong".to_string(),
        ));
    }

    if nama.is_empty() {
        return Err(AppError::BadRequest(
            "nama menu tidak boleh kosong".to_string(),
        ));
    }

    if request.harga == 0 {
        return Err(AppError::BadRequest(
            "harga menu harus lebih dari nol".to_string(),
        ));
    }

    let result = sqlx::query("INSERT INTO menu (kode, nama, harga, aktif) VALUES (?, ?, ?, 1)")
        .bind(&kode)
        .bind(nama)
        .bind(i64::from(request.harga))
        .execute(pool)
        .await;

    match result {
        Ok(_) => get_menu(pool, &kode).await,
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            Err(AppError::Conflict(format!("menu {kode} sudah terdaftar")))
        }
        Err(error) => Err(error.into()),
    }
}

pub async fn update_menu(
    pool: &SqlitePool,
    kode: &str,
    request: UpdateMenuRequest,
) -> Result<MenuResponse, AppError> {
    let kode = normalize_kode(kode);
    let nama = request.nama.trim();

    if nama.is_empty() {
        return Err(AppError::BadRequest(
            "nama menu tidak boleh kosong".to_string(),
        ));
    }

    if request.harga == 0 {
        return Err(AppError::BadRequest(
            "harga menu harus lebih dari nol".to_string(),
        ));
    }

    let aktif = request.aktif.unwrap_or(true);
    let result = sqlx::query("UPDATE menu SET nama = ?, harga = ?, aktif = ? WHERE kode = ?")
        .bind(nama)
        .bind(i64::from(request.harga))
        .bind(if aktif { 1 } else { 0 })
        .bind(&kode)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("menu {kode} tidak ditemukan")));
    }

    get_menu(pool, &kode).await
}

pub async fn delete_menu(pool: &SqlitePool, kode: &str) -> Result<(), AppError> {
    let kode = normalize_kode(kode);
    let result = sqlx::query("DELETE FROM menu WHERE kode = ?")
        .bind(&kode)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("menu {kode} tidak ditemukan")));
    }

    Ok(())
}

pub async fn create_order(
    pool: &SqlitePool,
    request: CreateOrderRequest,
) -> Result<OrderResponse, AppError> {
    if request.items.is_empty() {
        return Err(AppError::BadRequest(
            "pesanan harus memiliki minimal satu item".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    let mut items = Vec::with_capacity(request.items.len());

    for item in &request.items {
        if item.jumlah == 0 {
            return Err(AppError::BadRequest(
                "jumlah item harus lebih dari nol".to_string(),
            ));
        }

        let kode = normalize_kode(&item.kode);
        let row = sqlx::query_as::<_, (String, i64, i64)>(
            "SELECT nama, harga, aktif FROM menu WHERE kode = ?",
        )
        .bind(&kode)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("menu {kode} tidak ditemukan")))?;

        if row.2 == 0 {
            return Err(AppError::BadRequest(format!(
                "menu {kode} sedang tidak aktif"
            )));
        }

        items.push(ItemPesanan::new(kode, item.jumlah, row.1 as u32)?);
    }

    let transaksi = Transaksi::baru(request.pelanggan, items)?;
    let total = transaksi.total()?;

    let result = sqlx::query("INSERT INTO orders (pelanggan, status, total) VALUES (?, ?, ?)")
        .bind(&transaksi.pelanggan)
        .bind(transaksi.status.as_str())
        .bind(i64::from(total))
        .execute(&mut *tx)
        .await?;

    let order_id = result.last_insert_rowid();

    for item in &transaksi.items {
        sqlx::query("INSERT INTO order_items (order_id, kode, jumlah, harga) VALUES (?, ?, ?, ?)")
            .bind(order_id)
            .bind(&item.kode)
            .bind(i64::from(item.jumlah))
            .bind(i64::from(item.harga))
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    get_order(pool, order_id).await
}

pub async fn get_order(pool: &SqlitePool, id: i64) -> Result<OrderResponse, AppError> {
    let row = sqlx::query_as::<
        _,
        (
            i64,
            String,
            String,
            i64,
            Option<String>,
            String,
            Option<String>,
        ),
    >(
        "SELECT id, pelanggan, status, total, metode, created_at, paid_at \
         FROM orders WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("pesanan {id} tidak ditemukan")))?;

    let items = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT kode, jumlah, harga FROM order_items WHERE order_id = ? ORDER BY id",
    )
    .bind(id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(kode, jumlah, harga)| ItemResponse {
        kode,
        jumlah: jumlah as u32,
        harga: harga as u32,
        subtotal: (jumlah * harga) as u32,
    })
    .collect();

    Ok(OrderResponse {
        id: row.0,
        pelanggan: row.1,
        status: StatusPesanan::parse(&row.2)?.as_str().to_string(),
        total: row.3 as u32,
        metode: row.4,
        created_at: row.5,
        paid_at: row.6,
        items,
    })
}

pub async fn pay_order(
    pool: &SqlitePool,
    id: i64,
    metode: &str,
) -> Result<OrderResponse, AppError> {
    let payment = metode_pembayaran(metode).ok_or_else(|| {
        AppError::BadRequest(format!("metode pembayaran tidak dikenal: {metode}"))
    })?;

    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, (String, i64)>("SELECT status, total FROM orders WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("pesanan {id} tidak ditemukan")))?;

    let status = StatusPesanan::parse(&row.0)?;
    if !status.bisa_dibayar() {
        return Err(AppError::Conflict(format!(
            "pesanan {id} berstatus {} dan tidak bisa dibayar",
            status.as_str()
        )));
    }

    let receipt = payment.bayar(row.1 as u32)?;
    let updated = sqlx::query(
        "UPDATE orders SET status = ?, metode = ?, paid_at = datetime('now') \
         WHERE id = ? AND status = ?",
    )
    .bind(StatusPesanan::Dibayar.as_str())
    .bind(&receipt.metode)
    .bind(id)
    .bind(StatusPesanan::Baru.as_str())
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() == 0 {
        return Err(AppError::Conflict(
            "status pesanan berubah sebelum pembayaran disimpan".to_string(),
        ));
    }

    sqlx::query("INSERT INTO payments (order_id, metode, total) VALUES (?, ?, ?)")
        .bind(id)
        .bind(&receipt.metode)
        .bind(i64::from(receipt.total))
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    get_order(pool, id).await
}

pub async fn update_status(
    pool: &SqlitePool,
    id: i64,
    target: StatusPesanan,
) -> Result<OrderResponse, AppError> {
    if target == StatusPesanan::Dibayar {
        return Err(AppError::Conflict(
            "status dibayar hanya lewat endpoint pembayaran".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    let current_text = sqlx::query_scalar::<_, String>("SELECT status FROM orders WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("pesanan {id} tidak ditemukan")))?;

    let current = StatusPesanan::parse(&current_text)?;
    if !current.can_transition_to(target) {
        return Err(AppError::Conflict(format!(
            "transisi {} ke {} tidak diizinkan",
            current.as_str(),
            target.as_str()
        )));
    }

    let result = sqlx::query("UPDATE orders SET status = ? WHERE id = ? AND status = ?")
        .bind(target.as_str())
        .bind(id)
        .bind(current.as_str())
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::Conflict(
            "status pesanan berubah sebelum update disimpan".to_string(),
        ));
    }

    tx.commit().await?;
    get_order(pool, id).await
}

pub async fn daily_report(
    pool: &SqlitePool,
    tanggal: Option<&str>,
) -> Result<ReportResponse, AppError> {
    let rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT metode, total FROM payments \
         WHERE date(paid_at) = COALESCE(date(?), date('now'))",
    )
    .bind(tanggal)
    .fetch_all(pool)
    .await?;

    let jumlah_transaksi = rows.len() as u32;
    let total: u32 = rows.iter().map(|(_, total)| *total as u32).sum();
    let per_metode = rekap_per_metode(
        rows.into_iter()
            .map(|(metode, total)| (metode, total as u32)),
    );

    let tanggal = if let Some(tanggal) = tanggal {
        tanggal.to_string()
    } else {
        sqlx::query_scalar::<_, String>("SELECT date('now')")
            .fetch_one(pool)
            .await?
    };

    Ok(ReportResponse {
        tanggal,
        total,
        jumlah_transaksi,
        per_metode,
    })
}

pub async fn list_events(pool: &SqlitePool) -> Result<Vec<EventResponse>, AppError> {
    let rows = sqlx::query_as::<_, (i64, i64, String, String, String)>(
        "SELECT id, order_id, jenis, pesan, created_at FROM events \
         ORDER BY id DESC LIMIT 20",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(id, order_id, jenis, pesan, created_at)| EventResponse {
            id,
            order_id,
            jenis,
            pesan,
            created_at,
        })
        .collect())
}

fn normalize_kode(kode: &str) -> String {
    kode.trim().to_uppercase()
}

/// Rekap per metode untuk dipakai test dan handler.
pub fn rekap(rows: Vec<(String, u32)>) -> HashMap<String, u32> {
    rekap_per_metode(rows)
}
