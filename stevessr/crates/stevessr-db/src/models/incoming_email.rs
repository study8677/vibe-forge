use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IncomingEmail {
    pub id: i64,
    pub user_id: Option<i64>,
    pub topic_id: Option<i64>,
    pub post_id: Option<i64>,
    pub raw: Option<String>,
    pub error: Option<String>,
    pub message_id: String,
    pub from_address: String,
    pub to_addresses: Option<String>,
    pub cc_addresses: Option<String>,
    pub subject: Option<String>,
    pub rejection_message: Option<String>,
    pub is_auto_generated: bool,
    pub is_bounce: bool,
    pub imap_uid_validity: Option<i32>,
    pub imap_uid: Option<i32>,
    pub imap_sync: Option<bool>,
    pub imap_group_id: Option<i64>,
    pub created_via: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IncomingEmail {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM incoming_emails WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_message_id(pool: &PgPool, message_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM incoming_emails WHERE message_id = $1")
            .bind(message_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        message_id: &str,
        from_address: &str,
        to_addresses: Option<&str>,
        subject: Option<&str>,
        raw: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO incoming_emails (message_id, from_address, to_addresses, subject, raw)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(message_id)
        .bind(from_address)
        .bind(to_addresses)
        .bind(subject)
        .bind(raw)
        .fetch_one(pool)
        .await
    }
}
