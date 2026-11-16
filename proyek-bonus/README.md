# Shortener: proyek bonus Rust dari Nol

URL shortener kecil dengan statistik klik. Proyek ini berdiri sendiri, di luar 21 bab utama dan di luar Capstone. Tujuannya adalah contoh kerja yang memakai hampir semua materi Rust Book dalam satu layanan yang bisa dijalankan.

## Menjalankan

```console
$ cargo run
```

Default-nya memakai SQLite dan membuat file `shortener.db` di folder ini. Kalau ingin tanpa database:

```console
$ STORE=memory cargo run
```

Environment variable:

| Variable | Default | Keterangan |
| --- | --- | --- |
| `BIND_ADDR` | `127.0.0.1:3000` | Alamat HTTP |
| `DATABASE_URL` | `sqlite://shortener.db` | Lokasi SQLite |
| `BASE_URL` | `http://BIND_ADDR` | Base URL pada response short link |
| `STORE` | `sqlite` | `sqlite` atau `memory` |

Bantuan argumen:

```console
$ cargo run -- --help
```

## Endpoint

```console
$ curl -i -X POST http://127.0.0.1:3000/api/links \
  -H 'Content-Type: application/json' \
  -d '{"target":"https://example.com/promo","custom_code":"promo"}'

$ curl -i http://127.0.0.1:3000/promo
$ curl http://127.0.0.1:3000/api/stats/promo
$ curl http://127.0.0.1:3000/api/links
$ curl http://127.0.0.1:3000/api/audit
```

`POST /api/links` mengembalikan 201. Redirect mengembalikan 307 dan header `Location`. Statistik dihitung dari klik yang dikirim ke worker.

## Test dan release

```console
$ cargo test
$ cargo clippy --all-targets -- -D warnings
$ cargo build --release
$ ./target/release/shortener
```

Test mencakup unit test modul, integration test endpoint di `tests/api.rs`, dan doc test pada `ShortCode`.

## Struktur

```text
proyek-bonus/
├── Cargo.toml
├── COVERAGE.md
├── src/
│   ├── lib.rs          module publik dan re-export
│   ├── main.rs         setup runtime, store, worker, graceful shutdown
│   ├── config.rs       environment dan argumen
│   ├── error.rs        AppError dan IntoResponse
│   ├── model.rs        Link dan LinkStatus
│   ├── shortcode.rs    newtype ShortCode
│   ├── stats.rs        iterator, closure, lifetime
│   ├── store.rs        trait Store, memory, SQLite
│   ├── state.rs        Arc state
│   ├── routes.rs       axum router dan handler
│   └── worker.rs       tokio task, std thread, channel, Drop
├── tests/
│   └── api.rs          integration test endpoint
└── COVERAGE.md         peta konsep ke baris kode
```

## Konsep yang dijelaskan, tidak dipakai

Beberapa konsep Rust Book sengaja tidak dipakai di kode. Alasannya:

- Instalasi `rustup`: proyek ini menganggap Rust sudah terpasang; langkah instalasi ada di Bab 1 situs.
- `std::io::stdin`: layanan HTTP menerima input lewat JSON request, bukan prompt terminal.
- `Rc` dan `RefCell`: state dibagi ke banyak task async, jadi dipakai `Arc` dengan `RwLock` atau `Mutex`. `RefCell` tidak `Sync` dan bukan pilihan yang tepat untuk state lintas thread.
- `Weak`: tidak ada kepemilikan melingkar yang perlu diputus. Semua state dimiliki runtime dan dijatuhkan saat shutdown.
- `spawn_blocking`: semua I/O database memakai sqlx async dan handler tidak melakukan pekerjaan CPU berat. Kalau nanti ada hashing password atau pembacaan file besar, itu tempat `spawn_blocking` dipakai.
- Stream: response statistik kecil dan langsung dikirim sebagai JSON. Belum ada kebutuhan SSE atau body streaming.
- State pattern objek: status link cukup direpresentasikan dengan enum `LinkStatus`. Belum ada perilaku berbeda per status yang membutuhkan objek state.
- Workspace Cargo: satu package dengan `lib.rs` dan `main.rs` sudah memisahkan library dan binary. Workspace baru berguna kalau ada beberapa binary atau library yang dipakai bersama.
- `unsafe`: crate ini memakai `#![forbid(unsafe_code)]`. Layanan HTTP biasa tidak membutuhkan unsafe; semua state bersama ditangani `Arc`, `RwLock`, dan `Mutex`.
- Thread pool manual dari Bab 21: di proyek ini, runtime multi-thread Tokio dan axum menggantikan thread pool manual. Bab 21 tetap penting untuk memahami apa yang framework lakukan di belakang layar.

Perbedaan channel: `tokio::sync::mpsc` dipakai untuk worker async yang menulis klik ke store. Untuk audit, std thread memakai `blocking_recv` pada receiver Tokio karena thread itu memang blocking. Jangan memanggil `blocking_recv` di dalam task async.
