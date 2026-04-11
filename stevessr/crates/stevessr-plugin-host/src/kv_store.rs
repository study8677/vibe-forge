use async_trait::async_trait;
use sqlx::PgPool;
use stevessr_plugin_api::store::{PluginStore, StoreValue};
use stevessr_plugin_api::error::PluginError;

pub struct PgPluginStore {
    pool: PgPool,
    plugin_name: String,
}

impl PgPluginStore {
    pub fn new(pool: PgPool, plugin_name: String) -> Self {
        Self { pool, plugin_name }
    }
}

#[async_trait]
impl PluginStore for PgPluginStore {
    async fn get(&self, key: &str) -> Result<Option<StoreValue>, PluginError> {
        let row: Option<(String, String)> = sqlx::query_as(
            "SELECT type_name, COALESCE(value, '') FROM plugin_store_rows WHERE plugin_name = $1 AND key = $2"
        )
        .bind(&self.plugin_name)
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PluginError::StoreError(e.to_string()))?;

        Ok(row.map(|(type_name, value)| match type_name.as_str() {
            "integer" => StoreValue::Integer(value.parse().unwrap_or(0)),
            "json" => StoreValue::Json(serde_json::from_str(&value).unwrap_or(serde_json::Value::Null)),
            _ => StoreValue::String(value),
        }))
    }

    async fn set(&self, key: &str, value: StoreValue) -> Result<(), PluginError> {
        let (type_name, val_str) = match &value {
            StoreValue::String(s) => ("string", s.clone()),
            StoreValue::Json(v) => ("json", serde_json::to_string(v).unwrap_or_default()),
            StoreValue::Integer(i) => ("integer", i.to_string()),
        };

        sqlx::query(
            "INSERT INTO plugin_store_rows (plugin_name, key, type_name, value) VALUES ($1, $2, $3, $4)
             ON CONFLICT (plugin_name, key) DO UPDATE SET type_name = $3, value = $4"
        )
        .bind(&self.plugin_name)
        .bind(key)
        .bind(type_name)
        .bind(&val_str)
        .execute(&self.pool)
        .await
        .map_err(|e| PluginError::StoreError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), PluginError> {
        sqlx::query("DELETE FROM plugin_store_rows WHERE plugin_name = $1 AND key = $2")
            .bind(&self.plugin_name)
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| PluginError::StoreError(e.to_string()))?;
        Ok(())
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, PluginError> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT key FROM plugin_store_rows WHERE plugin_name = $1 AND key LIKE $2"
        )
        .bind(&self.plugin_name)
        .bind(format!("{}%", prefix))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| PluginError::StoreError(e.to_string()))?;

        Ok(rows.into_iter().map(|(k,)| k).collect())
    }
}
