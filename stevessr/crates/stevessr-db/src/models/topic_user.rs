use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicUser {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
    pub posted: bool,
    pub last_read_post_number: Option<i32>,
    pub last_visited_at: Option<DateTime<Utc>>,
    pub first_visited_at: Option<DateTime<Utc>>,
    pub notification_level: i16,
    pub notifications_changed_at: Option<DateTime<Utc>>,
    pub notifications_reason_id: Option<i32>,
    pub total_msecs_viewed: i64,
    pub cleared_pinned_at: Option<DateTime<Utc>>,
    pub bookmarked: bool,
    pub liked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TopicUser {
    pub async fn find_by_topic_and_user(pool: &PgPool, topic_id: i64, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_users WHERE topic_id = $1 AND user_id = $2")
            .bind(topic_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM topic_users WHERE user_id = $1 ORDER BY last_visited_at DESC NULLS LAST LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn upsert(
        pool: &PgPool,
        topic_id: i64,
        user_id: i64,
        notification_level: i16,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO topic_users (topic_id, user_id, notification_level, first_visited_at, last_visited_at)
               VALUES ($1, $2, $3, NOW(), NOW())
               ON CONFLICT (topic_id, user_id) DO UPDATE SET last_visited_at = NOW(), notification_level = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(topic_id)
        .bind(user_id)
        .bind(notification_level)
        .fetch_one(pool)
        .await
    }

    pub async fn update_last_read(pool: &PgPool, topic_id: i64, user_id: i64, post_number: i32) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE topic_users SET last_read_post_number = GREATEST(last_read_post_number, $3), updated_at = NOW() WHERE topic_id = $1 AND user_id = $2",
        )
        .bind(topic_id)
        .bind(user_id)
        .bind(post_number)
        .execute(pool)
        .await?;
        Ok(())
    }
}
