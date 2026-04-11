use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostRevision {
    pub id: i64,
    pub user_id: Option<i64>,
    pub post_id: Option<i64>,
    pub modifications: Option<serde_json::Value>,
    pub number: i32,
    pub hidden: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostRevision {
    pub async fn find_by_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_revisions WHERE post_id = $1 ORDER BY number ASC")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_number(pool: &PgPool, post_id: i64, number: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_revisions WHERE post_id = $1 AND number = $2")
            .bind(post_id)
            .bind(number)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        post_id: i64,
        user_id: i64,
        number: i32,
        modifications: serde_json::Value,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO post_revisions (post_id, user_id, number, modifications)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(post_id)
        .bind(user_id)
        .bind(number)
        .bind(modifications)
        .fetch_one(pool)
        .await
    }

    pub async fn hide(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE post_revisions SET hidden = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
