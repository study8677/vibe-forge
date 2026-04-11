use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicTimer {
    pub id: i64,
    pub topic_id: i64,
    pub execute_at: DateTime<Utc>,
    pub status_type: i32,
    pub user_id: i64,
    pub based_on_last_post: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_id: Option<i64>,
    pub category_id: Option<i64>,
    pub public_type: Option<bool>,
    pub duration_minutes: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TopicTimer {
    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_timers WHERE topic_id = $1 AND deleted_at IS NULL")
            .bind(topic_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_pending(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM topic_timers WHERE execute_at <= NOW() AND deleted_at IS NULL ORDER BY execute_at ASC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        topic_id: i64,
        execute_at: DateTime<Utc>,
        status_type: i32,
        user_id: i64,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO topic_timers (topic_id, execute_at, status_type, user_id)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(topic_id)
        .bind(execute_at)
        .bind(status_type)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn soft_delete(pool: &PgPool, id: i64, deleted_by_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topic_timers SET deleted_at = NOW(), deleted_by_id = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(deleted_by_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
