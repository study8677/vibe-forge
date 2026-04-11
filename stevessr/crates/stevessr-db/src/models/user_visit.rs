use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserVisit {
    pub id: i64,
    pub user_id: i64,
    pub visited_at: NaiveDate,
    pub posts_read: i32,
    pub mobile: bool,
    pub time_read: i32,
}

impl UserVisit {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_visits WHERE user_id = $1 ORDER BY visited_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_date(pool: &PgPool, user_id: i64, date: NaiveDate) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_visits WHERE user_id = $1 AND visited_at = $2")
            .bind(user_id)
            .bind(date)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, user_id: i64, date: NaiveDate, mobile: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_visits (user_id, visited_at, mobile)
               VALUES ($1, $2, $3)
               ON CONFLICT (user_id, visited_at) DO UPDATE SET posts_read = user_visits.posts_read
               RETURNING *"#,
        )
        .bind(user_id)
        .bind(date)
        .bind(mobile)
        .fetch_one(pool)
        .await
    }
}
