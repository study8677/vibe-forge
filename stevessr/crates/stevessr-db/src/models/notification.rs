use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: i64,
    pub notification_type: i32,
    pub user_id: i64,
    pub data: String,
    pub read: bool,
    pub topic_id: Option<i64>,
    pub post_number: Option<i32>,
    pub post_action_id: Option<i64>,
    pub high_priority: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Notification {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_unread(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM notifications WHERE user_id = $1 AND read = FALSE ORDER BY high_priority DESC, created_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn unread_count(pool: &PgPool, user_id: i64) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read = FALSE")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }

    pub async fn create(
        pool: &PgPool,
        notification_type: i32,
        user_id: i64,
        data: &str,
        topic_id: Option<i64>,
        post_number: Option<i32>,
        high_priority: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO notifications (notification_type, user_id, data, topic_id, post_number, high_priority)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
        )
        .bind(notification_type)
        .bind(user_id)
        .bind(data)
        .bind(topic_id)
        .bind(post_number)
        .bind(high_priority)
        .fetch_one(pool)
        .await
    }

    pub async fn mark_read(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE notifications SET read = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn mark_all_read(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE notifications SET read = TRUE, updated_at = NOW() WHERE user_id = $1 AND read = FALSE")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
