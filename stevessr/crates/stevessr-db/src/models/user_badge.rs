use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBadge {
    pub id: i64,
    pub badge_id: i64,
    pub user_id: i64,
    pub granted_at: DateTime<Utc>,
    pub granted_by_id: i64,
    pub post_id: Option<i64>,
    pub notification_id: Option<i64>,
    pub seq: i32,
    pub featured_rank: Option<i32>,
    pub is_favorite: Option<bool>,
    pub created_at: DateTime<Utc>,
}

impl UserBadge {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_badges WHERE user_id = $1 ORDER BY granted_at DESC")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_badge_and_user(pool: &PgPool, badge_id: i64, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_badges WHERE badge_id = $1 AND user_id = $2")
            .bind(badge_id)
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        badge_id: i64,
        user_id: i64,
        granted_by_id: i64,
        post_id: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_badges (badge_id, user_id, granted_at, granted_by_id, post_id) VALUES ($1, $2, NOW(), $3, $4) RETURNING *",
        )
        .bind(badge_id)
        .bind(user_id)
        .bind(granted_by_id)
        .bind(post_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_badges WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
