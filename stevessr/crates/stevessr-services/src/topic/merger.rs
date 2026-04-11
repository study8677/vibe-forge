use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct TopicMerger;

impl TopicMerger {
    pub async fn merge(pool: &PgPool, source_topic_id: i64, target_topic_id: i64) -> Result<()> {
        // Move all posts from source to target
        sqlx::query("UPDATE posts SET topic_id = $2, updated_at = NOW() WHERE topic_id = $1")
            .bind(source_topic_id).bind(target_topic_id).execute(pool).await?;

        // Re-number posts in target
        sqlx::query(
            "WITH numbered AS (SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) as rn FROM posts WHERE topic_id = $1 AND deleted_at IS NULL)
             UPDATE posts SET post_number = numbered.rn FROM numbered WHERE posts.id = numbered.id"
        ).bind(target_topic_id).execute(pool).await?;

        // Soft-delete source topic
        sqlx::query("UPDATE topics SET deleted_at = NOW(), visible = FALSE WHERE id = $1")
            .bind(source_topic_id).execute(pool).await?;

        Ok(())
    }
}
