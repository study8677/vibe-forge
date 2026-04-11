use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct TopicDestroyer;

impl TopicDestroyer {
    pub async fn destroy(pool: &PgPool, topic_id: i64, deleted_by_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET deleted_at = NOW(), deleted_by_id = $2, visible = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).bind(deleted_by_id).execute(pool).await?;
        // Soft delete all posts
        sqlx::query("UPDATE posts SET deleted_at = NOW(), deleted_by_id = $2 WHERE topic_id = $1 AND deleted_at IS NULL")
            .bind(topic_id).bind(deleted_by_id).execute(pool).await?;
        Ok(())
    }

    pub async fn recover(pool: &PgPool, topic_id: i64) -> Result<()> {
        sqlx::query("UPDATE topics SET deleted_at = NULL, deleted_by_id = NULL, visible = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(topic_id).execute(pool).await?;
        sqlx::query("UPDATE posts SET deleted_at = NULL, deleted_by_id = NULL WHERE topic_id = $1")
            .bind(topic_id).execute(pool).await?;
        Ok(())
    }
}
