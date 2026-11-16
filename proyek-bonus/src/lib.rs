//! Proyek bonus Rust dari Nol: URL shortener dengan statistik klik.
//!
//! Proyek ini sengaja tidak memakai `unsafe`. Semua akses data berbagi
//! memakai `Arc`, `RwLock`, dan `Mutex` dari standard library.

#![forbid(unsafe_code)]

pub mod config;
pub mod error;
pub mod model;
pub mod routes;
pub mod shortcode;
pub mod state;
pub mod stats;
pub mod store;
pub mod worker;

pub use error::AppError;
pub use model::{Link, LinkStatus};
pub use shortcode::ShortCode;
pub use state::AppState;
pub use store::{MemoryStore, SqliteStore, Store};
