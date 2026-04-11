use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailToken {
    pub id: i64,
    pub user_id: i64,
    pub email: String,
    pub token: String,
    pub confirmed: bool,
    pub expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EmailToken {
    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM email_tokens WHERE token = $1 AND expired = FALSE AND confirmed = FALSE")
            .bind(token)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64, email: &str, token: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO email_tokens (user_id, email, token) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(email)
        .bind(token)
        .fetch_one(pool)
        .await
    }

    pub async fn confirm(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE email_tokens SET confirmed = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn expire_all_for_user(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE email_tokens SET expired = TRUE, updated_at = NOW() WHERE user_id = $1 AND expired = FALSE")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
