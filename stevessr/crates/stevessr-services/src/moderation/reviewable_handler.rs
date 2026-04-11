use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Reviewable status constants.
pub const STATUS_PENDING: i32 = 0;
pub const STATUS_APPROVED: i32 = 1;
pub const STATUS_REJECTED: i32 = 2;
pub const STATUS_IGNORED: i32 = 3;
pub const STATUS_DELETED: i32 = 4;

pub struct ReviewableHandler;

impl ReviewableHandler {
    /// Approve a reviewable item.
    pub async fn approve(pool: &PgPool, reviewable_id: i64, reviewed_by_id: i64) -> Result<()> {
        let reviewable: Option<(i64, String, String, i64, i32)> = sqlx::query_as(
            "SELECT id, type, target_type, target_id, status FROM reviewables WHERE id = $1"
        )
        .bind(reviewable_id)
        .fetch_optional(pool)
        .await?;

        let (_, reviewable_type, target_type, target_id, status) = reviewable.ok_or(Error::NotFound {
            resource: "reviewable",
            id: reviewable_id.to_string(),
        })?;

        if status != STATUS_PENDING {
            return Err(Error::Forbidden("reviewable is not in pending state".into()));
        }

        // Update status
        sqlx::query(
            "UPDATE reviewables SET status = $2, reviewed_by_id = $3, reviewed_at = NOW(), updated_at = NOW() WHERE id = $1"
        )
        .bind(reviewable_id)
        .bind(STATUS_APPROVED)
        .bind(reviewed_by_id)
        .execute(pool)
        .await?;

        // Handle specific reviewable types
        match reviewable_type.as_str() {
            "ReviewableUser" => {
                // Approve the user
                sqlx::query("UPDATE users SET approved = TRUE, approved_by_id = $2, approved_at = NOW() WHERE id = $1")
                    .bind(target_id)
                    .bind(reviewed_by_id)
                    .execute(pool)
                    .await?;
            }
            "ReviewableQueuedPost" => {
                // The post was already created but hidden; make it visible
                sqlx::query("UPDATE posts SET hidden = FALSE, hidden_at = NULL WHERE id = $1")
                    .bind(target_id)
                    .execute(pool)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Reject a reviewable item.
    pub async fn reject(pool: &PgPool, reviewable_id: i64, reviewed_by_id: i64) -> Result<()> {
        let reviewable: Option<(i64, String, String, i64, i32)> = sqlx::query_as(
            "SELECT id, type, target_type, target_id, status FROM reviewables WHERE id = $1"
        )
        .bind(reviewable_id)
        .fetch_optional(pool)
        .await?;

        let (_, reviewable_type, target_type, target_id, status) = reviewable.ok_or(Error::NotFound {
            resource: "reviewable",
            id: reviewable_id.to_string(),
        })?;

        if status != STATUS_PENDING {
            return Err(Error::Forbidden("reviewable is not in pending state".into()));
        }

        sqlx::query(
            "UPDATE reviewables SET status = $2, reviewed_by_id = $3, reviewed_at = NOW(), updated_at = NOW() WHERE id = $1"
        )
        .bind(reviewable_id)
        .bind(STATUS_REJECTED)
        .bind(reviewed_by_id)
        .execute(pool)
        .await?;

        // Handle type-specific rejection
        match reviewable_type.as_str() {
            "ReviewableUser" => {
                // Reject the user (deactivate)
                sqlx::query("UPDATE users SET active = FALSE WHERE id = $1")
                    .bind(target_id)
                    .execute(pool)
                    .await?;
            }
            "ReviewableQueuedPost" => {
                // Delete the queued post
                sqlx::query("UPDATE posts SET deleted_at = NOW(), deleted_by_id = $2 WHERE id = $1")
                    .bind(target_id)
                    .bind(reviewed_by_id)
                    .execute(pool)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }

    /// List pending reviewables for the review queue.
    pub async fn list_pending(pool: &PgPool, page: i64, per_page: i64) -> Result<Vec<(i64, String, String, i64, f64)>> {
        let offset = (page - 1) * per_page;
        let rows: Vec<(i64, String, String, i64, f64)> = sqlx::query_as(
            "SELECT id, type, target_type, target_id, score
             FROM reviewables
             WHERE status = 0
             ORDER BY score DESC, created_at ASC
             LIMIT $1 OFFSET $2"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    pub async fn pending_count(pool: &PgPool) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM reviewables WHERE status = 0"
        )
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }
}
