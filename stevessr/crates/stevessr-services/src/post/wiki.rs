use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct PostWiki;

impl PostWiki {
    pub async fn toggle_wiki(pool: &PgPool, post_id: i64, wiki: bool) -> Result<()> {
        sqlx::query("UPDATE posts SET wiki = $2, updated_at = NOW() WHERE id = $1")
            .bind(post_id)
            .bind(wiki)
            .execute(pool)
            .await?;
        Ok(())
    }
}
