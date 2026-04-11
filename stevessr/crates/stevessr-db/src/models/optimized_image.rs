use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OptimizedImage {
    pub id: i64,
    pub sha1: String,
    pub extension: Option<String>,
    pub width: i32,
    pub height: i32,
    pub upload_id: i64,
    pub url: String,
    pub filesize: Option<i64>,
    pub etag: Option<String>,
    pub version: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OptimizedImage {
    pub async fn find_by_upload(pool: &PgPool, upload_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM optimized_images WHERE upload_id = $1")
            .bind(upload_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_upload_and_size(pool: &PgPool, upload_id: i64, width: i32, height: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM optimized_images WHERE upload_id = $1 AND width = $2 AND height = $3")
            .bind(upload_id)
            .bind(width)
            .bind(height)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        upload_id: i64,
        sha1: &str,
        width: i32,
        height: i32,
        url: &str,
        filesize: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO optimized_images (upload_id, sha1, width, height, url, filesize)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
        )
        .bind(upload_id)
        .bind(sha1)
        .bind(width)
        .bind(height)
        .bind(url)
        .bind(filesize)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM optimized_images WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
