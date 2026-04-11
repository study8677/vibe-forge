use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Handles post and topic flagging workflows.
pub struct FlagHandler;

/// The minimum number of flags required to auto-hide a post.
const AUTO_HIDE_THRESHOLD: i64 = 3;

impl FlagHandler {
    /// Flag a post for review.
    pub async fn flag_post(
        pool: &PgPool,
        post_id: i64,
        user_id: i64,
        flag_type: i16,
        message: Option<&str>,
    ) -> Result<()> {
        // Check that user hasn't already flagged this post
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM post_actions
             WHERE post_id = $1 AND user_id = $2 AND post_action_type_id = $3 AND deleted_at IS NULL"
        )
        .bind(post_id)
        .bind(user_id)
        .bind(flag_type)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(Error::AlreadyExists {
                resource: "flag",
                detail: "you already flagged this post".into(),
            });
        }

        // Create the flag action
        sqlx::query(
            "INSERT INTO post_actions (post_id, user_id, post_action_type_id, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())"
        )
        .bind(post_id)
        .bind(user_id)
        .bind(flag_type)
        .execute(pool)
        .await?;

        // Create or update a reviewable for this post
        let existing_reviewable: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM reviewables WHERE target_type = 'Post' AND target_id = $1 AND status = 0"
        )
        .bind(post_id)
        .fetch_optional(pool)
        .await?;

        if let Some((reviewable_id,)) = existing_reviewable {
            sqlx::query(
                "UPDATE reviewables SET score = score + 1.0, updated_at = NOW() WHERE id = $1"
            )
            .bind(reviewable_id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO reviewables (type, target_type, target_id, created_by_id, status, score, created_at, updated_at)
                 VALUES ('ReviewableFlaggedPost', 'Post', $1, $2, 0, 1.0, NOW(), NOW())"
            )
            .bind(post_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        }

        // Check if we should auto-hide the post
        Self::check_auto_hide(pool, post_id).await?;

        Ok(())
    }

    /// Check if a post has enough flags to be automatically hidden.
    async fn check_auto_hide(pool: &PgPool, post_id: i64) -> Result<()> {
        let flag_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM post_actions
             WHERE post_id = $1 AND post_action_type_id IN (3, 6, 7, 8) AND deleted_at IS NULL"
        )
        .bind(post_id)
        .fetch_one(pool)
        .await?;

        if flag_count.0 >= AUTO_HIDE_THRESHOLD {
            sqlx::query(
                "UPDATE posts SET hidden = TRUE, hidden_at = NOW(), hidden_reason_id = 2 WHERE id = $1 AND hidden = FALSE"
            )
            .bind(post_id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// Dismiss all flags on a post (used by moderators to clear flags without action).
    pub async fn dismiss_flags(pool: &PgPool, post_id: i64, dismissed_by_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE post_actions SET agreed_by_id = NULL, deferred_by_id = $2, deferred_at = NOW()
             WHERE post_id = $1 AND post_action_type_id IN (3, 6, 7, 8) AND deleted_at IS NULL"
        )
        .bind(post_id)
        .bind(dismissed_by_id)
        .execute(pool)
        .await?;

        // Resolve the reviewable
        sqlx::query(
            "UPDATE reviewables SET status = 3, reviewed_by_id = $2, reviewed_at = NOW()
             WHERE target_type = 'Post' AND target_id = $1 AND status = 0"
        )
        .bind(post_id)
        .bind(dismissed_by_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Agree with flags on a post and take action (hide or delete).
    pub async fn agree_with_flags(pool: &PgPool, post_id: i64, agreed_by_id: i64, delete_post: bool) -> Result<()> {
        sqlx::query(
            "UPDATE post_actions SET agreed_by_id = $2, agreed_at = NOW()
             WHERE post_id = $1 AND post_action_type_id IN (3, 6, 7, 8) AND deleted_at IS NULL"
        )
        .bind(post_id)
        .bind(agreed_by_id)
        .execute(pool)
        .await?;

        if delete_post {
            sqlx::query("UPDATE posts SET deleted_at = NOW(), deleted_by_id = $2 WHERE id = $1")
                .bind(post_id)
                .bind(agreed_by_id)
                .execute(pool)
                .await?;
        } else {
            sqlx::query("UPDATE posts SET hidden = TRUE, hidden_at = NOW(), hidden_reason_id = 4 WHERE id = $1")
                .bind(post_id)
                .execute(pool)
                .await?;
        }

        // Resolve the reviewable
        sqlx::query(
            "UPDATE reviewables SET status = 1, reviewed_by_id = $2, reviewed_at = NOW()
             WHERE target_type = 'Post' AND target_id = $1 AND status = 0"
        )
        .bind(post_id)
        .bind(agreed_by_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
