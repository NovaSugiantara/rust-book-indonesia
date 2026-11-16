use crate::error::AppError;
use crate::model::{Link, LinkStatus};
use crate::shortcode::ShortCode;
use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::collections::HashMap;
use std::str::FromStr;
use tokio::sync::RwLock;

/// Abstraksi penyimpanan link.
///
/// `type Error` adalah associated type. Implementasi in-memory dan SQLite
/// memakai `AppError` yang sama supaya handler tidak peduli backend-nya.
#[async_trait]
pub trait Store: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn insert_link(&self, link: Link) -> Result<(), Self::Error>;
    async fn get_link(&self, code: &ShortCode) -> Result<Option<Link>, Self::Error>;
    async fn increment_clicks(&self, code: &ShortCode) -> Result<u64, Self::Error>;
    async fn list_links(&self) -> Result<Vec<Link>, Self::Error>;
}

/// Implementasi untuk test dan mode tanpa database.
#[derive(Default)]
pub struct MemoryStore {
    links: RwLock<HashMap<String, Link>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Store for MemoryStore {
    type Error = AppError;

    async fn insert_link(&self, link: Link) -> Result<(), AppError> {
        let mut links = self.links.write().await;

        if links.contains_key(link.code.as_str()) {
            return Err(AppError::Conflict("kode sudah dipakai".to_string()));
        }

        links.insert(link.code.to_string(), link);
        Ok(())
    }

    async fn get_link(&self, code: &ShortCode) -> Result<Option<Link>, AppError> {
        Ok(self.links.read().await.get(code.as_str()).cloned())
    }

    async fn increment_clicks(&self, code: &ShortCode) -> Result<u64, AppError> {
        let mut links = self.links.write().await;
        let Some(link) = links.get_mut(code.as_str()) else {
            return Err(AppError::NotFound);
        };

        link.clicks += 1;
        Ok(link.clicks)
    }

    async fn list_links(&self) -> Result<Vec<Link>, AppError> {
        let mut links: Vec<Link> = self.links.read().await.values().cloned().collect();
        links.sort_by_key(|link| std::cmp::Reverse(link.clicks));
        Ok(links)
    }
}

/// Implementasi SQLite memakai sqlx.
pub struct SqliteStore {
    pool: SqlitePool,
}

impl SqliteStore {
    pub async fn connect(url: &str) -> Result<Self, AppError> {
        let options = SqliteConnectOptions::from_str(url)?.create_if_missing(true);
        let max_connections = if url.contains(":memory:") { 1 } else { 5 };
        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect_with(options)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS links (
                code TEXT PRIMARY KEY,
                target TEXT NOT NULL,
                clicks INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'active'
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[async_trait]
impl Store for SqliteStore {
    type Error = AppError;

    async fn insert_link(&self, link: Link) -> Result<(), AppError> {
        let result =
            sqlx::query("INSERT INTO links (code, target, clicks, status) VALUES (?, ?, ?, ?)")
                .bind(link.code.as_str())
                .bind(&link.target)
                .bind(link.clicks as i64)
                .bind(link.status.as_str())
                .execute(&self.pool)
                .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
                Err(AppError::Conflict("kode sudah dipakai".to_string()))
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn get_link(&self, code: &ShortCode) -> Result<Option<Link>, AppError> {
        let row = sqlx::query_as::<_, (String, i64, String)>(
            "SELECT target, clicks, status FROM links WHERE code = ?",
        )
        .bind(code.as_str())
        .fetch_optional(&self.pool)
        .await?;

        let Some((target, clicks, status)) = row else {
            return Ok(None);
        };

        Ok(Some(Link {
            code: code.clone(),
            target,
            clicks: clicks as u64,
            status: LinkStatus::from_db(&status)?,
        }))
    }

    async fn increment_clicks(&self, code: &ShortCode) -> Result<u64, AppError> {
        let clicks = sqlx::query_scalar::<_, i64>(
            "UPDATE links SET clicks = clicks + 1 WHERE code = ? RETURNING clicks",
        )
        .bind(code.as_str())
        .fetch_optional(&self.pool)
        .await?;

        clicks.map(|value| value as u64).ok_or(AppError::NotFound)
    }

    async fn list_links(&self) -> Result<Vec<Link>, AppError> {
        let rows = sqlx::query_as::<_, (String, String, i64, String)>(
            "SELECT code, target, clicks, status FROM links ORDER BY clicks DESC, code ASC",
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|(code, target, clicks, status)| {
                Ok(Link {
                    code: ShortCode::parse(&code)?,
                    target,
                    clicks: clicks as u64,
                    status: LinkStatus::from_db(&status)?,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn memory_store_roundtrip() {
        let store = MemoryStore::new();
        let code = ShortCode::parse("promo").unwrap();
        store
            .insert_link(Link::new(code.clone(), "https://example.com".to_string()))
            .await
            .unwrap();

        assert_eq!(store.increment_clicks(&code).await.unwrap(), 1);
        assert_eq!(
            store.get_link(&code).await.unwrap().unwrap().target,
            "https://example.com"
        );
    }

    #[tokio::test]
    async fn sqlite_store_roundtrip() {
        let store = SqliteStore::connect("sqlite::memory:").await.unwrap();
        let code = ShortCode::parse("sqlite").unwrap();
        store
            .insert_link(Link::new(code.clone(), "https://example.org".to_string()))
            .await
            .unwrap();

        assert_eq!(store.increment_clicks(&code).await.unwrap(), 1);
        assert_eq!(store.list_links().await.unwrap().len(), 1);
    }
}
