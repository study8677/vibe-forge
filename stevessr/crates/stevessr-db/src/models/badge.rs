use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Badge {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub badge_type_id: i64,
    pub grant_count: i32,
    pub allow_title: bool,
    pub multiple_grant: bool,
    pub icon: Option<String>,
    pub image_upload_id: Option<i64>,
    pub listable: bool,
    pub target_posts: bool,
    pub enabled: bool,
    pub auto_revoke: bool,
    pub badge_grouping_id: i64,
    pub trigger: Option<i32>,
    pub show_posts: bool,
    pub system: bool,
    pub long_description: Option<String>,
    pub query: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Badge {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM badges WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM badges WHERE enabled = TRUE ORDER BY badge_type_id ASC, name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM badges WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
        badge_type_id: i64,
        badge_grouping_id: i64,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO badges (name, description, badge_type_id, badge_grouping_id)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(name)
        .bind(description)
        .bind(badge_type_id)
        .bind(badge_grouping_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM badges WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
