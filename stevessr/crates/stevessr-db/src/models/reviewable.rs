use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reviewable {
    pub id: i64,
    pub reviewable_type: String,
    pub status: i32,
    pub created_by_id: i64,
    pub reviewable_by_moderator: bool,
    pub reviewable_by_group_id: Option<i64>,
    pub category_id: Option<i64>,
    pub topic_id: Option<i64>,
    pub score: f64,
    pub potential_spam: bool,
    pub target_id: Option<i64>,
    pub target_type: Option<String>,
    pub target_created_by_id: Option<i64>,
    pub payload: Option<serde_json::Value>,
    pub version: i32,
    pub latest_score: Option<DateTime<Utc>>,
    pub force_review: bool,
    pub reject_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Reviewable {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM reviewables WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_pending(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM reviewables WHERE status = 0 ORDER BY score DESC, created_at ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_target(pool: &PgPool, target_type: &str, target_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM reviewables WHERE target_type = $1 AND target_id = $2 AND status = 0",
        )
        .bind(target_type)
        .bind(target_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        reviewable_type: &str,
        created_by_id: i64,
        target_id: Option<i64>,
        target_type: Option<&str>,
        target_created_by_id: Option<i64>,
        category_id: Option<i64>,
        topic_id: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO reviewables (reviewable_type, created_by_id, target_id, target_type, target_created_by_id, category_id, topic_id)
               VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
        )
        .bind(reviewable_type)
        .bind(created_by_id)
        .bind(target_id)
        .bind(target_type)
        .bind(target_created_by_id)
        .bind(category_id)
        .bind(topic_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(pool: &PgPool, id: i64, status: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE reviewables SET status = $2, version = version + 1, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(status)
            .execute(pool)
            .await?;
        Ok(())
    }
}
