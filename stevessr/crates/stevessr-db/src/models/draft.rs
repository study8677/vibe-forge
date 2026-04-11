use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Draft {
    pub id: i64,
    pub user_id: i64,
    pub draft_key: String,
    pub data: String,
    pub sequence: i32,
    pub revisions: i32,
    pub owner: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Draft {
    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM drafts WHERE user_id = $1 ORDER BY updated_at DESC")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_key(pool: &PgPool, user_id: i64, draft_key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM drafts WHERE user_id = $1 AND draft_key = $2")
            .bind(user_id)
            .bind(draft_key)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(
        pool: &PgPool,
        user_id: i64,
        draft_key: &str,
        data: &str,
        sequence: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO drafts (user_id, draft_key, data, sequence)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (user_id, draft_key) DO UPDATE SET data = $3, sequence = $4, revisions = drafts.revisions + 1, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(user_id)
        .bind(draft_key)
        .bind(data)
        .bind(sequence)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, user_id: i64, draft_key: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM drafts WHERE user_id = $1 AND draft_key = $2")
            .bind(user_id)
            .bind(draft_key)
            .execute(pool)
            .await?;
        Ok(())
    }
}
