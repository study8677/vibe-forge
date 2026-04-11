use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserProfile {
    pub id: i64,
    pub user_id: i64,
    pub location: Option<String>,
    pub website: Option<String>,
    pub bio_raw: Option<String>,
    pub bio_cooked: Option<String>,
    pub bio_cooked_version: Option<i32>,
    pub dismissed_banner_key: Option<i32>,
    pub badge_granted_title: Option<bool>,
    pub card_background_upload_id: Option<i64>,
    pub profile_background_upload_id: Option<i64>,
    pub featured_topic_id: Option<i64>,
    pub views: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_profiles (user_id) VALUES ($1) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_bio(pool: &PgPool, user_id: i64, bio_raw: &str, bio_cooked: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_profiles SET bio_raw = $2, bio_cooked = $3, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .bind(bio_raw)
            .bind(bio_cooked)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_location(pool: &PgPool, user_id: i64, location: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_profiles SET location = $2, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .bind(location)
            .execute(pool)
            .await?;
        Ok(())
    }
}
