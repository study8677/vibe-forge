use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct TopicTracker;

impl TopicTracker {
    pub async fn set_notification_level(pool: &PgPool, user_id: i64, topic_id: i64, level: i16) -> Result<()> {
        sqlx::query(
            "INSERT INTO topic_users (user_id, topic_id, notification_level, notifications_changed_at)
             VALUES ($1, $2, $3, NOW())
             ON CONFLICT (user_id, topic_id) DO UPDATE SET notification_level = $3, notifications_changed_at = NOW()"
        )
        .bind(user_id).bind(topic_id).bind(level)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn update_read_state(pool: &PgPool, user_id: i64, topic_id: i64, post_number: i32) -> Result<()> {
        sqlx::query(
            "INSERT INTO topic_users (user_id, topic_id, last_read_post_number, last_visited_at, first_visited_at)
             VALUES ($1, $2, $3, NOW(), NOW())
             ON CONFLICT (user_id, topic_id) DO UPDATE SET last_read_post_number = GREATEST(topic_users.last_read_post_number, $3), last_visited_at = NOW()"
        )
        .bind(user_id).bind(topic_id).bind(post_number)
        .execute(pool).await?;
        Ok(())
    }
}
