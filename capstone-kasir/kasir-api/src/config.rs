use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://kasir.db".to_string());

        if bind_addr.trim().is_empty() {
            return Err("BIND_ADDR tidak boleh kosong".to_string());
        }

        if database_url.trim().is_empty() {
            return Err("DATABASE_URL tidak boleh kosong".to_string());
        }

        Ok(Self {
            bind_addr,
            database_url,
        })
    }
}
