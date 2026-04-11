use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserHistory {
    pub id: i64,
    pub action: i32,
    pub acting_user_id: Option<i64>,
    pub target_user_id: Option<i64>,
    pub details: Option<String>,
    pub context: Option<String>,
    pub ip_address: Option<String>,
    pub email: Option<String>,
    pub subject: Option<String>,
    pub previous_value: Option<String>,
    pub new_value: Option<String>,
    pub topic_id: Option<i64>,
    pub post_id: Option<i64>,
    pub category_id: Option<i64>,
    pub admin_only: bool,
    pub custom_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserHistory {
    pub async fn find_by_target_user(pool: &PgPool, target_user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_histories WHERE target_user_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(target_user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_action(pool: &PgPool, action: i32, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_histories WHERE action = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(action)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        action: i32,
        acting_user_id: Option<i64>,
        target_user_id: Option<i64>,
        details: Option<&str>,
        context: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_histories (action, acting_user_id, target_user_id, details, context)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(action)
        .bind(acting_user_id)
        .bind(target_user_id)
        .bind(details)
        .bind(context)
        .fetch_one(pool)
        .await
    }
}
