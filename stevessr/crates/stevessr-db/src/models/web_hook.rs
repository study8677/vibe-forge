use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebHook {
    pub id: i64,
    pub payload_url: String,
    pub content_type: i32,
    pub last_delivery_status: i32,
    pub status: i32,
    pub secret: Option<String>,
    pub wildcard_web_hook: bool,
    pub verify_certificate: bool,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WebHook {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM web_hooks WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_active(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM web_hooks WHERE active = TRUE")
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        payload_url: &str,
        content_type: i32,
        secret: Option<&str>,
        wildcard: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO web_hooks (payload_url, content_type, secret, wildcard_web_hook, active)
               VALUES ($1, $2, $3, $4, TRUE) RETURNING *"#,
        )
        .bind(payload_url)
        .bind(content_type)
        .bind(secret)
        .bind(wildcard)
        .fetch_one(pool)
        .await
    }

    pub async fn deactivate(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE web_hooks SET active = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM web_hooks WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
