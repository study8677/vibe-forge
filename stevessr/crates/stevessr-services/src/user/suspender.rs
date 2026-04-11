use chrono::{DateTime, Utc};
use sqlx::PgPool;
use stevessr_core::error::Result;
use stevessr_db::models::user::User;

pub struct UserSuspender;

impl UserSuspender {
    pub async fn suspend(pool: &PgPool, user_id: i64, until: DateTime<Utc>, _reason: &str, _by_user_id: i64) -> Result<()> {
        User::suspend(pool, user_id, until).await?;
        // TODO: revoke all auth tokens, log to user_histories
        Ok(())
    }

    pub async fn unsuspend(pool: &PgPool, user_id: i64) -> Result<()> {
        User::unsuspend(pool, user_id).await?;
        Ok(())
    }
}
