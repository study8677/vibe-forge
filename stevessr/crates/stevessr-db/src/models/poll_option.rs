use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PollOption {
    pub id: i64,
    pub poll_id: i64,
    pub digest: String,
    pub html: String,
    pub anonymous_votes: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PollOption {
    pub async fn find_by_poll(pool: &PgPool, poll_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM poll_options WHERE poll_id = $1 ORDER BY id ASC")
            .bind(poll_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, poll_id: i64, digest: &str, html: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO poll_options (poll_id, digest, html) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(poll_id)
        .bind(digest)
        .bind(html)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_for_poll(pool: &PgPool, poll_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM poll_options WHERE poll_id = $1")
            .bind(poll_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
