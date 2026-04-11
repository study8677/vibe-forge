use sqlx::PgPool;
use stevessr_core::error::Result;

/// Logs staff actions for audit trail.
pub struct StaffActionLogger;

impl StaffActionLogger {
    pub async fn log(
        pool: &PgPool,
        staff_user_id: i64,
        action_type: &str,
        target_user_id: Option<i64>,
        subject: Option<&str>,
        details: Option<&str>,
        context: Option<&str>,
        ip_address: Option<&str>,
    ) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO user_histories (acting_user_id, action, target_user_id, subject, details, context, ip_address, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW()) RETURNING id"
        )
        .bind(staff_user_id)
        .bind(action_type)
        .bind(target_user_id)
        .bind(subject)
        .bind(details)
        .bind(context)
        .bind(ip_address)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    /// Log a user silence action.
    pub async fn log_silence(pool: &PgPool, staff_user_id: i64, target_user_id: i64, reason: &str) -> Result<()> {
        Self::log(
            pool,
            staff_user_id,
            "silence_user",
            Some(target_user_id),
            None,
            Some(reason),
            None,
            None,
        ).await?;
        Ok(())
    }

    /// Log a user suspension action.
    pub async fn log_suspend(pool: &PgPool, staff_user_id: i64, target_user_id: i64, reason: &str) -> Result<()> {
        Self::log(
            pool,
            staff_user_id,
            "suspend_user",
            Some(target_user_id),
            None,
            Some(reason),
            None,
            None,
        ).await?;
        Ok(())
    }

    /// Log a topic deletion.
    pub async fn log_delete_topic(pool: &PgPool, staff_user_id: i64, topic_id: i64) -> Result<()> {
        Self::log(
            pool,
            staff_user_id,
            "delete_topic",
            None,
            Some(&topic_id.to_string()),
            None,
            None,
            None,
        ).await?;
        Ok(())
    }

    /// Log a post deletion.
    pub async fn log_delete_post(pool: &PgPool, staff_user_id: i64, post_id: i64) -> Result<()> {
        Self::log(
            pool,
            staff_user_id,
            "delete_post",
            None,
            Some(&post_id.to_string()),
            None,
            None,
            None,
        ).await?;
        Ok(())
    }

    /// Log an impersonation start.
    pub async fn log_impersonate(pool: &PgPool, staff_user_id: i64, target_user_id: i64) -> Result<()> {
        Self::log(
            pool,
            staff_user_id,
            "impersonate",
            Some(target_user_id),
            None,
            None,
            None,
            None,
        ).await?;
        Ok(())
    }

    /// Retrieve recent staff actions for the admin log page.
    pub async fn recent(pool: &PgPool, limit: i64) -> Result<Vec<(i64, i64, String, Option<i64>, Option<String>, chrono::DateTime<chrono::Utc>)>> {
        let rows = sqlx::query_as(
            "SELECT id, acting_user_id, action, target_user_id, details, created_at
             FROM user_histories
             WHERE action NOT LIKE 'check_%'
             ORDER BY created_at DESC
             LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }
}
