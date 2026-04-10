use crate::db::Database;
use crate::exif_parser;
use image::DynamicImage;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};

/// Thumbnail sizes: small (grid view) and medium (preview).
const THUMB_SM: u32 = 240;
const THUMB_MD: u32 = 720;

/// Background thumbnail worker loop.
/// Polls for pending photos, generates thumbnails using rayon parallelism,
/// and updates the database.
pub async fn worker_loop(db: Arc<Database>, data_dir: PathBuf, _workers: usize) {
    info!("Thumbnail worker started");

    loop {
        let db2 = db.clone();
        let dd2 = data_dir.clone();

        let processed = tokio::task::spawn_blocking(move || {
            let items = match db2.get_pending_thumbs(32) {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("Failed to query pending thumbs: {e}");
                    return 0;
                }
            };
            if items.is_empty() {
                return 0;
            }

            let count = items.len();
            use rayon::prelude::*;
            items.par_iter().for_each(|item| {
                match generate_thumbnail(&item.path, item.id, &dd2) {
                    Ok((w, h, taken_at, orientation)) => {
                        let _ = db2.update_thumb_done(item.id, w, h, taken_at, orientation);
                    }
                    Err(e) => {
                        debug!("Thumbnail error for {}: {e}", item.path);
                        let _ = db2.update_thumb_error(item.id);
                    }
                }
            });
            count
        })
        .await
        .unwrap_or(0);

        if processed == 0 {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}

/// Generate small + medium thumbnails for one image.
/// Also extracts EXIF metadata.  Returns `(width, height, taken_at, orientation)`.
fn generate_thumbnail(
    src: &str,
    id: i64,
    data_dir: &Path,
) -> Result<(i32, i32, Option<String>, i32), Box<dyn std::error::Error + Send + Sync>> {
    let src_path = Path::new(src);

    // Extract EXIF before opening image (lighter I/O path)
    let (taken_at, orientation) = exif_parser::extract_exif(src_path);

    let img = image::open(src_path)?;
    let (w, h) = (img.width(), img.height());

    // Apply EXIF orientation
    let img = apply_orientation(img, orientation);

    // Small thumbnail
    let sm = img.thumbnail(THUMB_SM, THUMB_SM);
    let sm_path = thumb_path(data_dir, id, "sm");
    std::fs::create_dir_all(sm_path.parent().unwrap())?;
    sm.save_with_format(&sm_path, image::ImageFormat::Jpeg)?;

    // Medium thumbnail
    let md = img.thumbnail(THUMB_MD, THUMB_MD);
    let md_path = thumb_path(data_dir, id, "md");
    md.save_with_format(&md_path, image::ImageFormat::Jpeg)?;

    Ok((w as i32, h as i32, taken_at, orientation))
}

/// Compute the filesystem path for a thumbnail.
/// Uses two-level directory bucketing based on id for uniform distribution
/// across 65 536 directories (~153 files each at 10 M scale).
pub fn thumb_path(data_dir: &Path, id: i64, size: &str) -> PathBuf {
    let id_u = id as u64;
    data_dir
        .join("thumbs")
        .join(format!("{:02x}", (id_u / 256) % 256))
        .join(format!("{:02x}", id_u % 256))
        .join(format!("{id}_{size}.jpg"))
}

/// Rotate / flip image according to EXIF orientation tag.
fn apply_orientation(img: DynamicImage, orientation: i32) -> DynamicImage {
    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img, // 1 = normal
    }
}
