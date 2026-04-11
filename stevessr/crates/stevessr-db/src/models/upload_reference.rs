use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UploadReference {
    pub id: i64,
    pub upload_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UploadReference {
    pub async fn find_by_upload(pool: &PgPool, upload_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM upload_references WHERE upload_id = $1")
            .bind(upload_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_target(pool: &PgPool, target_type: &str, target_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM upload_references WHERE target_type = $1 AND target_id = $2")
            .bind(target_type)
            .bind(target_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, upload_id: i64, target_type: &str, target_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO upload_references (upload_id, target_type, target_id) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(upload_id)
        .bind(target_type)
        .bind(target_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM upload_references WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
