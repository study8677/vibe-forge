use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicAllowedGroup {
    pub id: i64,
    pub group_id: i64,
    pub topic_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TopicAllowedGroup {
    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_allowed_groups WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, topic_id: i64, group_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO topic_allowed_groups (topic_id, group_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(topic_id)
        .bind(group_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, topic_id: i64, group_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM topic_allowed_groups WHERE topic_id = $1 AND group_id = $2")
            .bind(topic_id)
            .bind(group_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
