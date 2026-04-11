use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKey {
    pub id: i64,
    pub key: String,
    pub truncated_key: String,
    pub user_id: Option<i64>,
    pub created_by_id: Option<i64>,
    pub description: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKey {
    pub async fn find_by_key(pool: &PgPool, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM api_keys WHERE key = $1 AND revoked_at IS NULL")
            .bind(key)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM api_keys ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        key: &str,
        truncated_key: &str,
        user_id: Option<i64>,
        created_by_id: Option<i64>,
        description: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO api_keys (key, truncated_key, user_id, created_by_id, description)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(key)
        .bind(truncated_key)
        .bind(user_id)
        .bind(created_by_id)
        .bind(description)
        .fetch_one(pool)
        .await
    }

    pub async fn revoke(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE api_keys SET revoked_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn touch(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE api_keys SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
