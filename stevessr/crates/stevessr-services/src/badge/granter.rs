use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct BadgeGranter;

impl BadgeGranter {
    /// Grant a badge to a user. Idempotent -- does nothing if already granted
    /// (unless the badge allows multiple grants).
    pub async fn grant(pool: &PgPool, badge_id: i64, user_id: i64, granted_by_id: i64, reason: Option<&str>) -> Result<i64> {
        // Check badge exists
        let badge: Option<(i64, bool)> = sqlx::query_as(
            "SELECT id, multiple_grant FROM badges WHERE id = $1 AND enabled = TRUE"
        )
        .bind(badge_id)
        .fetch_optional(pool)
        .await?;

        let (_id, multiple_grant) = badge.ok_or(Error::NotFound {
            resource: "badge",
            id: badge_id.to_string(),
        })?;

        // Check for existing grant (unless multiple_grant is allowed)
        if !multiple_grant {
            let existing: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM user_badges WHERE badge_id = $1 AND user_id = $2"
            )
            .bind(badge_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

            if let Some((existing_id,)) = existing {
                return Ok(existing_id);
            }
        }

        // Grant the badge
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO user_badges (badge_id, user_id, granted_by_id, granted_at, created_at)
             VALUES ($1, $2, $3, NOW(), NOW()) RETURNING id"
        )
        .bind(badge_id)
        .bind(user_id)
        .bind(granted_by_id)
        .fetch_one(pool)
        .await?;

        // Update badge grant count
        sqlx::query("UPDATE badges SET grant_count = grant_count + 1 WHERE id = $1")
            .bind(badge_id)
            .execute(pool)
            .await?;

        // Create notification for badge grant
        sqlx::query(
            "INSERT INTO notifications (notification_type, user_id, data, created_at, updated_at)
             VALUES (12, $1, $2, NOW(), NOW())"
        )
        .bind(user_id)
        .bind(serde_json::json!({
            "badge_id": badge_id,
            "badge_name": "",
            "granted_by": granted_by_id.to_string(),
            "reason": reason.unwrap_or(""),
        }).to_string())
        .execute(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn revoke(pool: &PgPool, user_badge_id: i64) -> Result<()> {
        let badge_info: Option<(i64,)> = sqlx::query_as(
            "SELECT badge_id FROM user_badges WHERE id = $1"
        )
        .bind(user_badge_id)
        .fetch_optional(pool)
        .await?;

        if let Some((badge_id,)) = badge_info {
            sqlx::query("DELETE FROM user_badges WHERE id = $1")
                .bind(user_badge_id)
                .execute(pool)
                .await?;

            sqlx::query("UPDATE badges SET grant_count = GREATEST(grant_count - 1, 0) WHERE id = $1")
                .bind(badge_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}
