use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WatchedWord {
    pub id: i64,
    pub word: String,
    pub action: i32,
    pub replacement: Option<String>,
    pub case_sensitive: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WatchedWord {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM watched_words ORDER BY action ASC, word ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_action(pool: &PgPool, action: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM watched_words WHERE action = $1 ORDER BY word ASC")
            .bind(action)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, word: &str, action: i32, replacement: Option<&str>, case_sensitive: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO watched_words (word, action, replacement, case_sensitive) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(word)
        .bind(action)
        .bind(replacement)
        .bind(case_sensitive)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM watched_words WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
