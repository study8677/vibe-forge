use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserStat {
    pub id: i64,
    pub user_id: i64,
    pub topics_entered: i32,
    pub time_read: i64,
    pub days_visited: i32,
    pub posts_read_count: i32,
    pub likes_given: i32,
    pub likes_received: i32,
    pub new_since: DateTime<Utc>,
    pub read_faq: Option<DateTime<Utc>>,
    pub first_post_created_at: Option<DateTime<Utc>>,
    pub post_count: i32,
    pub topic_count: i32,
    pub bounce_score: f64,
    pub reset_bounce_score_after: Option<DateTime<Utc>>,
    pub flags_agreed: i32,
    pub flags_disagreed: i32,
    pub flags_ignored: i32,
    pub first_unread_at: DateTime<Utc>,
    pub distinct_badge_count: i32,
    pub first_unread_pm_at: DateTime<Utc>,
    pub digest_attempted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserStat {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_stats WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_stats (user_id, new_since, first_unread_at, first_unread_pm_at) VALUES ($1, NOW(), NOW(), NOW()) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_post_count(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_stats SET post_count = post_count + 1, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_topic_count(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_stats SET topic_count = topic_count + 1, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_likes_given(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_stats SET likes_given = likes_given + 1, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_likes_received(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_stats SET likes_received = likes_received + 1, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
