use sqlx::PgPool;
use stevessr_plugin_api::error::PluginError;
use std::path::Path;

pub struct PluginMigrationRunner;

impl PluginMigrationRunner {
    pub async fn run(pool: &PgPool, plugin_name: &str, migrations_dir: &Path) -> Result<(), PluginError> {
        if !migrations_dir.exists() { return Ok(()); }

        let mut entries: Vec<_> = std::fs::read_dir(migrations_dir)
            .map_err(|e| PluginError::MigrationError(e.to_string()))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let version = entry.file_name().to_string_lossy().to_string();

            let already_applied: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM plugin_migrations WHERE plugin_name = $1 AND version = $2)"
            )
            .bind(plugin_name)
            .bind(&version)
            .fetch_one(pool)
            .await
            .map_err(|e| PluginError::MigrationError(e.to_string()))?;

            if !already_applied {
                let sql = std::fs::read_to_string(entry.path())
                    .map_err(|e| PluginError::MigrationError(e.to_string()))?;

                sqlx::query(&sql).execute(pool).await
                    .map_err(|e| PluginError::MigrationError(format!("{}: {}", version, e)))?;

                sqlx::query("INSERT INTO plugin_migrations (plugin_name, version) VALUES ($1, $2)")
                    .bind(plugin_name)
                    .bind(&version)
                    .execute(pool)
                    .await
                    .map_err(|e| PluginError::MigrationError(e.to_string()))?;

                tracing::info!(plugin = plugin_name, migration = %version, "applied plugin migration");
            }
        }

        Ok(())
    }
}
