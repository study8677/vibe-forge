use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebHookEvent {
    pub id: i64,
    pub web_hook_id: i64,
    pub headers: Option<String>,
    pub payload: Option<String>,
    pub status: i32,
    pub response_headers: Option<String>,
    pub response_body: Option<String>,
    pub duration: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WebHookEvent {
    pub async fn find_by_webhook(pool: &PgPool, web_hook_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM web_hook_events WHERE web_hook_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(web_hook_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM web_hook_events WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        web_hook_id: i64,
        headers: Option<&str>,
        payload: Option<&str>,
        status: i32,
        response_headers: Option<&str>,
        response_body: Option<&str>,
        duration: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO web_hook_events (web_hook_id, headers, payload, status, response_headers, response_body, duration)
               VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
        )
        .bind(web_hook_id)
        .bind(headers)
        .bind(payload)
        .bind(status)
        .bind(response_headers)
        .bind(response_body)
        .bind(duration)
        .fetch_one(pool)
        .await
    }
}
