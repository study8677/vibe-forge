use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Post action types matching Discourse conventions.
pub const ACTION_LIKE: i16 = 2;
pub const ACTION_FLAG_SPAM: i16 = 8;
pub const ACTION_FLAG_INAPPROPRIATE: i16 = 6;
pub const ACTION_FLAG_OFF_TOPIC: i16 = 3;
pub const ACTION_FLAG_SOMETHING_ELSE: i16 = 7;
pub const ACTION_BOOKMARK: i16 = 1;

pub struct PostActionHandler;

impl PostActionHandler {
    pub async fn act(pool: &PgPool, post_id: i64, user_id: i64, action_type: i16) -> Result<()> {
        // Check for duplicate action
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM post_actions WHERE post_id = $1 AND user_id = $2 AND post_action_type_id = $3 AND deleted_at IS NULL"
        )
        .bind(post_id)
        .bind(user_id)
        .bind(action_type)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(Error::AlreadyExists {
                resource: "post_action",
                detail: "you already performed this action".into(),
            });
        }

        // Prevent self-like
        if action_type == ACTION_LIKE {
            let post_author: Option<(i64,)> = sqlx::query_as(
                "SELECT user_id FROM posts WHERE id = $1"
            )
            .bind(post_id)
            .fetch_optional(pool)
            .await?;

            if let Some((author_id,)) = post_author {
                if author_id == user_id {
                    return Err(Error::Forbidden("you cannot like your own post".into()));
                }
            }
        }

        // Create the action
        sqlx::query(
            "INSERT INTO post_actions (post_id, user_id, post_action_type_id, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())"
        )
        .bind(post_id)
        .bind(user_id)
        .bind(action_type)
        .execute(pool)
        .await?;

        // Update counters
        if action_type == ACTION_LIKE {
            sqlx::query("UPDATE posts SET like_count = like_count + 1 WHERE id = $1")
                .bind(post_id)
                .execute(pool)
                .await?;

            // Update user stats
            sqlx::query("UPDATE user_stats SET likes_given = likes_given + 1 WHERE user_id = $1")
                .bind(user_id)
                .execute(pool)
                .await?;

            let post_author: Option<(i64,)> = sqlx::query_as(
                "SELECT user_id FROM posts WHERE id = $1"
            )
            .bind(post_id)
            .fetch_optional(pool)
            .await?;

            if let Some((author_id,)) = post_author {
                sqlx::query("UPDATE user_stats SET likes_received = likes_received + 1 WHERE user_id = $1")
                    .bind(author_id)
                    .execute(pool)
                    .await?;

                // Create notification for like
                sqlx::query(
                    "INSERT INTO notifications (notification_type, user_id, data, topic_id, post_number, created_at, updated_at)
                     VALUES (5, $1, $2, (SELECT topic_id FROM posts WHERE id = $3), (SELECT post_number FROM posts WHERE id = $3), NOW(), NOW())"
                )
                .bind(author_id)
                .bind(serde_json::json!({"display_username": user_id.to_string()}).to_string())
                .bind(post_id)
                .execute(pool)
                .await?;
            }
        }

        // If this is a flag, create a reviewable
        if matches!(action_type, ACTION_FLAG_SPAM | ACTION_FLAG_INAPPROPRIATE | ACTION_FLAG_OFF_TOPIC | ACTION_FLAG_SOMETHING_ELSE) {
            sqlx::query(
                "INSERT INTO reviewables (type, target_type, target_id, created_by_id, status, score, created_at, updated_at)
                 VALUES ('ReviewableFlaggedPost', 'Post', $1, $2, 0, 1.0, NOW(), NOW())"
            )
            .bind(post_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn undo(pool: &PgPool, post_id: i64, user_id: i64, action_type: i16) -> Result<()> {
        let result = sqlx::query(
            "UPDATE post_actions SET deleted_at = NOW() WHERE post_id = $1 AND user_id = $2 AND post_action_type_id = $3 AND deleted_at IS NULL"
        )
        .bind(post_id)
        .bind(user_id)
        .bind(action_type)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound {
                resource: "post_action",
                id: format!("post={} user={} type={}", post_id, user_id, action_type),
            });
        }

        if action_type == ACTION_LIKE {
            sqlx::query("UPDATE posts SET like_count = GREATEST(like_count - 1, 0) WHERE id = $1")
                .bind(post_id)
                .execute(pool)
                .await?;

            sqlx::query("UPDATE user_stats SET likes_given = GREATEST(likes_given - 1, 0) WHERE user_id = $1")
                .bind(user_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}
