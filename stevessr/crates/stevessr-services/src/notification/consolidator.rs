use sqlx::PgPool;
use stevessr_core::error::Result;

/// Consolidates multiple similar notifications into a single summary
/// notification (e.g., "3 people liked your post" instead of 3 separate notifications).
pub struct NotificationConsolidator;

impl NotificationConsolidator {
    /// Attempt to consolidate a new notification with recent similar ones.
    /// Returns true if consolidated (meaning no new row was created), false if a new notification is needed.
    pub async fn try_consolidate(
        pool: &PgPool,
        user_id: i64,
        notification_type: i32,
        topic_id: Option<i64>,
        post_number: Option<i32>,
        data: &serde_json::Value,
    ) -> Result<bool> {
        // Look for an existing unread notification of the same type for the same post
        let existing: Option<(i64, String)> = sqlx::query_as(
            "SELECT id, data FROM notifications
             WHERE user_id = $1 AND notification_type = $2 AND topic_id IS NOT DISTINCT FROM $3
                   AND post_number IS NOT DISTINCT FROM $4 AND read = FALSE
             ORDER BY created_at DESC LIMIT 1"
        )
        .bind(user_id)
        .bind(notification_type)
        .bind(topic_id)
        .bind(post_number)
        .fetch_optional(pool)
        .await?;

        if let Some((existing_id, existing_data_str)) = existing {
            // Parse existing data and merge
            let mut existing_data: serde_json::Value = serde_json::from_str(&existing_data_str)
                .unwrap_or_else(|_| serde_json::json!({}));

            // Increment the consolidated count
            let count = existing_data.get("count")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) + 1;

            existing_data["count"] = serde_json::json!(count);

            // Append the new username to the list of display usernames
            if let Some(new_username) = data.get("display_username") {
                let mut usernames: Vec<serde_json::Value> = existing_data
                    .get("display_usernames")
                    .and_then(|v| v.as_array().cloned())
                    .unwrap_or_default();
                usernames.push(new_username.clone());
                existing_data["display_usernames"] = serde_json::json!(usernames);
            }

            sqlx::query(
                "UPDATE notifications SET data = $2, updated_at = NOW() WHERE id = $1"
            )
            .bind(existing_id)
            .bind(existing_data.to_string())
            .execute(pool)
            .await?;

            return Ok(true);
        }

        Ok(false)
    }
}
