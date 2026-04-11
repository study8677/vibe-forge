use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserStatus {
    pub id: i64,
    pub user_id: i64,
    pub emoji: String,
    pub description: String,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserStatus {
    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_statuses WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(
        pool: &PgPool,
        user_id: i64,
        emoji: &str,
        description: &str,
        ends_at: Option<DateTime<Utc>>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_statuses (user_id, emoji, description, ends_at)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (user_id) DO UPDATE SET emoji = $2, description = $3, ends_at = $4, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(user_id)
        .bind(emoji)
        .bind(description)
        .bind(ends_at)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_statuses WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM user_statuses WHERE ends_at IS NOT NULL AND ends_at < NOW()")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
