use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupHistory {
    pub id: i64,
    pub group_id: i64,
    pub acting_user_id: i64,
    pub target_user_id: Option<i64>,
    pub action: i32,
    pub subject: Option<String>,
    pub prev_value: Option<String>,
    pub new_value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GroupHistory {
    pub async fn find_by_group(pool: &PgPool, group_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM group_histories WHERE group_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(group_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        group_id: i64,
        acting_user_id: i64,
        target_user_id: Option<i64>,
        action: i32,
        subject: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO group_histories (group_id, acting_user_id, target_user_id, action, subject)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(group_id)
        .bind(acting_user_id)
        .bind(target_user_id)
        .bind(action)
        .bind(subject)
        .fetch_one(pool)
        .await
    }
}
