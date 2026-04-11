use sqlx::PgPool;
use chrono::{DateTime, Utc};
use stevessr_core::error::Result;

pub struct TopicStatusUpdater;

impl TopicStatusUpdater {
    pub async fn close(pool: &PgPool, topic_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET closed = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).execute(pool).await?;
        Ok(())
    }

    pub async fn open(pool: &PgPool, topic_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET closed = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).execute(pool).await?;
        Ok(())
    }

    pub async fn archive(pool: &PgPool, topic_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET archived = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).execute(pool).await?;
        Ok(())
    }

    pub async fn unarchive(pool: &PgPool, topic_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET archived = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).execute(pool).await?;
        Ok(())
    }

    pub async fn pin(pool: &PgPool, topic_id: i64, globally: bool, until: Option<DateTime<Utc>>) -> Result<()> {
        sqlx::query("UPDATE topics SET pinned_at = NOW(), pinned_globally = $2, pinned_until = $3, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).bind(globally).bind(until).execute(pool).await?;
        Ok(())
    }

    pub async fn unpin(pool: &PgPool, topic_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET pinned_at = NULL, pinned_globally = FALSE, pinned_until = NULL, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).execute(pool).await?;
        Ok(())
    }

    pub async fn set_visible(pool: &PgPool, topic_id: i64, visible: bool) -> Result<()> {
        sqlx::query("UPDATE topics SET visible = $2, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).bind(visible).execute(pool).await?;
        Ok(())
    }

    pub async fn set_slow_mode(pool: &PgPool, topic_id: i64, seconds: i32) -> Result<()> {
        sqlx::query("UPDATE topics SET slow_mode_seconds = $2, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).bind(seconds).execute(pool).await?;
        Ok(())
    }
}
