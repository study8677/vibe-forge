use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use rand::Rng;

pub struct SessionManager;

impl SessionManager {
    /// Create a new session for a user. Returns the session token.
    pub async fn create(pool: &PgPool, user_id: i64, ip_address: Option<&str>, user_agent: Option<&str>) -> Result<String> {
        let token = Self::generate_token();

        sqlx::query(
            "INSERT INTO user_auth_tokens (user_id, auth_token, prev_auth_token, user_agent, client_ip, seen_at, rotated_at, created_at, updated_at)
             VALUES ($1, $2, $2, $3, $4, NOW(), NOW(), NOW(), NOW())"
        )
        .bind(user_id)
        .bind(&token)
        .bind(user_agent)
        .bind(ip_address)
        .execute(pool)
        .await?;

        // Update user last seen
        sqlx::query("UPDATE users SET last_seen_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(token)
    }

    /// Validate a session token and return the user_id.
    pub async fn validate(pool: &PgPool, token: &str) -> Result<i64> {
        let row: Option<(i64,)> = sqlx::query_as(
            "SELECT user_id FROM user_auth_tokens
             WHERE (auth_token = $1 OR prev_auth_token = $1)
             LIMIT 1"
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        let (user_id,) = row.ok_or(Error::Unauthorized("invalid session token".into()))?;

        // Update seen_at
        sqlx::query("UPDATE user_auth_tokens SET seen_at = NOW() WHERE auth_token = $1 OR prev_auth_token = $1")
            .bind(token)
            .execute(pool)
            .await?;

        Ok(user_id)
    }

    /// Rotate a session token (security best practice).
    pub async fn rotate(pool: &PgPool, current_token: &str) -> Result<String> {
        let new_token = Self::generate_token();

        let result = sqlx::query(
            "UPDATE user_auth_tokens SET prev_auth_token = auth_token, auth_token = $2, rotated_at = NOW()
             WHERE auth_token = $1"
        )
        .bind(current_token)
        .bind(&new_token)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::Unauthorized("invalid session token".into()));
        }

        Ok(new_token)
    }

    /// Destroy a specific session.
    pub async fn destroy(pool: &PgPool, token: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_auth_tokens WHERE auth_token = $1 OR prev_auth_token = $1")
            .bind(token)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Destroy all sessions for a user (e.g., after password change).
    pub async fn destroy_all(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM user_auth_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Clean up expired sessions older than the given number of days.
    pub async fn cleanup_expired(pool: &PgPool, max_age_days: i32) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM user_auth_tokens WHERE seen_at < NOW() - ($1 || ' days')::INTERVAL"
        )
        .bind(max_age_days.to_string())
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    fn generate_token() -> String {
        use rand::RngCore;
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
}
