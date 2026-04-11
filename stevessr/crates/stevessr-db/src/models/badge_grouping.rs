use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BadgeGrouping {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BadgeGrouping {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM badge_groupings ORDER BY position ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM badge_groupings WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, name: &str, position: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO badge_groupings (name, position) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(position)
        .fetch_one(pool)
        .await
    }
}
