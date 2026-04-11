use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PollVote {
    pub id: i64,
    pub poll_id: i64,
    pub poll_option_id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PollVote {
    pub async fn find_by_poll_and_user(pool: &PgPool, poll_id: i64, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM poll_votes WHERE poll_id = $1 AND user_id = $2")
            .bind(poll_id)
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, poll_id: i64, poll_option_id: i64, user_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO poll_votes (poll_id, poll_option_id, user_id) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(poll_id)
        .bind(poll_option_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_for_user(pool: &PgPool, poll_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM poll_votes WHERE poll_id = $1 AND user_id = $2")
            .bind(poll_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn vote_count(pool: &PgPool, poll_option_id: i64) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM poll_votes WHERE poll_option_id = $1")
            .bind(poll_option_id)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}
