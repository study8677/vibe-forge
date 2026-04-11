use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Draft keys for different editing contexts.
pub const DRAFT_NEW_TOPIC: &str = "new_topic";
pub const DRAFT_NEW_PRIVATE_MESSAGE: &str = "new_private_message";

pub struct DraftManager;

impl DraftManager {
    /// Save or update a draft.
    pub async fn save(
        pool: &PgPool,
        user_id: i64,
        draft_key: &str,
        sequence: i32,
        data: &str,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO drafts (user_id, draft_key, sequence, data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())
             ON CONFLICT (user_id, draft_key) DO UPDATE SET data = $4, sequence = $3, updated_at = NOW()"
        )
        .bind(user_id)
        .bind(draft_key)
        .bind(sequence)
        .bind(data)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Get a specific draft.
    pub async fn get(pool: &PgPool, user_id: i64, draft_key: &str) -> Result<Option<(String, i32)>> {
        let row: Option<(String, i32)> = sqlx::query_as(
            "SELECT data, sequence FROM drafts WHERE user_id = $1 AND draft_key = $2"
        )
        .bind(user_id)
        .bind(draft_key)
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    /// Delete a specific draft.
    pub async fn delete(pool: &PgPool, user_id: i64, draft_key: &str) -> Result<()> {
        sqlx::query("DELETE FROM drafts WHERE user_id = $1 AND draft_key = $2")
            .bind(user_id)
            .bind(draft_key)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// List all drafts for a user.
    pub async fn list(pool: &PgPool, user_id: i64) -> Result<Vec<(String, String, i32, chrono::DateTime<chrono::Utc>)>> {
        let rows: Vec<(String, String, i32, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT draft_key, data, sequence, updated_at FROM drafts WHERE user_id = $1 ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    /// Count user's drafts.
    pub async fn count(pool: &PgPool, user_id: i64) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM drafts WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    /// Clean up old drafts (e.g., older than 180 days).
    pub async fn cleanup_old(pool: &PgPool, max_age_days: i32) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM drafts WHERE updated_at < NOW() - ($1 || ' days')::INTERVAL"
        )
        .bind(max_age_days.to_string())
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }
}
