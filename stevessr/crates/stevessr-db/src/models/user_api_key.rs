use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserApiKey {
    pub id: i64,
    pub user_id: i64,
    pub client_id: String,
    pub key: String,
    pub application_name: String,
    pub push_url: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub scopes: Option<Vec<String>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserApiKey {
    pub async fn find_by_key(pool: &PgPool, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_api_keys WHERE key = $1 AND revoked_at IS NULL")
            .bind(key)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_api_keys WHERE user_id = $1 ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        client_id: &str,
        key: &str,
        application_name: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_api_keys (user_id, client_id, key, application_name)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(user_id)
        .bind(client_id)
        .bind(key)
        .bind(application_name)
        .fetch_one(pool)
        .await
    }

    pub async fn revoke(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_api_keys SET revoked_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
