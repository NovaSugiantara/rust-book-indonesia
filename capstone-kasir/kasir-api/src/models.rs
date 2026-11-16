use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct MenuResponse {
    pub kode: String,
    pub nama: String,
    pub harga: u32,
    pub aktif: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMenuRequest {
    pub kode: String,
    pub nama: String,
    pub harga: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuRequest {
    pub nama: String,
    pub harga: u32,
    pub aktif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ItemRequest {
    pub kode: String,
    pub jumlah: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub pelanggan: String,
    pub items: Vec<ItemRequest>,
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub kode: String,
    pub jumlah: u32,
    pub harga: u32,
    pub subtotal: u32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i64,
    pub pelanggan: String,
    pub status: String,
    pub total: u32,
    pub metode: Option<String>,
    pub created_at: String,
    pub paid_at: Option<String>,
    pub items: Vec<ItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct PayRequest {
    pub metode: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusRequest {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ReportResponse {
    pub tanggal: String,
    pub total: u32,
    pub jumlah_transaksi: u32,
    pub per_metode: HashMap<String, u32>,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub id: i64,
    pub order_id: i64,
    pub jenis: String,
    pub pesan: String,
    pub created_at: String,
}
