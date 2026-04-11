use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAction {
    pub id: i64,
    pub action_type: i32,
    pub user_id: i64,
    pub target_topic_id: Option<i64>,
    pub target_post_id: Option<i64>,
    pub target_user_id: Option<i64>,
    pub acting_user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAction {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_actions WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_type(pool: &PgPool, user_id: i64, action_type: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_actions WHERE user_id = $1 AND action_type = $2 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .bind(action_type)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        action_type: i32,
        user_id: i64,
        acting_user_id: i64,
        target_topic_id: Option<i64>,
        target_post_id: Option<i64>,
        target_user_id: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_actions (action_type, user_id, acting_user_id, target_topic_id, target_post_id, target_user_id)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
        )
        .bind(action_type)
        .bind(user_id)
        .bind(acting_user_id)
        .bind(target_topic_id)
        .bind(target_post_id)
        .bind(target_user_id)
        .fetch_one(pool)
        .await
    }
}
