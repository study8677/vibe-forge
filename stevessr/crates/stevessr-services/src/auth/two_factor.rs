use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct TwoFactorManager;

impl TwoFactorManager {
    /// Generate a new TOTP secret for a user.
    pub async fn generate_secret(pool: &PgPool, user_id: i64) -> Result<String> {
        use rand::RngCore;

        let mut secret_bytes = [0u8; 20];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        let secret = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &secret_bytes);

        // Store the secret (not yet enabled)
        sqlx::query(
            "INSERT INTO user_second_factors (user_id, method, data, enabled, created_at, updated_at)
             VALUES ($1, 1, $2, FALSE, NOW(), NOW())
             ON CONFLICT (user_id, method) DO UPDATE SET data = $2, enabled = FALSE, updated_at = NOW()"
        )
        .bind(user_id)
        .bind(&secret)
        .execute(pool)
        .await?;

        Ok(secret)
    }

    /// Enable 2FA after the user confirms with a valid TOTP code.
    pub async fn enable(pool: &PgPool, user_id: i64, code: &str) -> Result<Vec<String>> {
        let secret_row: Option<(String,)> = sqlx::query_as(
            "SELECT data FROM user_second_factors WHERE user_id = $1 AND method = 1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        let secret = secret_row.ok_or(Error::NotFound {
            resource: "2fa_secret",
            id: user_id.to_string(),
        })?.0;

        // Verify the TOTP code
        if !Self::verify_totp(&secret, code) {
            return Err(Error::Unauthorized("invalid 2FA code".into()));
        }

        // Enable 2FA
        sqlx::query(
            "UPDATE user_second_factors SET enabled = TRUE, updated_at = NOW() WHERE user_id = $1 AND method = 1"
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        // Generate backup codes
        let backup_codes = Self::generate_backup_codes(pool, user_id).await?;

        Ok(backup_codes)
    }

    /// Disable 2FA for a user.
    pub async fn disable(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM user_second_factors WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Verify a TOTP code during login.
    pub async fn verify(pool: &PgPool, user_id: i64, code: &str) -> Result<bool> {
        // Check TOTP
        let secret_row: Option<(String, bool)> = sqlx::query_as(
            "SELECT data, enabled FROM user_second_factors WHERE user_id = $1 AND method = 1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some((secret, true)) = secret_row {
            if Self::verify_totp(&secret, code) {
                return Ok(true);
            }
        }

        // Check backup codes
        let backup: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM user_second_factors
             WHERE user_id = $1 AND method = 2 AND data = $2 AND enabled = TRUE"
        )
        .bind(user_id)
        .bind(code)
        .fetch_optional(pool)
        .await?;

        if let Some((backup_id,)) = backup {
            // Consume the backup code
            sqlx::query("UPDATE user_second_factors SET enabled = FALSE, updated_at = NOW() WHERE id = $1")
                .bind(backup_id)
                .execute(pool)
                .await?;
            return Ok(true);
        }

        Ok(false)
    }

    /// Check if a user has 2FA enabled.
    pub async fn is_enabled(pool: &PgPool, user_id: i64) -> Result<bool> {
        let row: Option<(bool,)> = sqlx::query_as(
            "SELECT enabled FROM user_second_factors WHERE user_id = $1 AND method = 1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|(e,)| e).unwrap_or(false))
    }

    fn verify_totp(secret: &str, code: &str) -> bool {
        // Simple TOTP verification using HMAC-SHA1
        // In production, use a proper TOTP library
        let time_step = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 30;

        // Check current step and adjacent steps for clock drift
        for offset in [-1i64, 0, 1] {
            let counter = (time_step as i64 + offset) as u64;
            let expected = Self::generate_totp_code(secret, counter);
            if expected == code {
                return true;
            }
        }

        false
    }

    fn generate_totp_code(secret: &str, counter: u64) -> String {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;

        let secret_bytes = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret)
            .unwrap_or_default();

        let counter_bytes = counter.to_be_bytes();
        let mut mac = Hmac::<Sha1>::new_from_slice(&secret_bytes).unwrap();
        mac.update(&counter_bytes);
        let result = mac.finalize().into_bytes();

        let offset = (result[19] & 0x0f) as usize;
        let code = ((result[offset] as u32 & 0x7f) << 24)
            | ((result[offset + 1] as u32) << 16)
            | ((result[offset + 2] as u32) << 8)
            | (result[offset + 3] as u32);

        format!("{:06}", code % 1_000_000)
    }

    async fn generate_backup_codes(pool: &PgPool, user_id: i64) -> Result<Vec<String>> {
        use rand::Rng;

        // Remove existing backup codes
        sqlx::query("DELETE FROM user_second_factors WHERE user_id = $1 AND method = 2")
            .bind(user_id)
            .execute(pool)
            .await?;

        let mut codes = Vec::new();
        for _ in 0..10 {
            let code: u32 = rand::thread_rng().gen_range(10000000..99999999);
            let code_str = code.to_string();

            sqlx::query(
                "INSERT INTO user_second_factors (user_id, method, data, enabled, created_at, updated_at)
                 VALUES ($1, 2, $2, TRUE, NOW(), NOW())"
            )
            .bind(user_id)
            .bind(&code_str)
            .execute(pool)
            .await?;

            codes.push(code_str);
        }

        Ok(codes)
    }
}
