# Kasir Warung API (Capstone)

Materi ini berada di luar 21 bab The Rust Programming Language. API dibangun dengan axum 0.8.9, tokio 1.54.1, dan sqlx 0.9.0 di atas SQLite.

## Menjalankan

```console
$ export BIND_ADDR=127.0.0.1:3000
$ export DATABASE_URL=sqlite://kasir.db
$ export RUST_LOG=info
$ cargo run -p kasir-api
```

Migrasi sqlx dijalankan otomatis saat startup. Database dibuat jika belum ada.

## Endpoint

| Method | Path | Fungsi |
| --- | --- | --- |
| GET | `/health` | Status layanan |
| GET | `/api/menu` | Daftar menu |
| POST | `/api/menu` | Tambah menu |
| GET | `/api/menu/{kode}` | Detail menu |
| PUT | `/api/menu/{kode}` | Ubah menu |
| DELETE | `/api/menu/{kode}` | Hapus menu |
| POST | `/api/orders` | Buat pesanan |
| GET | `/api/orders/{id}` | Detail pesanan |
| POST | `/api/orders/{id}/pay` | Bayar lewat trait object |
| PATCH | `/api/orders/{id}/status` | Ubah status dengan aturan transisi |
| GET | `/api/reports/daily` | Laporan harian |
| GET | `/api/events` | Event latar belakang |

## Perintah kualitas

```console
$ cargo test
$ cargo clippy --all-targets -- -D warnings
$ cargo build --release
```

## Struktur

- `kasir-core/`: domain `ItemPesanan`, `Transaksi`, `StatusPesanan`, `KasirError`, `PaymentMethod`, dan rekap iterator.
- `kasir-api/`: axum router, handler, sqlx store, migrasi, worker notifikasi, test endpoint.
- `kasir-api/migrations/`: skema SQLite.

## Catatan

Dockerfile tidak disertakan karena tidak wajib. Untuk deployment sederhana, binary release ada di `target/release/kasir-api` setelah `cargo build --release`. Jangan commit database SQLite atau folder target.
