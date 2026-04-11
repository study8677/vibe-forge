use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct PostLocker;

impl PostLocker {
    pub async fn lock(pool: &PgPool, post_id: i64, locked_by_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE posts SET locked_by_id = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(post_id)
        .bind(locked_by_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unlock(pool: &PgPool, post_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE posts SET locked_by_id = NULL, updated_at = NOW() WHERE id = $1"
        )
        .bind(post_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
