use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAvatar {
    pub id: i64,
    pub user_id: i64,
    pub custom_upload_id: Option<i64>,
    pub gravatar_upload_id: Option<i64>,
    pub last_gravatar_download_attempt: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAvatar {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_avatars WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_avatars (user_id) VALUES ($1) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_custom(pool: &PgPool, user_id: i64, upload_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_avatars SET custom_upload_id = $2, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .bind(upload_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
