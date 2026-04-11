use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub automatic: bool,
    pub user_count: i32,
    pub automatic_membership_email_domains: Option<String>,
    pub primary_group: bool,
    pub title: Option<String>,
    pub grant_trust_level: Option<i16>,
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    pub flair_url: Option<String>,
    pub flair_bg_color: Option<String>,
    pub flair_color: Option<String>,
    pub bio_raw: Option<String>,
    pub bio_cooked: Option<String>,
    pub allow_membership_requests: bool,
    pub full_name: Option<String>,
    pub default_notification_level: i16,
    pub visibility_level: i32,
    pub public_exit: bool,
    pub public_admission: bool,
    pub membership_request_template: Option<String>,
    pub messageable_level: i32,
    pub mentionable_level: i32,
    pub members_visibility_level: i32,
    pub publish_read_state: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Group {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM groups WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM groups WHERE LOWER(name) = LOWER($1)")
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM groups ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_visible(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM groups WHERE visibility_level = 0 ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        full_name: Option<&str>,
        visibility_level: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO groups (name, full_name, visibility_level) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(full_name)
        .bind(visibility_level)
        .fetch_one(pool)
        .await
    }

    pub async fn update_user_count(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE groups SET user_count = (SELECT COUNT(*) FROM group_users WHERE group_id = $1), updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM groups WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
