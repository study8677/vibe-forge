use sqlx::PgPool;
use stevessr_core::error::Result;
use super::granter::BadgeGranter;

/// Well-known badge IDs for built-in badges.
pub const BADGE_FIRST_LIKE: i64 = 1;
pub const BADGE_FIRST_POST: i64 = 2;
pub const BADGE_FIRST_REPLY: i64 = 3;
pub const BADGE_READ_GUIDELINES: i64 = 4;
pub const BADGE_WELCOME: i64 = 5;
pub const BADGE_AUTOBIOGRAPHER: i64 = 6;
pub const BADGE_NICE_TOPIC: i64 = 7;
pub const BADGE_GOOD_TOPIC: i64 = 8;
pub const BADGE_GREAT_TOPIC: i64 = 9;
pub const BADGE_NICE_POST: i64 = 10;
pub const BADGE_GOOD_POST: i64 = 11;
pub const BADGE_GREAT_POST: i64 = 12;

/// System user ID used for automatic badge grants.
const SYSTEM_USER_ID: i64 = -1;

/// Handles automatic badge triggers based on user activity.
pub struct BadgeTriggerHandler;

impl BadgeTriggerHandler {
    /// Called after a user creates their first post.
    pub async fn on_first_post(pool: &PgPool, user_id: i64) -> Result<()> {
        let post_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM posts WHERE user_id = $1 AND deleted_at IS NULL"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        if post_count.0 == 1 {
            BadgeGranter::grant(pool, BADGE_FIRST_POST, user_id, SYSTEM_USER_ID, None).await?;
        }

        Ok(())
    }

    /// Called after a user receives their first like.
    pub async fn on_first_like_received(pool: &PgPool, user_id: i64) -> Result<()> {
        let like_count: (i64,) = sqlx::query_as(
            "SELECT COALESCE(likes_received, 0) FROM user_stats WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        if like_count.0 == 1 {
            BadgeGranter::grant(pool, BADGE_WELCOME, user_id, SYSTEM_USER_ID, None).await?;
        }

        Ok(())
    }

    /// Called after a user gives their first like.
    pub async fn on_first_like_given(pool: &PgPool, user_id: i64) -> Result<()> {
        let like_count: (i64,) = sqlx::query_as(
            "SELECT COALESCE(likes_given, 0) FROM user_stats WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        if like_count.0 == 1 {
            BadgeGranter::grant(pool, BADGE_FIRST_LIKE, user_id, SYSTEM_USER_ID, None).await?;
        }

        Ok(())
    }

    /// Called after a user updates their profile bio.
    pub async fn on_profile_updated(pool: &PgPool, user_id: i64) -> Result<()> {
        let bio: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT bio_raw FROM user_profiles WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some((Some(bio_text),)) = bio {
            if !bio_text.is_empty() {
                BadgeGranter::grant(pool, BADGE_AUTOBIOGRAPHER, user_id, SYSTEM_USER_ID, None).await?;
            }
        }

        Ok(())
    }

    /// Check post/topic like milestones for Nice/Good/Great badges.
    pub async fn check_like_milestones(pool: &PgPool, post_id: i64) -> Result<()> {
        let post_info: Option<(i64, i32, i64)> = sqlx::query_as(
            "SELECT p.user_id, p.like_count, p.topic_id FROM posts p WHERE p.id = $1"
        )
        .bind(post_id)
        .fetch_optional(pool)
        .await?;

        if let Some((user_id, like_count, _topic_id)) = post_info {
            if like_count >= 10 {
                BadgeGranter::grant(pool, BADGE_NICE_POST, user_id, SYSTEM_USER_ID, None).await?;
            }
            if like_count >= 25 {
                BadgeGranter::grant(pool, BADGE_GOOD_POST, user_id, SYSTEM_USER_ID, None).await?;
            }
            if like_count >= 50 {
                BadgeGranter::grant(pool, BADGE_GREAT_POST, user_id, SYSTEM_USER_ID, None).await?;
            }
        }

        Ok(())
    }
}
