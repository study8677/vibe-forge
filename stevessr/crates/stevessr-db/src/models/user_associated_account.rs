use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAssociatedAccount {
    pub id: i64,
    pub provider_name: String,
    pub provider_uid: String,
    pub user_id: Option<i64>,
    pub last_used: Option<DateTime<Utc>>,
    pub info: Option<serde_json::Value>,
    pub credentials: Option<serde_json::Value>,
    pub extra: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAssociatedAccount {
    pub async fn find_by_provider(pool: &PgPool, provider_name: &str, provider_uid: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_associated_accounts WHERE provider_name = $1 AND provider_uid = $2",
        )
        .bind(provider_name)
        .bind(provider_uid)
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_associated_accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        provider_name: &str,
        provider_uid: &str,
        user_id: Option<i64>,
        info: Option<serde_json::Value>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_associated_accounts (provider_name, provider_uid, user_id, info)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(provider_name)
        .bind(provider_uid)
        .bind(user_id)
        .bind(info)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_associated_accounts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
