use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use rand::RngCore;

pub struct InviteGenerator;

impl InviteGenerator {
    /// Generate a new invite link.
    pub async fn generate(
        pool: &PgPool,
        invited_by_id: i64,
        email: Option<&str>,
        group_ids: &[i64],
        topic_id: Option<i64>,
        max_redemptions: Option<i32>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<String> {
        let invite_key = Self::generate_key();

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO invites (invite_key, invited_by_id, email, max_redemptions_allowed, expires_at, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id"
        )
        .bind(&invite_key)
        .bind(invited_by_id)
        .bind(email)
        .bind(max_redemptions.unwrap_or(1))
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        let invite_id = row.0;

        // Associate groups
        for gid in group_ids {
            sqlx::query(
                "INSERT INTO invited_groups (invite_id, group_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())"
            )
            .bind(invite_id)
            .bind(gid)
            .execute(pool)
            .await?;
        }

        // Associate topic
        if let Some(tid) = topic_id {
            sqlx::query(
                "UPDATE invites SET topic_id = $2 WHERE id = $1"
            )
            .bind(invite_id)
            .bind(tid)
            .execute(pool)
            .await?;
        }

        Ok(invite_key)
    }

    /// Revoke an invite.
    pub async fn revoke(pool: &PgPool, invite_id: i64) -> Result<()> {
        sqlx::query("UPDATE invites SET invalidated_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(invite_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Get invite details by key.
    pub async fn find_by_key(pool: &PgPool, key: &str) -> Result<Option<(i64, i64, Option<String>, i32, i32)>> {
        let row: Option<(i64, i64, Option<String>, i32, i32)> = sqlx::query_as(
            "SELECT id, invited_by_id, email, max_redemptions_allowed, redemption_count
             FROM invites
             WHERE invite_key = $1 AND invalidated_at IS NULL AND (expires_at IS NULL OR expires_at > NOW())"
        )
        .bind(key)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    fn generate_key() -> String {
        let mut bytes = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
}
