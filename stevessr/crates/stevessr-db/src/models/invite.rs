use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Invite {
    pub id: i64,
    pub invite_key: String,
    pub email: Option<String>,
    pub invited_by_id: i64,
    pub max_redemptions_allowed: i32,
    pub redemption_count: i32,
    pub expires_at: DateTime<Utc>,
    pub invalidated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_id: Option<i64>,
    pub custom_message: Option<String>,
    pub domain: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Invite {
    pub async fn find_by_key(pool: &PgPool, invite_key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM invites WHERE invite_key = $1 AND deleted_at IS NULL")
            .bind(invite_key)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_inviter(pool: &PgPool, invited_by_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM invites WHERE invited_by_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC LIMIT $2",
        )
        .bind(invited_by_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        invite_key: &str,
        email: Option<&str>,
        invited_by_id: i64,
        max_redemptions: i32,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO invites (invite_key, email, invited_by_id, max_redemptions_allowed, expires_at)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(invite_key)
        .bind(email)
        .bind(invited_by_id)
        .bind(max_redemptions)
        .bind(expires_at)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_redemption(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE invites SET redemption_count = redemption_count + 1, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn invalidate(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE invites SET invalidated_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
