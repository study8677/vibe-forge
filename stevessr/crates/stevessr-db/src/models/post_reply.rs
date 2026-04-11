use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostReply {
    pub id: i64,
    pub post_id: i64,
    pub reply_post_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostReply {
    pub async fn find_by_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_replies WHERE post_id = $1")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_replies_to(pool: &PgPool, reply_post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_replies WHERE reply_post_id = $1")
            .bind(reply_post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, post_id: i64, reply_post_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO post_replies (post_id, reply_post_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(post_id)
        .bind(reply_post_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, post_id: i64, reply_post_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM post_replies WHERE post_id = $1 AND reply_post_id = $2")
            .bind(post_id)
            .bind(reply_post_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
