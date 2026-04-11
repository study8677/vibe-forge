use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct UserDestroyer;

impl UserDestroyer {
    pub async fn destroy(pool: &PgPool, user_id: i64) -> Result<()> {
        // Cascade deletes handle most cleanup via FK constraints
        stevessr_db::models::user::User::delete(pool, user_id).await?;
        Ok(())
    }
}
