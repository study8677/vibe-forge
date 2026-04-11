use chrono::{DateTime, Utc};
use sqlx::PgPool;
use stevessr_core::error::Result;
use stevessr_db::models::topic_timer::TopicTimer;

pub struct TopicTimerManager;

impl TopicTimerManager {
    pub async fn set_timer(pool: &PgPool, topic_id: i64, user_id: i64, execute_at: DateTime<Utc>, status_type: i16) -> Result<TopicTimer> {
        // Cancel existing timer of same type
        sqlx::query("UPDATE topic_timers SET deleted_at = NOW() WHERE topic_id = $1 AND status_type = $2 AND deleted_at IS NULL")
            .bind(topic_id).bind(status_type).execute(pool).await?;

        TopicTimer::create(pool, topic_id, execute_at, status_type as i32, user_id).await.map_err(Into::into)
    }

    pub async fn cancel_timer(pool: &PgPool, topic_id: i64, status_type: i16) -> Result<()> {
        sqlx::query("UPDATE topic_timers SET deleted_at = NOW() WHERE topic_id = $1 AND status_type = $2 AND deleted_at IS NULL")
            .bind(topic_id).bind(status_type).execute(pool).await?;
        Ok(())
    }
}
