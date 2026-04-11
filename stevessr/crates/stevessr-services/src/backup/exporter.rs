use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use std::path::PathBuf;

pub struct BackupExporter {
    backup_dir: PathBuf,
}

impl BackupExporter {
    pub fn new(backup_dir: PathBuf) -> Self {
        Self { backup_dir }
    }

    /// Create a full database backup as a compressed SQL dump.
    pub async fn export(&self, pool: &PgPool, initiated_by_id: i64) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let filename = format!("backup-{}.sql.gz", timestamp);
        let filepath = self.backup_dir.join(&filename);

        // Ensure backup directory exists
        tokio::fs::create_dir_all(&self.backup_dir)
            .await
            .map_err(|e| Error::Internal(format!("failed to create backup dir: {}", e)))?;

        // Use pg_dump for the actual backup
        let db_url = std::env::var("DATABASE_URL")
            .map_err(|_| Error::Internal("DATABASE_URL not set".into()))?;

        let output = tokio::process::Command::new("pg_dump")
            .arg(&db_url)
            .arg("--no-owner")
            .arg("--no-privileges")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| Error::Internal(format!("failed to start pg_dump: {}", e)))?
            .wait_with_output()
            .await
            .map_err(|e| Error::Internal(format!("pg_dump failed: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Internal("pg_dump exited with error".into()));
        }

        // Compress the output
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let file = std::fs::File::create(&filepath)
            .map_err(|e| Error::Internal(format!("failed to create backup file: {}", e)))?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&output.stdout)
            .map_err(|e| Error::Internal(format!("failed to write backup: {}", e)))?;
        encoder.finish()
            .map_err(|e| Error::Internal(format!("failed to finalize backup: {}", e)))?;

        // Log the backup
        sqlx::query(
            "INSERT INTO backups (filename, size, created_at)
             VALUES ($1, $2, NOW())"
        )
        .bind(&filename)
        .bind(output.stdout.len() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            filename = %filename,
            initiated_by = initiated_by_id,
            "backup created successfully"
        );

        Ok(filename)
    }

    /// List available backups.
    pub async fn list(&self, pool: &PgPool) -> Result<Vec<(i64, String, i64, chrono::DateTime<chrono::Utc>)>> {
        let rows = sqlx::query_as(
            "SELECT id, filename, size, created_at FROM backups ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    /// Delete an old backup file and its record.
    pub async fn delete(&self, pool: &PgPool, backup_id: i64) -> Result<()> {
        let filename: Option<(String,)> = sqlx::query_as(
            "SELECT filename FROM backups WHERE id = $1"
        )
        .bind(backup_id)
        .fetch_optional(pool)
        .await?;

        if let Some((name,)) = filename {
            let filepath = self.backup_dir.join(&name);
            let _ = tokio::fs::remove_file(&filepath).await;
        }

        sqlx::query("DELETE FROM backups WHERE id = $1")
            .bind(backup_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
