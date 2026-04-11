use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSecondFactor {
    pub id: i64,
    pub user_id: i64,
    pub method: i16,
    pub data: String,
    pub enabled: bool,
    pub last_used: Option<DateTime<Utc>>,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSecondFactor {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_second_factors WHERE user_id = $1 AND enabled = TRUE")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_second_factors WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        method: i16,
        data: &str,
        name: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_second_factors (user_id, method, data, enabled, name) VALUES ($1, $2, $3, TRUE, $4) RETURNING *",
        )
        .bind(user_id)
        .bind(method)
        .bind(data)
        .bind(name)
        .fetch_one(pool)
        .await
    }

    pub async fn disable(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_second_factors SET enabled = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
