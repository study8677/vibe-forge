use sqlx::PgPool;
use stevessr_core::error::Result;

/// Handles automatic group membership based on email domain rules
/// and trust level requirements.
pub struct AutoGroupMembership;

impl AutoGroupMembership {
    /// Check and apply automatic group membership rules for a user.
    pub async fn apply_for_user(pool: &PgPool, user_id: i64) -> Result<Vec<i64>> {
        let mut added_groups = Vec::new();

        // Get user's primary email domain
        let email_row: Option<(String,)> = sqlx::query_as(
            "SELECT email FROM user_emails WHERE user_id = $1 AND primary_email = TRUE"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        let email_domain = email_row
            .as_ref()
            .and_then(|(email,)| email.split('@').nth(1))
            .map(|d| d.to_string());

        // Get user trust level
        let user_row: Option<(i16,)> = sqlx::query_as(
            "SELECT trust_level FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        let trust_level = user_row.map(|(tl,)| tl).unwrap_or(0);

        // Find groups with automatic membership configured
        let auto_groups: Vec<(i64, Option<String>, i16)> = sqlx::query_as(
            "SELECT id, automatic_membership_email_domains, automatic_membership_trust_level
             FROM groups WHERE automatic_membership_email_domains IS NOT NULL
                OR automatic_membership_trust_level IS NOT NULL"
        )
        .fetch_all(pool)
        .await?;

        for (group_id, domains, required_tl) in auto_groups {
            let mut should_add = false;

            // Check email domain match
            if let (Some(domains_str), Some(user_domain)) = (&domains, &email_domain) {
                let allowed: Vec<&str> = domains_str.split('|').collect();
                if allowed.iter().any(|d| d.trim() == user_domain) {
                    should_add = true;
                }
            }

            // Check trust level requirement
            if trust_level >= required_tl {
                should_add = true;
            }

            if should_add {
                // Idempotent add
                let existing: Option<(i64,)> = sqlx::query_as(
                    "SELECT id FROM group_users WHERE group_id = $1 AND user_id = $2"
                )
                .bind(group_id)
                .bind(user_id)
                .fetch_optional(pool)
                .await?;

                if existing.is_none() {
                    sqlx::query(
                        "INSERT INTO group_users (group_id, user_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())"
                    )
                    .bind(group_id)
                    .bind(user_id)
                    .execute(pool)
                    .await?;

                    sqlx::query("UPDATE groups SET user_count = user_count + 1 WHERE id = $1")
                        .bind(group_id)
                        .execute(pool)
                        .await?;

                    added_groups.push(group_id);
                }
            }
        }

        Ok(added_groups)
    }
}
