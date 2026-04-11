use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailLog {
    pub id: i64,
    pub to_address: String,
    pub email_type: String,
    pub user_id: Option<i64>,
    pub post_id: Option<i64>,
    pub bounce_key: Option<String>,
    pub bounced: bool,
    pub message_id: Option<String>,
    pub smtp_group_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EmailLog {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM email_logs WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM email_logs WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        to_address: &str,
        email_type: &str,
        user_id: Option<i64>,
        post_id: Option<i64>,
        bounce_key: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO email_logs (to_address, email_type, user_id, post_id, bounce_key)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(to_address)
        .bind(email_type)
        .bind(user_id)
        .bind(post_id)
        .bind(bounce_key)
        .fetch_one(pool)
        .await
    }

    pub async fn mark_bounced(pool: &PgPool, bounce_key: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE email_logs SET bounced = TRUE, updated_at = NOW() WHERE bounce_key = $1")
            .bind(bounce_key)
            .execute(pool)
            .await?;
        Ok(())
    }
}
