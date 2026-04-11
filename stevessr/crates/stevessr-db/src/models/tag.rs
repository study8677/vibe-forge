use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub topic_count: i32,
    pub pm_topic_count: i32,
    pub target_tag_id: Option<i64>,
    pub staff_topic_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tags WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tags WHERE LOWER(name) = LOWER($1)")
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tags ORDER BY topic_count DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn search(pool: &PgPool, query: &str, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        let pattern = format!("%{}%", query.to_lowercase());
        sqlx::query_as::<_, Self>(
            "SELECT * FROM tags WHERE LOWER(name) LIKE $1 ORDER BY topic_count DESC LIMIT $2",
        )
        .bind(pattern)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(pool: &PgPool, name: &str, description: Option<&str>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO tags (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(description)
        .fetch_one(pool)
        .await
    }

    pub async fn update_topic_count(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE tags SET topic_count = (SELECT COUNT(*) FROM topic_tags WHERE tag_id = $1), updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
