use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct OAuth2Provider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub scopes: Vec<String>,
}

pub struct OAuth2UserInfo {
    pub provider_uid: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

pub struct OAuth2Manager;

impl OAuth2Manager {
    /// Generate the authorization URL for an OAuth2 provider.
    pub fn authorize_url(provider: &OAuth2Provider, redirect_uri: &str, state: &str) -> String {
        let scopes = provider.scopes.join(" ");
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            provider.authorize_url,
            urlencoding::encode(&provider.client_id),
            urlencoding::encode(redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state),
        )
    }

    /// Exchange an authorization code for user info and find/create the associated user.
    pub async fn authenticate(
        pool: &PgPool,
        provider_name: &str,
        provider_uid: &str,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<i64> {
        // Check for existing OAuth2 user association
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT user_id FROM oauth2_user_infos WHERE provider = $1 AND uid = $2"
        )
        .bind(provider_name)
        .bind(provider_uid)
        .fetch_optional(pool)
        .await?;

        if let Some((user_id,)) = existing {
            return Ok(user_id);
        }

        // If email is provided, try to find an existing user with that email
        if let Some(email_addr) = email {
            let user_by_email: Option<(i64,)> = sqlx::query_as(
                "SELECT user_id FROM user_emails WHERE email = $1"
            )
            .bind(email_addr)
            .fetch_optional(pool)
            .await?;

            if let Some((user_id,)) = user_by_email {
                // Associate this OAuth2 identity with the existing user
                Self::create_association(pool, user_id, provider_name, provider_uid, email, name).await?;
                return Ok(user_id);
            }
        }

        // No existing user found; caller must handle account creation
        Err(Error::NotFound {
            resource: "user",
            id: format!("oauth2:{}:{}", provider_name, provider_uid),
        })
    }

    /// Create an OAuth2 user association.
    pub async fn create_association(
        pool: &PgPool,
        user_id: i64,
        provider: &str,
        uid: &str,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO oauth2_user_infos (user_id, provider, uid, email, name, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             ON CONFLICT (provider, uid) DO UPDATE SET email = $4, name = $5, updated_at = NOW()"
        )
        .bind(user_id)
        .bind(provider)
        .bind(uid)
        .bind(email)
        .bind(name)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Remove an OAuth2 association.
    pub async fn remove_association(pool: &PgPool, user_id: i64, provider: &str) -> Result<()> {
        sqlx::query(
            "DELETE FROM oauth2_user_infos WHERE user_id = $1 AND provider = $2"
        )
        .bind(user_id)
        .bind(provider)
        .execute(pool)
        .await?;
        Ok(())
    }
}
