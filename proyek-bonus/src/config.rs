use std::env;
use std::fmt;

/// Konfigurasi layanan dari environment variable.
#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub base_url: String,
    pub use_memory: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let bind_addr = bind_addr.trim().to_string();
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://shortener.db".to_string());
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| format!("http://{bind_addr}"));
        let store = env::var("STORE").unwrap_or_else(|_| "sqlite".to_string());

        let use_memory = match store.as_str() {
            "memory" => true,
            "sqlite" => false,
            other => {
                return Err(ConfigError(format!(
                    "STORE harus memory atau sqlite, bukan {other}"
                )));
            }
        };

        Ok(Self {
            bind_addr,
            database_url,
            base_url,
            use_memory,
        })
    }

    pub fn wants_help() -> bool {
        env::args().any(|argument| argument == "--help" || argument == "-h")
    }

    pub fn help() -> &'static str {
        "shortener\n\n\
         Env:\n\
         BIND_ADDR      alamat HTTP, default 127.0.0.1:3000\n\
         DATABASE_URL   URL SQLite, default sqlite://shortener.db\n\
         BASE_URL       URL publik untuk short link, default http://BIND_ADDR\n\
         STORE          sqlite atau memory, default sqlite\n\n\
         Jalankan: cargo run\n\
         Hentikan: Ctrl+C"
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug)]
pub struct ConfigError(pub String);
