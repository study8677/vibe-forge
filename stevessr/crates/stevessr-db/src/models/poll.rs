use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Poll {
    pub id: i64,
    pub post_id: i64,
    pub name: String,
    pub close_at: Option<DateTime<Utc>>,
    pub poll_type: i32,
    pub status: i32,
    pub results: i32,
    pub visibility: i32,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub step: Option<i32>,
    pub anonymous_voters: Option<i32>,
    pub chart_type: i32,
    pub groups: Option<String>,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Poll {
    pub async fn find_by_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM polls WHERE post_id = $1")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_post_and_name(pool: &PgPool, post_id: i64, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM polls WHERE post_id = $1 AND name = $2")
            .bind(post_id)
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        post_id: i64,
        name: &str,
        poll_type: i32,
        status: i32,
        close_at: Option<DateTime<Utc>>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO polls (post_id, name, poll_type, status, close_at)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(post_id)
        .bind(name)
        .bind(poll_type)
        .bind(status)
        .bind(close_at)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(pool: &PgPool, id: i64, status: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE polls SET status = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(status)
            .execute(pool)
            .await?;
        Ok(())
    }
}
