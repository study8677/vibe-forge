use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use std::path::{Path, PathBuf};

pub struct UploadParams {
    pub user_id: i64,
    pub original_filename: String,
    pub content_type: String,
    pub file_size: i64,
    pub data: Vec<u8>,
}

pub struct UploadResult {
    pub id: i64,
    pub url: String,
    pub short_url: String,
    pub original_filename: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

pub struct UploadStore {
    base_path: PathBuf,
    base_url: String,
}

impl UploadStore {
    pub fn new(base_path: PathBuf, base_url: String) -> Self {
        Self { base_path, base_url }
    }

    pub async fn store(&self, pool: &PgPool, params: UploadParams) -> Result<UploadResult> {
        use sha2::{Sha256, Digest};

        // Validate
        super::validator::UploadValidator::validate(
            &params.original_filename,
            &params.content_type,
            params.file_size,
        )?;

        // Generate SHA256 hash for deduplication
        let mut hasher = Sha256::new();
        hasher.update(&params.data);
        let sha256 = format!("{:x}", hasher.finalize());

        // Check for existing upload with same hash
        let existing: Option<(i64, String, String)> = sqlx::query_as(
            "SELECT id, url, short_url FROM uploads WHERE sha1 = $1"
        )
        .bind(&sha256)
        .fetch_optional(pool)
        .await?;

        if let Some((id, url, short_url)) = existing {
            return Ok(UploadResult {
                id,
                url,
                short_url,
                original_filename: params.original_filename,
                file_size: params.file_size,
                width: None,
                height: None,
            });
        }

        // Determine storage path
        let extension = Path::new(&params.original_filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");

        let short_path = format!("original/{}_{}.{}", &sha256[..8], params.user_id, extension);
        let full_path = self.base_path.join(&short_path);

        // Ensure directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::Internal(format!("failed to create upload dir: {}", e)))?;
        }

        // Write file
        tokio::fs::write(&full_path, &params.data)
            .await
            .map_err(|e| Error::Internal(format!("failed to write upload: {}", e)))?;

        let url = format!("{}/{}", self.base_url, short_path);
        let short_url = format!("upload://{}", &sha256[..10]);

        // Get image dimensions if applicable
        let (width, height) = if params.content_type.starts_with("image/") {
            super::optimizer::ImageOptimizer::get_dimensions(&params.data)
        } else {
            (None, None)
        };

        // Store metadata in database
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO uploads (user_id, original_filename, filesize, url, short_url, sha1, extension, width, height, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW()) RETURNING id"
        )
        .bind(params.user_id)
        .bind(&params.original_filename)
        .bind(params.file_size)
        .bind(&url)
        .bind(&short_url)
        .bind(&sha256)
        .bind(extension)
        .bind(width)
        .bind(height)
        .fetch_one(pool)
        .await?;

        Ok(UploadResult {
            id: row.0,
            url,
            short_url,
            original_filename: params.original_filename,
            file_size: params.file_size,
            width,
            height,
        })
    }

    pub async fn delete(&self, pool: &PgPool, upload_id: i64) -> Result<()> {
        let upload: Option<(String,)> = sqlx::query_as(
            "SELECT url FROM uploads WHERE id = $1"
        )
        .bind(upload_id)
        .fetch_optional(pool)
        .await?;

        if let Some((url,)) = upload {
            // Remove file from disk
            let relative_path = url.strip_prefix(&self.base_url).unwrap_or(&url);
            let full_path = self.base_path.join(relative_path.trim_start_matches('/'));
            let _ = tokio::fs::remove_file(&full_path).await;
        }

        sqlx::query("DELETE FROM uploads WHERE id = $1")
            .bind(upload_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
