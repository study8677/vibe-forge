use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CategoryUser {
    pub id: i64,
    pub category_id: i64,
    pub user_id: i64,
    pub notification_level: i16,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryUser {
    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM category_users WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_category_and_user(pool: &PgPool, category_id: i64, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM category_users WHERE category_id = $1 AND user_id = $2")
            .bind(category_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, category_id: i64, user_id: i64, notification_level: i16) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO category_users (category_id, user_id, notification_level)
               VALUES ($1, $2, $3)
               ON CONFLICT (category_id, user_id) DO UPDATE SET notification_level = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(category_id)
        .bind(user_id)
        .bind(notification_level)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, category_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM category_users WHERE category_id = $1 AND user_id = $2")
            .bind(category_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
