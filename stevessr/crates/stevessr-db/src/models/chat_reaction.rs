use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatReaction {
    pub id: i64,
    pub chat_message_id: i64,
    pub user_id: i64,
    pub emoji: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatReaction {
    pub async fn find_by_message(pool: &PgPool, chat_message_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_reactions WHERE chat_message_id = $1")
            .bind(chat_message_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, chat_message_id: i64, user_id: i64, emoji: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO chat_reactions (chat_message_id, user_id, emoji) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(chat_message_id)
        .bind(user_id)
        .bind(emoji)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, chat_message_id: i64, user_id: i64, emoji: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM chat_reactions WHERE chat_message_id = $1 AND user_id = $2 AND emoji = $3")
            .bind(chat_message_id)
            .bind(user_id)
            .bind(emoji)
            .execute(pool)
            .await?;
        Ok(())
    }
}
