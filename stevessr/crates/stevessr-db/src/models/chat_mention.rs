use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMention {
    pub id: i64,
    pub chat_message_id: i64,
    pub target_id: i64,
    pub target_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatMention {
    pub async fn find_by_message(pool: &PgPool, chat_message_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_mentions WHERE chat_message_id = $1")
            .bind(chat_message_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_for_user(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM chat_mentions WHERE target_id = $1 AND target_type = 'User' ORDER BY created_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(pool: &PgPool, chat_message_id: i64, target_id: i64, target_type: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO chat_mentions (chat_message_id, target_id, target_type) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(chat_message_id)
        .bind(target_id)
        .bind(target_type)
        .fetch_one(pool)
        .await
    }
}
