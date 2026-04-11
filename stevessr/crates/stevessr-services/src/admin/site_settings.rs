use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use std::collections::HashMap;

pub struct SiteSettingsManager;

impl SiteSettingsManager {
    /// Get a single site setting by name.
    pub async fn get(pool: &PgPool, name: &str) -> Result<Option<String>> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT value FROM site_settings WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|(v,)| v))
    }

    /// Get a site setting with a default value.
    pub async fn get_or_default(pool: &PgPool, name: &str, default: &str) -> Result<String> {
        Ok(Self::get(pool, name).await?.unwrap_or_else(|| default.to_string()))
    }

    /// Get a boolean site setting.
    pub async fn get_bool(pool: &PgPool, name: &str, default: bool) -> Result<bool> {
        let value = Self::get(pool, name).await?;
        Ok(value.map(|v| v == "true" || v == "1" || v == "t").unwrap_or(default))
    }

    /// Get an integer site setting.
    pub async fn get_int(pool: &PgPool, name: &str, default: i64) -> Result<i64> {
        let value = Self::get(pool, name).await?;
        Ok(value.and_then(|v| v.parse().ok()).unwrap_or(default))
    }

    /// Set a site setting value.
    pub async fn set(pool: &PgPool, name: &str, value: &str, updated_by_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO site_settings (name, value, data_type, updated_at)
             VALUES ($1, $2, 1, NOW())
             ON CONFLICT (name) DO UPDATE SET value = $2, updated_at = NOW()"
        )
        .bind(name)
        .bind(value)
        .execute(pool)
        .await?;

        // Log the setting change
        sqlx::query(
            "INSERT INTO user_histories (acting_user_id, action, subject, previous_value, new_value, created_at, updated_at)
             VALUES ($1, 'change_site_setting', $2, (SELECT value FROM site_settings WHERE name = $2), $3, NOW(), NOW())"
        )
        .bind(updated_by_id)
        .bind(name)
        .bind(value)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get all site settings as a map.
    pub async fn get_all(pool: &PgPool) -> Result<HashMap<String, String>> {
        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT name, value FROM site_settings ORDER BY name"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().collect())
    }

    /// Delete a site setting (revert to default).
    pub async fn delete(pool: &PgPool, name: &str) -> Result<()> {
        sqlx::query("DELETE FROM site_settings WHERE name = $1")
            .bind(name)
            .execute(pool)
            .await?;
        Ok(())
    }
}

// Facade functions for the API layer
pub async fn list_all(pool: &PgPool) -> Result<Vec<serde_json::Value>> { todo!() }
pub async fn update_setting(pool: &PgPool, _id: &str, _value: &str) -> Result<()> { todo!() }
