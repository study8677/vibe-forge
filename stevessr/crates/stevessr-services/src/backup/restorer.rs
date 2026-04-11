use stevessr_core::error::{Error, Result};
use std::path::PathBuf;

pub struct BackupRestorer {
    backup_dir: PathBuf,
}

impl BackupRestorer {
    pub fn new(backup_dir: PathBuf) -> Self {
        Self { backup_dir }
    }

    /// Restore a backup from a file.
    /// WARNING: This will replace all data in the database.
    pub async fn restore(&self, filename: &str) -> Result<()> {
        let filepath = self.backup_dir.join(filename);

        if !filepath.exists() {
            return Err(Error::NotFound {
                resource: "backup",
                id: filename.to_string(),
            });
        }

        let db_url = std::env::var("DATABASE_URL")
            .map_err(|_| Error::Internal("DATABASE_URL not set".into()))?;

        // Decompress if needed
        let sql_data = if filename.ends_with(".gz") {
            use flate2::read::GzDecoder;
            use std::io::Read;

            let file = std::fs::File::open(&filepath)
                .map_err(|e| Error::Internal(format!("failed to open backup: {}", e)))?;
            let mut decoder = GzDecoder::new(file);
            let mut data = String::new();
            decoder.read_to_string(&mut data)
                .map_err(|e| Error::Internal(format!("failed to decompress backup: {}", e)))?;
            data
        } else {
            tokio::fs::read_to_string(&filepath)
                .await
                .map_err(|e| Error::Internal(format!("failed to read backup: {}", e)))?
        };

        // Execute via psql
        let mut child = tokio::process::Command::new("psql")
            .arg(&db_url)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| Error::Internal(format!("failed to start psql: {}", e)))?;

        if let Some(stdin) = child.stdin.as_mut() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(sql_data.as_bytes()).await
                .map_err(|e| Error::Internal(format!("failed to write to psql: {}", e)))?;
        }

        let output = child.wait_with_output().await
            .map_err(|e| Error::Internal(format!("psql failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Internal(format!("restore failed: {}", stderr)));
        }

        tracing::info!(filename = %filename, "backup restored successfully");

        Ok(())
    }
}
