use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicLink {
    pub id: i64,
    pub topic_id: i64,
    pub post_id: Option<i64>,
    pub user_id: i64,
    pub url: String,
    pub domain: String,
    pub internal: bool,
    pub link_topic_id: Option<i64>,
    pub link_post_id: Option<i64>,
    pub clicks: i32,
    pub reflection: bool,
    pub title: Option<String>,
    pub crawled_at: Option<DateTime<Utc>>,
    pub quote: bool,
    pub extension: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TopicLink {
    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_links WHERE topic_id = $1 ORDER BY clicks DESC")
            .bind(topic_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_links WHERE post_id = $1")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        topic_id: i64,
        post_id: Option<i64>,
        user_id: i64,
        url: &str,
        domain: &str,
        internal: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO topic_links (topic_id, post_id, user_id, url, domain, internal)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
        )
        .bind(topic_id)
        .bind(post_id)
        .bind(user_id)
        .bind(url)
        .bind(domain)
        .bind(internal)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_clicks(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topic_links SET clicks = clicks + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
