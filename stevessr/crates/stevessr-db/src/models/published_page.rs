use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PublishedPage {
    pub id: i64,
    pub topic_id: i64,
    pub slug: String,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PublishedPage {
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM published_pages WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM published_pages WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all_public(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM published_pages WHERE public = TRUE ORDER BY slug ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, topic_id: i64, slug: &str, public: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO published_pages (topic_id, slug, public) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(topic_id)
        .bind(slug)
        .bind(public)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM published_pages WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
