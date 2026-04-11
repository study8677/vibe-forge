use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostCustomField {
    pub id: i64,
    pub post_id: i64,
    pub name: String,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostCustomField {
    pub async fn find_by_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_custom_fields WHERE post_id = $1")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_name(pool: &PgPool, post_id: i64, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_custom_fields WHERE post_id = $1 AND name = $2")
            .bind(post_id)
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, post_id: i64, name: &str, value: Option<&str>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO post_custom_fields (post_id, name, value)
               VALUES ($1, $2, $3)
               ON CONFLICT (post_id, name) DO UPDATE SET value = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(post_id)
        .bind(name)
        .bind(value)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM post_custom_fields WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
