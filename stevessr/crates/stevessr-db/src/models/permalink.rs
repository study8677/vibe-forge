use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Permalink {
    pub id: i64,
    pub url: String,
    pub topic_id: Option<i64>,
    pub post_id: Option<i64>,
    pub category_id: Option<i64>,
    pub tag_id: Option<i64>,
    pub external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Permalink {
    pub async fn find_by_url(pool: &PgPool, url: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM permalinks WHERE url = $1")
            .bind(url)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM permalinks ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        url: &str,
        topic_id: Option<i64>,
        post_id: Option<i64>,
        category_id: Option<i64>,
        external_url: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO permalinks (url, topic_id, post_id, category_id, external_url)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(url)
        .bind(topic_id)
        .bind(post_id)
        .bind(category_id)
        .bind(external_url)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM permalinks WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
