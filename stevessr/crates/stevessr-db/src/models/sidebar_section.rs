use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SidebarSection {
    pub id: i64,
    pub user_id: Option<i64>,
    pub title: String,
    pub public: bool,
    pub section_type: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SidebarSection {
    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM sidebar_sections WHERE user_id = $1 ORDER BY title ASC")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_public(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM sidebar_sections WHERE public = TRUE ORDER BY title ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: Option<i64>, title: &str, public: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO sidebar_sections (user_id, title, public) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(title)
        .bind(public)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sidebar_sections WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
