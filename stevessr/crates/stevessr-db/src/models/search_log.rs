use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SearchLog {
    pub id: i64,
    pub term: String,
    pub user_id: Option<i64>,
    pub ip_address: Option<String>,
    pub search_result_id: Option<i64>,
    pub search_type: i32,
    pub search_result_type: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl SearchLog {
    pub async fn create(
        pool: &PgPool,
        term: &str,
        user_id: Option<i64>,
        ip_address: Option<&str>,
        search_type: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO search_logs (term, user_id, ip_address, search_type) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(term)
        .bind(user_id)
        .bind(ip_address)
        .bind(search_type)
        .fetch_one(pool)
        .await
    }

    pub async fn popular_terms(pool: &PgPool, limit: i64) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as::<_, (String, i64)>(
            "SELECT term, COUNT(*) as cnt FROM search_logs GROUP BY term ORDER BY cnt DESC LIMIT $1",
        )
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn recent(pool: &PgPool, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM search_logs ORDER BY created_at DESC LIMIT $1")
            .bind(limit)
            .fetch_all(pool)
            .await
    }
}
