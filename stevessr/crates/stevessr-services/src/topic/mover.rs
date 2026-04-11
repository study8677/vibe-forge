use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct PostMover;

impl PostMover {
    pub async fn move_posts(pool: &PgPool, source_topic_id: i64, post_ids: &[i64], target_topic_id: i64) -> Result<()> {
        for post_id in post_ids {
            sqlx::query("UPDATE posts SET topic_id = $2, updated_at = NOW() WHERE id = $1 AND topic_id = $3")
                .bind(post_id).bind(target_topic_id).bind(source_topic_id)
                .execute(pool).await?;
        }
        // Update post counts
        sqlx::query("UPDATE topics SET posts_count = (SELECT COUNT(*) FROM posts WHERE topic_id = topics.id AND deleted_at IS NULL), updated_at = NOW() WHERE id IN ($1, $2)")
            .bind(source_topic_id).bind(target_topic_id)
            .execute(pool).await?;
        Ok(())
    }
}
