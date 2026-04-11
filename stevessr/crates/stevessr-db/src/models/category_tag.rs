use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CategoryTag {
    pub id: i64,
    pub category_id: i64,
    pub tag_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryTag {
    pub async fn find_by_category(pool: &PgPool, category_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM category_tags WHERE category_id = $1")
            .bind(category_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, category_id: i64, tag_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO category_tags (category_id, tag_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(category_id)
        .bind(tag_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, category_id: i64, tag_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM category_tags WHERE category_id = $1 AND tag_id = $2")
            .bind(category_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
