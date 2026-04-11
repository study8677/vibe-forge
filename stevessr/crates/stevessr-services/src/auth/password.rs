use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct PasswordAuthenticator;

impl PasswordAuthenticator {
    /// Authenticate a user by username/email and password.
    pub async fn authenticate(pool: &PgPool, login: &str, password: &str) -> Result<i64> {
        // Find user by username or email
        let user: Option<(i64, bool, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
            "SELECT u.id, u.active, u.suspended_till
             FROM users u
             LEFT JOIN user_emails ue ON ue.user_id = u.id AND ue.primary_email = TRUE
             WHERE u.username_lower = LOWER($1) OR ue.email = LOWER($1)
             LIMIT 1"
        )
        .bind(login)
        .fetch_optional(pool)
        .await?;

        let (user_id, active, suspended_till) = user.ok_or(Error::Unauthorized(
            "invalid username or password".into(),
        ))?;

        if !active {
            return Err(Error::Unauthorized("account is not active".into()));
        }

        if let Some(suspended) = suspended_till {
            if suspended > chrono::Utc::now() {
                return Err(Error::Unauthorized("account is suspended".into()));
            }
        }

        // Get stored password hash
        let hash_row: Option<(String,)> = sqlx::query_as(
            "SELECT value FROM user_custom_fields WHERE user_id = $1 AND name = 'password_hash'"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        let stored_hash = hash_row.ok_or(Error::Unauthorized(
            "invalid username or password".into(),
        ))?.0;

        // Verify password
        Self::verify_password(password, &stored_hash)?;

        // Update last seen
        sqlx::query("UPDATE users SET last_seen_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(user_id)
    }

    fn verify_password(password: &str, stored_hash: &str) -> Result<()> {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};

        let parsed_hash = PasswordHash::new(stored_hash)
            .map_err(|_| Error::Internal("corrupted password hash".into()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::Unauthorized("invalid username or password".into()))?;

        Ok(())
    }

    /// Change a user's password.
    pub async fn change_password(pool: &PgPool, user_id: i64, current_password: &str, new_password: &str) -> Result<()> {
        // Verify current password
        let hash_row: Option<(String,)> = sqlx::query_as(
            "SELECT value FROM user_custom_fields WHERE user_id = $1 AND name = 'password_hash'"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some((hash,)) = hash_row {
            Self::verify_password(current_password, &hash)?;
        }

        // Validate new password
        if new_password.len() < 10 {
            return Err(Error::Validation({
                let mut e = stevessr_core::error::ValidationErrors::new();
                e.add("password", "must be at least 10 characters");
                e
            }));
        }

        // Hash and store new password
        Self::store_password(pool, user_id, new_password).await
    }

    pub async fn store_password(pool: &PgPool, user_id: i64, password: &str) -> Result<()> {
        use argon2::{Argon2, PasswordHasher};
        use argon2::password_hash::SaltString;
        use rand::rngs::OsRng;

        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Error::Internal(format!("password hash failed: {}", e)))?
            .to_string();

        sqlx::query(
            "INSERT INTO user_custom_fields (user_id, name, value, created_at, updated_at)
             VALUES ($1, 'password_hash', $2, NOW(), NOW())
             ON CONFLICT (user_id, name) DO UPDATE SET value = $2, updated_at = NOW()"
        )
        .bind(user_id)
        .bind(&hash)
        .execute(pool)
        .await?;

        Ok(())
    }
}
