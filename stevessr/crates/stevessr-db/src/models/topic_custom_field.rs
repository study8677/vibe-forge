use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicCustomField {
    pub id: i64,
    pub topic_id: i64,
    pub name: String,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TopicCustomField {
    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_custom_fields WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_name(pool: &PgPool, topic_id: i64, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_custom_fields WHERE topic_id = $1 AND name = $2")
            .bind(topic_id)
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, topic_id: i64, name: &str, value: Option<&str>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO topic_custom_fields (topic_id, name, value)
               VALUES ($1, $2, $3)
               ON CONFLICT (topic_id, name) DO UPDATE SET value = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(topic_id)
        .bind(name)
        .bind(value)
        .fetch_one(pool)
        .await
    }
}
