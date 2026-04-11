use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatDraft {
    pub id: i64,
    pub user_id: i64,
    pub chat_channel_id: i64,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatDraft {
    pub async fn find_by_channel_and_user(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_drafts WHERE chat_channel_id = $1 AND user_id = $2")
            .bind(channel_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, user_id: i64, chat_channel_id: i64, data: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO chat_drafts (user_id, chat_channel_id, data)
               VALUES ($1, $2, $3)
               ON CONFLICT (user_id, chat_channel_id) DO UPDATE SET data = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(user_id)
        .bind(chat_channel_id)
        .bind(data)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, user_id: i64, chat_channel_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM chat_drafts WHERE user_id = $1 AND chat_channel_id = $2")
            .bind(user_id)
            .bind(chat_channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
