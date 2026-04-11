use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SingleSignOnRecord {
    pub id: i64,
    pub user_id: i64,
    pub external_id: String,
    pub last_payload: Option<String>,
    pub external_username: Option<String>,
    pub external_email: Option<String>,
    pub external_name: Option<String>,
    pub external_avatar_url: Option<String>,
    pub external_profile_background_url: Option<String>,
    pub external_card_background_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SingleSignOnRecord {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM single_sign_on_records WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_external_id(pool: &PgPool, external_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM single_sign_on_records WHERE external_id = $1")
            .bind(external_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        external_id: &str,
        last_payload: Option<&str>,
        external_email: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO single_sign_on_records (user_id, external_id, last_payload, external_email)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(user_id)
        .bind(external_id)
        .bind(last_payload)
        .bind(external_email)
        .fetch_one(pool)
        .await
    }
}
