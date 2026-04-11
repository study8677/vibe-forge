use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_channel_id: i64,
    pub user_id: i64,
    pub message: String,
    pub cooked: String,
    pub cooked_version: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_id: Option<i64>,
    pub in_reply_to_id: Option<i64>,
    pub last_editor_id: Option<i64>,
    pub excerpt: Option<String>,
    pub thread_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatMessage {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_messages WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_channel(pool: &PgPool, channel_id: i64, limit: i64, before_id: Option<i64>) -> Result<Vec<Self>, sqlx::Error> {
        if let Some(before) = before_id {
            sqlx::query_as::<_, Self>(
                "SELECT * FROM chat_messages WHERE chat_channel_id = $1 AND id < $3 AND deleted_at IS NULL ORDER BY id DESC LIMIT $2",
            )
            .bind(channel_id)
            .bind(limit)
            .bind(before)
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, Self>(
                "SELECT * FROM chat_messages WHERE chat_channel_id = $1 AND deleted_at IS NULL ORDER BY id DESC LIMIT $2",
            )
            .bind(channel_id)
            .bind(limit)
            .fetch_all(pool)
            .await
        }
    }

    pub async fn create(
        pool: &PgPool,
        chat_channel_id: i64,
        user_id: i64,
        message: &str,
        cooked: &str,
        in_reply_to_id: Option<i64>,
        thread_id: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO chat_messages (chat_channel_id, user_id, message, cooked, in_reply_to_id, thread_id)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
        )
        .bind(chat_channel_id)
        .bind(user_id)
        .bind(message)
        .bind(cooked)
        .bind(in_reply_to_id)
        .bind(thread_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_message(pool: &PgPool, id: i64, message: &str, cooked: &str, editor_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE chat_messages SET message = $2, cooked = $3, last_editor_id = $4, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(message)
            .bind(cooked)
            .bind(editor_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn soft_delete(pool: &PgPool, id: i64, deleted_by_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE chat_messages SET deleted_at = NOW(), deleted_by_id = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(deleted_by_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
