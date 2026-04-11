use sqlx::PgPool;
use stevessr_core::error::Result;
use stevessr_db::models::user::User;

pub struct UserActivator;

impl UserActivator {
    pub async fn activate(pool: &PgPool, user_id: i64) -> Result<()> {
        User::activate(pool, user_id).await?;
        Ok(())
    }

    pub async fn deactivate(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query("UPDATE users SET active = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
