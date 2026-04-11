use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ColorScheme {
    pub id: i64,
    pub name: String,
    pub version: i32,
    pub via_wizard: bool,
    pub base_scheme_id: Option<String>,
    pub theme_id: Option<i64>,
    pub user_selectable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ColorScheme {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM color_schemes WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM color_schemes ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, name: &str, base_scheme_id: Option<&str>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO color_schemes (name, base_scheme_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(base_scheme_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM color_schemes WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
