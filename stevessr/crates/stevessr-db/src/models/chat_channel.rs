use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatChannel {
    pub id: i64,
    pub chatable_id: Option<i64>,
    pub chatable_type: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: i32,
    pub user_count: i32,
    pub last_message_id: Option<i64>,
    pub auto_join_users: bool,
    pub allow_channel_wide_mentions: bool,
    pub messages_count: i32,
    pub threading_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatChannel {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM chat_channels WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_public(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM chat_channels WHERE chatable_type = 'Category' AND status = 0 ORDER BY name ASC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_direct_messages_for_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"SELECT cc.* FROM chat_channels cc
               INNER JOIN chat_memberships cm ON cm.chat_channel_id = cc.id
               WHERE cm.user_id = $1 AND cc.chatable_type = 'DirectMessage'
               ORDER BY cc.updated_at DESC"#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        chatable_id: Option<i64>,
        chatable_type: Option<&str>,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO chat_channels (chatable_id, chatable_type, name, slug, description, status)
               VALUES ($1, $2, $3, $4, $5, 0) RETURNING *"#,
        )
        .bind(chatable_id)
        .bind(chatable_type)
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(pool: &PgPool, id: i64, status: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE chat_channels SET status = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(status)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM chat_channels WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
