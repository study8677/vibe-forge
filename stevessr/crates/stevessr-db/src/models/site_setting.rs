use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SiteSetting {
    pub id: i64,
    pub name: String,
    pub data_type: i32,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SiteSetting {
    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM site_settings WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM site_settings ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, name: &str, data_type: i32, value: Option<&str>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO site_settings (name, data_type, value)
               VALUES ($1, $2, $3)
               ON CONFLICT (name) DO UPDATE SET value = $3, data_type = $2, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(name)
        .bind(data_type)
        .bind(value)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM site_settings WHERE name = $1")
            .bind(name)
            .execute(pool)
            .await?;
        Ok(())
    }
}
