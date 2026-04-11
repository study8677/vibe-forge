use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScreenedEmail {
    pub id: i64,
    pub email: String,
    pub action_type: i32,
    pub match_count: i32,
    pub last_match_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ScreenedEmail {
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM screened_emails WHERE LOWER(email) = LOWER($1)")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM screened_emails ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, email: &str, action_type: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO screened_emails (email, action_type) VALUES ($1, $2) RETURNING *",
        )
        .bind(email)
        .bind(action_type)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_match(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE screened_emails SET match_count = match_count + 1, last_match_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM screened_emails WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
