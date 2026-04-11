use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PluginStoreRow {
    pub id: i64,
    pub plugin_name: String,
    pub key: String,
    pub type_name: String,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PluginStoreRow {
    pub async fn find_by_plugin_and_key(pool: &PgPool, plugin_name: &str, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM plugin_store_rows WHERE plugin_name = $1 AND key = $2")
            .bind(plugin_name)
            .bind(key)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_plugin(pool: &PgPool, plugin_name: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM plugin_store_rows WHERE plugin_name = $1")
            .bind(plugin_name)
            .fetch_all(pool)
            .await
    }

    pub async fn upsert(
        pool: &PgPool,
        plugin_name: &str,
        key: &str,
        type_name: &str,
        value: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO plugin_store_rows (plugin_name, key, type_name, value)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (plugin_name, key) DO UPDATE SET value = $4, type_name = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(plugin_name)
        .bind(key)
        .bind(type_name)
        .bind(value)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, plugin_name: &str, key: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM plugin_store_rows WHERE plugin_name = $1 AND key = $2")
            .bind(plugin_name)
            .bind(key)
            .execute(pool)
            .await?;
        Ok(())
    }
}
