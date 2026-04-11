use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DirectoryItem {
    pub id: i64,
    pub period_type: i32,
    pub user_id: i64,
    pub likes_received: i32,
    pub likes_given: i32,
    pub topics_entered: i32,
    pub topic_count: i32,
    pub post_count: i32,
    pub days_visited: i32,
    pub posts_read: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DirectoryItem {
    pub async fn find_by_period(pool: &PgPool, period_type: i32, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM directory_items WHERE period_type = $1 ORDER BY likes_received DESC LIMIT $2 OFFSET $3",
        )
        .bind(period_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_user_and_period(pool: &PgPool, user_id: i64, period_type: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM directory_items WHERE user_id = $1 AND period_type = $2")
            .bind(user_id)
            .bind(period_type)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(
        pool: &PgPool,
        period_type: i32,
        user_id: i64,
        likes_received: i32,
        likes_given: i32,
        topic_count: i32,
        post_count: i32,
        days_visited: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO directory_items (period_type, user_id, likes_received, likes_given, topic_count, post_count, days_visited)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               ON CONFLICT (period_type, user_id) DO UPDATE SET
                 likes_received = $3, likes_given = $4, topic_count = $5, post_count = $6, days_visited = $7, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(period_type)
        .bind(user_id)
        .bind(likes_received)
        .bind(likes_given)
        .bind(topic_count)
        .bind(post_count)
        .bind(days_visited)
        .fetch_one(pool)
        .await
    }
}
