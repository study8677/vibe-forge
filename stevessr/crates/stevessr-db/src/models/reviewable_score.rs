use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReviewableScore {
    pub id: i64,
    pub reviewable_id: i64,
    pub user_id: i64,
    pub reviewable_score_type: i32,
    pub status: i32,
    pub score: f64,
    pub take_action_bonus: f64,
    pub reviewed_by_id: Option<i64>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub meta_topic_id: Option<i64>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReviewableScore {
    pub async fn find_by_reviewable(pool: &PgPool, reviewable_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM reviewable_scores WHERE reviewable_id = $1 ORDER BY created_at ASC")
            .bind(reviewable_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        reviewable_id: i64,
        user_id: i64,
        score_type: i32,
        score: f64,
        reason: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO reviewable_scores (reviewable_id, user_id, reviewable_score_type, score, reason)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(reviewable_id)
        .bind(user_id)
        .bind(score_type)
        .bind(score)
        .bind(reason)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(pool: &PgPool, id: i64, status: i32, reviewed_by_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE reviewable_scores SET status = $2, reviewed_by_id = $3, reviewed_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(status)
            .bind(reviewed_by_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
