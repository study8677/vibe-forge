use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct InviteRedeemer;

impl InviteRedeemer {
    /// Redeem an invite for a user.
    pub async fn redeem(pool: &PgPool, invite_key: &str, user_id: i64) -> Result<()> {
        // Find the invite
        let invite = super::generator::InviteGenerator::find_by_key(pool, invite_key).await?;
        let (invite_id, invited_by_id, _email, max_redemptions, current_count) = invite.ok_or(Error::NotFound {
            resource: "invite",
            id: invite_key.to_string(),
        })?;

        // Check max redemptions
        if current_count >= max_redemptions {
            return Err(Error::Forbidden("invite has been fully redeemed".into()));
        }

        // Check not already redeemed by this user
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM invited_users WHERE invite_id = $1 AND user_id = $2"
        )
        .bind(invite_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(Error::AlreadyExists {
                resource: "invited_user",
                detail: "you already redeemed this invite".into(),
            });
        }

        // Record the redemption
        sqlx::query(
            "INSERT INTO invited_users (invite_id, user_id, redeemed_at, created_at, updated_at) VALUES ($1, $2, NOW(), NOW(), NOW())"
        )
        .bind(invite_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        // Increment redemption count
        sqlx::query("UPDATE invites SET redemption_count = redemption_count + 1, updated_at = NOW() WHERE id = $1")
            .bind(invite_id)
            .execute(pool)
            .await?;

        // Add user to invited groups
        let groups: Vec<(i64,)> = sqlx::query_as(
            "SELECT group_id FROM invited_groups WHERE invite_id = $1"
        )
        .bind(invite_id)
        .fetch_all(pool)
        .await?;

        for (group_id,) in groups {
            sqlx::query(
                "INSERT INTO group_users (group_id, user_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) ON CONFLICT DO NOTHING"
            )
            .bind(group_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        }

        // Notify the inviter
        sqlx::query(
            "INSERT INTO notifications (notification_type, user_id, data, created_at, updated_at)
             VALUES (8, $1, $2, NOW(), NOW())"
        )
        .bind(invited_by_id)
        .bind(serde_json::json!({"display_username": user_id.to_string()}).to_string())
        .execute(pool)
        .await?;

        Ok(())
    }
}
