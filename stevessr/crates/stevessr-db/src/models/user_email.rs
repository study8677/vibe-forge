use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserEmail {
    pub id: i64,
    pub user_id: i64,
    pub email: String,
    pub primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserEmail {
    pub async fn find_primary(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(r#"SELECT * FROM user_emails WHERE user_id = $1 AND "primary" = TRUE"#)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_emails WHERE LOWER(email) = LOWER($1)")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64, email: &str, primary: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_emails (user_id, email, "primary") VALUES ($1, $2, $3) RETURNING *"#,
        )
        .bind(user_id)
        .bind(email)
        .bind(primary)
        .fetch_one(pool)
        .await
    }
}
