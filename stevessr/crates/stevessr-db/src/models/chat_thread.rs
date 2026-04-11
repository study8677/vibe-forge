use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatThread {
    pub id: i64,
    pub channel_id: i64,
    pub original_message_id: i64,
    pub original_message_user_id: i64,
    pub status: i32,
    pub title: Option<String>,
    pub replies_count: i32,
    pub last_message_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatThread {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_threads WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_channel(pool: &PgPool, channel_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM chat_threads WHERE channel_id = $1 ORDER BY updated_at DESC LIMIT $2",
        )
        .bind(channel_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        channel_id: i64,
        original_message_id: i64,
        original_message_user_id: i64,
        title: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO chat_threads (channel_id, original_message_id, original_message_user_id, title, status)
               VALUES ($1, $2, $3, $4, 0) RETURNING *"#,
        )
        .bind(channel_id)
        .bind(original_message_id)
        .bind(original_message_user_id)
        .bind(title)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_replies(pool: &PgPool, id: i64, last_message_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE chat_threads SET replies_count = replies_count + 1, last_message_id = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(last_message_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
