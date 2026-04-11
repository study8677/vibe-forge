use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKeyScope {
    pub id: i64,
    pub api_key_id: i64,
    pub resource: String,
    pub action: Option<String>,
    pub allowed_parameters: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKeyScope {
    pub async fn find_by_api_key(pool: &PgPool, api_key_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM api_key_scopes WHERE api_key_id = $1")
            .bind(api_key_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        api_key_id: i64,
        resource: &str,
        action: Option<&str>,
        allowed_parameters: Option<serde_json::Value>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO api_key_scopes (api_key_id, resource, action, allowed_parameters)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(api_key_id)
        .bind(resource)
        .bind(action)
        .bind(allowed_parameters)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_for_api_key(pool: &PgPool, api_key_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM api_key_scopes WHERE api_key_id = $1")
            .bind(api_key_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
