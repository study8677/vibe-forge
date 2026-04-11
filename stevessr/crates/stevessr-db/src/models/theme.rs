use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Theme {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub compiler_version: Option<i32>,
    pub user_selectable: bool,
    pub hidden: bool,
    pub color_scheme_id: Option<i64>,
    pub remote_theme_id: Option<i64>,
    pub component: bool,
    pub enabled: bool,
    pub auto_update: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Theme {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM themes WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM themes ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_user_selectable(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM themes WHERE user_selectable = TRUE AND enabled = TRUE ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, name: &str, user_id: i64, component: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO themes (name, user_id, component) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(user_id)
        .bind(component)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM themes WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
