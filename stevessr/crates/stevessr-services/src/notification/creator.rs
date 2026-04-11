use sqlx::PgPool;
use stevessr_core::error::Result;

/// Notification types matching Discourse conventions.
pub const NOTIFICATION_MENTIONED: i32 = 1;
pub const NOTIFICATION_REPLIED: i32 = 2;
pub const NOTIFICATION_QUOTED: i32 = 3;
pub const NOTIFICATION_EDITED: i32 = 4;
pub const NOTIFICATION_LIKED: i32 = 5;
pub const NOTIFICATION_PRIVATE_MESSAGE: i32 = 6;
pub const NOTIFICATION_INVITED_TO_PM: i32 = 7;
pub const NOTIFICATION_WATCHING_FIRST_POST: i32 = 9;
pub const NOTIFICATION_TOPIC_REMINDER: i32 = 10;
pub const NOTIFICATION_LINKED: i32 = 11;
pub const NOTIFICATION_GRANTED_BADGE: i32 = 12;
pub const NOTIFICATION_GROUP_MENTIONED: i32 = 15;
pub const NOTIFICATION_BOOKMARK_REMINDER: i32 = 16;

pub struct NotificationCreator;

impl NotificationCreator {
    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        notification_type: i32,
        topic_id: Option<i64>,
        post_number: Option<i32>,
        data: &serde_json::Value,
    ) -> Result<i64> {
        // Do not notify if user has read the post already at this number
        if let (Some(tid), Some(pn)) = (topic_id, post_number) {
            let already_read: Option<(i32,)> = sqlx::query_as(
                "SELECT last_read_post_number FROM topic_users WHERE user_id = $1 AND topic_id = $2"
            )
            .bind(user_id)
            .bind(tid)
            .fetch_optional(pool)
            .await?;

            if let Some((last_read,)) = already_read {
                if last_read >= pn {
                    // User has already read past this post; skip notification
                    // Still create it but mark as read
                    let row: (i64,) = sqlx::query_as(
                        "INSERT INTO notifications (notification_type, user_id, data, topic_id, post_number, read, created_at, updated_at)
                         VALUES ($1, $2, $3, $4, $5, TRUE, NOW(), NOW()) RETURNING id"
                    )
                    .bind(notification_type)
                    .bind(user_id)
                    .bind(data.to_string())
                    .bind(tid)
                    .bind(pn)
                    .fetch_one(pool)
                    .await?;
                    return Ok(row.0);
                }
            }
        }

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO notifications (notification_type, user_id, data, topic_id, post_number, read, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, FALSE, NOW(), NOW()) RETURNING id"
        )
        .bind(notification_type)
        .bind(user_id)
        .bind(data.to_string())
        .bind(topic_id)
        .bind(post_number)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn mark_read(pool: &PgPool, user_id: i64, notification_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE notifications SET read = TRUE WHERE id = $1 AND user_id = $2"
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_all_read(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE notifications SET read = TRUE WHERE user_id = $1 AND read = FALSE"
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unread_count(pool: &PgPool, user_id: i64) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read = FALSE"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }
}
