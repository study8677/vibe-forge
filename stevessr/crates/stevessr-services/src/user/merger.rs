use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct UserMerger;

impl UserMerger {
    pub async fn merge(pool: &PgPool, source_user_id: i64, target_user_id: i64) -> Result<()> {
        // Transfer posts
        sqlx::query("UPDATE posts SET user_id = $2 WHERE user_id = $1")
            .bind(source_user_id).bind(target_user_id).execute(pool).await?;

        // Transfer topics
        sqlx::query("UPDATE topics SET user_id = $2 WHERE user_id = $1")
            .bind(source_user_id).bind(target_user_id).execute(pool).await?;

        // Transfer notifications
        sqlx::query("UPDATE notifications SET user_id = $2 WHERE user_id = $1")
            .bind(source_user_id).bind(target_user_id).execute(pool).await?;

        // Transfer post actions (likes, flags)
        sqlx::query("UPDATE post_actions SET user_id = $2 WHERE user_id = $1")
            .bind(source_user_id).bind(target_user_id).execute(pool).await?;

        // Delete source user
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(source_user_id).execute(pool).await?;

        Ok(())
    }
}
