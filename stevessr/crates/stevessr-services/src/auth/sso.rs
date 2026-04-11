use stevessr_core::error::{Error, Result};
use sqlx::PgPool;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// SSO (Single Sign-On) configuration and payload handling.
/// Implements the Discourse SSO protocol.
pub struct SsoManager {
    secret: String,
}

#[derive(Debug)]
pub struct SsoPayload {
    pub nonce: String,
    pub return_sso_url: String,
    pub external_id: String,
    pub email: String,
    pub username: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub admin: Option<bool>,
    pub moderator: Option<bool>,
    pub groups: Vec<String>,
}

impl SsoManager {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Generate an SSO request payload (sent to the SSO provider).
    pub fn generate_request(&self, nonce: &str, return_url: &str) -> Result<(String, String)> {
        let raw_payload = format!("nonce={}&return_sso_url={}", nonce, urlencoding::encode(return_url));
        let encoded_payload = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            raw_payload.as_bytes(),
        );

        let sig = self.sign(&encoded_payload)?;

        Ok((encoded_payload, sig))
    }

    /// Validate and parse an SSO response payload (received from the SSO provider).
    pub fn parse_response(&self, sso: &str, sig: &str) -> Result<SsoPayload> {
        // Verify signature
        let expected_sig = self.sign(sso)?;
        if !constant_time_eq(sig.as_bytes(), expected_sig.as_bytes()) {
            return Err(Error::Unauthorized("invalid SSO signature".into()));
        }

        // Decode payload
        let decoded = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            sso.as_bytes(),
        )
        .map_err(|_| Error::Unauthorized("invalid SSO payload encoding".into()))?;

        let payload_str = String::from_utf8(decoded)
            .map_err(|_| Error::Unauthorized("invalid SSO payload".into()))?;

        // Parse query params
        let params: std::collections::HashMap<String, String> = payload_str
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?.to_string();
                let value = parts.next().map(|v| urlencoding::decode(v).unwrap_or_default().into_owned()).unwrap_or_default();
                Some((key, value))
            })
            .collect();

        Ok(SsoPayload {
            nonce: params.get("nonce").cloned().unwrap_or_default(),
            return_sso_url: params.get("return_sso_url").cloned().unwrap_or_default(),
            external_id: params.get("external_id").cloned().unwrap_or_default(),
            email: params.get("email").cloned().unwrap_or_default(),
            username: params.get("username").cloned(),
            name: params.get("name").cloned(),
            avatar_url: params.get("avatar_url").cloned(),
            admin: params.get("admin").and_then(|v| v.parse().ok()),
            moderator: params.get("moderator").and_then(|v| v.parse().ok()),
            groups: params.get("groups")
                .map(|g| g.split(',').map(String::from).collect())
                .unwrap_or_default(),
        })
    }

    /// Find or create a user from an SSO payload.
    pub async fn find_or_create_user(pool: &PgPool, payload: &SsoPayload) -> Result<i64> {
        // Check for existing SSO record
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT user_id FROM single_sign_on_records WHERE external_id = $1"
        )
        .bind(&payload.external_id)
        .fetch_optional(pool)
        .await?;

        if let Some((user_id,)) = existing {
            return Ok(user_id);
        }

        // Check by email
        let user_by_email: Option<(i64,)> = sqlx::query_as(
            "SELECT user_id FROM user_emails WHERE email = $1"
        )
        .bind(&payload.email)
        .fetch_optional(pool)
        .await?;

        if let Some((user_id,)) = user_by_email {
            // Create SSO association
            sqlx::query(
                "INSERT INTO single_sign_on_records (user_id, external_id, external_email, created_at, updated_at)
                 VALUES ($1, $2, $3, NOW(), NOW())"
            )
            .bind(user_id)
            .bind(&payload.external_id)
            .bind(&payload.email)
            .execute(pool)
            .await?;
            return Ok(user_id);
        }

        Err(Error::NotFound {
            resource: "user",
            id: format!("sso:{}", payload.external_id),
        })
    }

    fn sign(&self, payload: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|_| Error::Internal("HMAC key error".into()))?;
        mac.update(payload.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
