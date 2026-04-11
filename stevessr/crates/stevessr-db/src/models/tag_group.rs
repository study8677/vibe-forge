use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagGroup {
    pub id: i64,
    pub name: String,
    pub one_per_topic: bool,
    pub parent_tag_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TagGroup {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tag_groups WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tag_groups ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, name: &str, one_per_topic: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO tag_groups (name, one_per_topic) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(one_per_topic)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tag_groups WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
