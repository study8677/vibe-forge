use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMembership {
    pub id: i64,
    pub user_id: i64,
    pub chat_channel_id: i64,
    pub last_read_message_id: Option<i64>,
    pub following: bool,
    pub muted: bool,
    pub desktop_notification_level: i16,
    pub mobile_notification_level: i16,
    pub notification_level: i16,
    pub join_mode: Option<i32>,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatMembership {
    pub async fn find_by_channel_and_user(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_memberships WHERE chat_channel_id = $1 AND user_id = $2")
            .bind(channel_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_channel(pool: &PgPool, channel_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_memberships WHERE chat_channel_id = $1 AND following = TRUE")
            .bind(channel_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_memberships WHERE user_id = $1 AND following = TRUE")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64, chat_channel_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO chat_memberships (user_id, chat_channel_id, following) VALUES ($1, $2, TRUE) RETURNING *",
        )
        .bind(user_id)
        .bind(chat_channel_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_last_read(pool: &PgPool, id: i64, message_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE chat_memberships SET last_read_message_id = $2, last_viewed_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(message_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn unfollow(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE chat_memberships SET following = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
