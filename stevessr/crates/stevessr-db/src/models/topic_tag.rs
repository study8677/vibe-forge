use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicTag {
    pub id: i64,
    pub topic_id: i64,
    pub tag_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TopicTag {
    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_tags WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_tag(pool: &PgPool, tag_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_tags WHERE tag_id = $1")
            .bind(tag_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, topic_id: i64, tag_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO topic_tags (topic_id, tag_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(topic_id)
        .bind(tag_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, topic_id: i64, tag_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM topic_tags WHERE topic_id = $1 AND tag_id = $2")
            .bind(topic_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_all_for_topic(pool: &PgPool, topic_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM topic_tags WHERE topic_id = $1")
            .bind(topic_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
