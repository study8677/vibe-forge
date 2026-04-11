use sqlx::PgPool;
use stevessr_core::error::Result;

/// Handles bookmark reminder notifications.
pub struct BookmarkReminder;

impl BookmarkReminder {
    /// Find all bookmarks with reminders that are due now.
    pub async fn find_due_reminders(pool: &PgPool) -> Result<Vec<(i64, i64, String, i64, Option<String>)>> {
        let rows: Vec<(i64, i64, String, i64, Option<String>)> = sqlx::query_as(
            "SELECT id, user_id, bookmarkable_type, bookmarkable_id, name
             FROM bookmarks
             WHERE reminder_at IS NOT NULL AND reminder_at <= NOW() AND reminder_sent_at IS NULL"
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    /// Send reminder notifications for all due bookmarks.
    pub async fn process_due_reminders(pool: &PgPool) -> Result<u64> {
        let due = Self::find_due_reminders(pool).await?;
        let mut sent = 0u64;

        for (bookmark_id, user_id, bookmarkable_type, bookmarkable_id, name) in due {
            // Create a notification
            let data = serde_json::json!({
                "bookmark_name": name.unwrap_or_default(),
                "bookmarkable_type": bookmarkable_type,
                "bookmarkable_id": bookmarkable_id,
            });

            // notification_type 16 = bookmark reminder
            sqlx::query(
                "INSERT INTO notifications (notification_type, user_id, data, created_at, updated_at)
                 VALUES (16, $1, $2, NOW(), NOW())"
            )
            .bind(user_id)
            .bind(data.to_string())
            .execute(pool)
            .await?;

            // Mark reminder as sent
            sqlx::query("UPDATE bookmarks SET reminder_sent_at = NOW() WHERE id = $1")
                .bind(bookmark_id)
                .execute(pool)
                .await?;

            sent += 1;
        }

        Ok(sent)
    }
}
