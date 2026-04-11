use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Upload {
    pub id: i64,
    pub user_id: i64,
    pub original_filename: String,
    pub filesize: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub url: String,
    pub sha1: Option<String>,
    pub origin: Option<String>,
    pub retain_hours: Option<i32>,
    pub extension: Option<String>,
    pub thumbnail_width: Option<i32>,
    pub thumbnail_height: Option<i32>,
    pub etag: Option<String>,
    pub secure: bool,
    pub access_control_post_id: Option<i64>,
    pub original_sha1: Option<String>,
    pub animated: Option<bool>,
    pub verification_status: i16,
    pub dominant_color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Upload {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM uploads WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_sha1(pool: &PgPool, sha1: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM uploads WHERE sha1 = $1")
            .bind(sha1)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM uploads WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        original_filename: &str,
        filesize: i64,
        url: &str,
        sha1: Option<&str>,
        extension: Option<&str>,
        width: Option<i32>,
        height: Option<i32>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO uploads (user_id, original_filename, filesize, url, sha1, extension, width, height)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#,
        )
        .bind(user_id)
        .bind(original_filename)
        .bind(filesize)
        .bind(url)
        .bind(sha1)
        .bind(extension)
        .bind(width)
        .bind(height)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM uploads WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
