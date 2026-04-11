use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::PluginError;

/// Per-plugin scoped key-value store backed by PostgreSQL.
#[async_trait]
pub trait PluginStore: Send + Sync {
    /// Get a value by key.
    async fn get(&self, key: &str) -> Result<Option<StoreValue>, PluginError>;

    /// Set a value by key.
    async fn set(&self, key: &str, value: StoreValue) -> Result<(), PluginError>;

    /// Delete a key.
    async fn delete(&self, key: &str) -> Result<(), PluginError>;

    /// List all keys with the given prefix.
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, PluginError>;

    /// Check if a key exists.
    async fn exists(&self, key: &str) -> Result<bool, PluginError> {
        Ok(self.get(key).await?.is_some())
    }
}

/// Store value types matching the plugin_store_rows table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoreValue {
    String(String),
    Json(serde_json::Value),
    Integer(i64),
}

impl StoreValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&serde_json::Value> {
        match self {
            Self::Json(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Json(_) => "json",
            Self::Integer(_) => "integer",
        }
    }
}
