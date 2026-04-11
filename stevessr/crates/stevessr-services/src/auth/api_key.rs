use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use rand::RngCore;

pub struct ApiKeyManager;

impl ApiKeyManager {
    /// Create a new API key for a user or as a global admin key.
    pub async fn create(
        pool: &PgPool,
        created_by_id: i64,
        user_id: Option<i64>,
        description: &str,
    ) -> Result<(i64, String)> {
        let key = Self::generate_key();
        let key_hash = Self::hash_key(&key);

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO api_keys (key_hash, truncated_key, user_id, created_by_id, description, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id"
        )
        .bind(&key_hash)
        .bind(&key[..8]) // Store first 8 chars for display
        .bind(user_id)
        .bind(created_by_id)
        .bind(description)
        .fetch_one(pool)
        .await?;

        Ok((row.0, key))
    }

    /// Authenticate a request using an API key.
    pub async fn authenticate(pool: &PgPool, key: &str) -> Result<(i64, Option<i64>)> {
        let key_hash = Self::hash_key(key);

        let row: Option<(i64, Option<i64>, bool)> = sqlx::query_as(
            "SELECT id, user_id, revoked FROM api_keys WHERE key_hash = $1"
        )
        .bind(&key_hash)
        .fetch_optional(pool)
        .await?;

        let (key_id, user_id, revoked) = row.ok_or(Error::Unauthorized("invalid API key".into()))?;

        if revoked {
            return Err(Error::Unauthorized("API key has been revoked".into()));
        }

        // Update last used timestamp
        sqlx::query("UPDATE api_keys SET last_used_at = NOW() WHERE id = $1")
            .bind(key_id)
            .execute(pool)
            .await?;

        Ok((key_id, user_id))
    }

    /// Revoke an API key.
    pub async fn revoke(pool: &PgPool, key_id: i64) -> Result<()> {
        sqlx::query("UPDATE api_keys SET revoked = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(key_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// List API keys (with truncated keys only for security).
    pub async fn list(pool: &PgPool) -> Result<Vec<(i64, String, Option<i64>, String, bool)>> {
        let rows: Vec<(i64, String, Option<i64>, String, bool)> = sqlx::query_as(
            "SELECT id, truncated_key, user_id, description, revoked
             FROM api_keys
             ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    fn generate_key() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    fn hash_key(key: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
