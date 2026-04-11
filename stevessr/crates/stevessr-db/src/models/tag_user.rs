use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagUser {
    pub id: i64,
    pub tag_id: i64,
    pub user_id: i64,
    pub notification_level: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TagUser {
    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tag_users WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, tag_id: i64, user_id: i64, notification_level: i16) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO tag_users (tag_id, user_id, notification_level)
               VALUES ($1, $2, $3)
               ON CONFLICT (tag_id, user_id) DO UPDATE SET notification_level = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(tag_id)
        .bind(user_id)
        .bind(notification_level)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, tag_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tag_users WHERE tag_id = $1 AND user_id = $2")
            .bind(tag_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
