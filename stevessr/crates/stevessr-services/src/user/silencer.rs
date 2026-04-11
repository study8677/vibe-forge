use chrono::{DateTime, Utc};
use sqlx::PgPool;
use stevessr_core::error::Result;
use stevessr_db::models::user::User;

pub struct UserSilencer;

impl UserSilencer {
    pub async fn silence(pool: &PgPool, user_id: i64, until: DateTime<Utc>, _reason: &str, _by_user_id: i64) -> Result<()> {
        User::silence(pool, user_id, until).await?;
        // TODO: log to user_histories, create notification
        Ok(())
    }

    pub async fn unsilence(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query("UPDATE users SET silenced_till = NULL, updated_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
