use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct PostDestroyer;

impl PostDestroyer {
    pub async fn destroy(pool: &PgPool, post_id: i64, deleted_by_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE posts SET deleted_at = NOW(), deleted_by_id = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(post_id)
        .bind(deleted_by_id)
        .execute(pool)
        .await?;

        // Update topic post count
        sqlx::query(
            "UPDATE topics SET posts_count = posts_count - 1, updated_at = NOW()
             WHERE id = (SELECT topic_id FROM posts WHERE id = $1)"
        )
        .bind(post_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn recover(pool: &PgPool, post_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE posts SET deleted_at = NULL, deleted_by_id = NULL, updated_at = NOW() WHERE id = $1"
        )
        .bind(post_id)
        .execute(pool)
        .await?;

        // Update topic post count
        sqlx::query(
            "UPDATE topics SET posts_count = posts_count + 1, updated_at = NOW()
             WHERE id = (SELECT topic_id FROM posts WHERE id = $1)"
        )
        .bind(post_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
