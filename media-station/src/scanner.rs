use crate::db::Database;
use crate::models::{NewMedia, ScanStatus};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tracing::info;
use walkdir::WalkDir;

const PHOTO_EXTS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif", "heic", "heif",
];
const VIDEO_EXTS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "3gp", "ts",
];
const BATCH_SIZE: usize = 5000;

/// Shared scan progress — fully sync-safe via atomics + Mutex.
pub struct ScanProgress {
    pub scanning: AtomicBool,
    pub files_found: AtomicU64,
    pub files_indexed: AtomicU64,
    pub current_dir: Mutex<String>,
}

impl ScanProgress {
    pub fn new() -> Self {
        Self {
            scanning: AtomicBool::new(false),
            files_found: AtomicU64::new(0),
            files_indexed: AtomicU64::new(0),
            current_dir: Mutex::new(String::new()),
        }
    }

    pub fn to_status(&self) -> ScanStatus {
        ScanStatus {
            scanning: self.scanning.load(Ordering::Relaxed),
            files_found: self.files_found.load(Ordering::Relaxed),
            files_indexed: self.files_indexed.load(Ordering::Relaxed),
            current_dir: self.current_dir.lock().unwrap().clone(),
        }
    }
}

/// Launch a scan on a background blocking thread.
/// Returns immediately if a scan is already running.
pub fn start_scan(db: Arc<Database>, dirs: Vec<PathBuf>, progress: Arc<ScanProgress>) {
    if progress.scanning.swap(true, Ordering::SeqCst) {
        info!("Scan already in progress, skipping");
        return;
    }

    progress.files_found.store(0, Ordering::Relaxed);
    progress.files_indexed.store(0, Ordering::Relaxed);
    *progress.current_dir.lock().unwrap() = String::new();

    tokio::task::spawn_blocking(move || {
        for dir in &dirs {
            if dir.is_dir() {
                info!("Scanning directory: {}", dir.display());
                scan_dir(dir, &db, &progress);
            } else {
                tracing::warn!("Skipping non-existent directory: {}", dir.display());
            }
        }
        progress.scanning.store(false, Ordering::SeqCst);
        info!(
            "Scan complete — {} found, {} newly indexed",
            progress.files_found.load(Ordering::Relaxed),
            progress.files_indexed.load(Ordering::Relaxed),
        );
    });
}

fn scan_dir(dir: &Path, db: &Database, progress: &ScanProgress) {
    let now = chrono::Utc::now().to_rfc3339();
    let mut batch: Vec<NewMedia> = Vec::with_capacity(BATCH_SIZE);

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_ascii_lowercase(),
            None => continue,
        };

        let media_type = if PHOTO_EXTS.contains(&ext.as_str()) {
            "photo"
        } else if VIDEO_EXTS.contains(&ext.as_str()) {
            "video"
        } else {
            continue;
        };

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let dir_path = path
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();

        let metadata = entry.metadata().ok();
        let file_size = metadata.as_ref().map(|m| m.len() as i64).unwrap_or(0);
        let modified = metadata
            .as_ref()
            .and_then(|m| m.modified().ok())
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_default();

        let mime = mime_guess::from_path(path)
            .first()
            .map(|m| m.to_string())
            .unwrap_or_default();

        batch.push(NewMedia {
            path: path.to_string_lossy().to_string(),
            filename,
            dir_path: dir_path.clone(),
            file_size,
            media_type: media_type.to_string(),
            mime_type: mime,
            file_modified_at: modified,
            indexed_at: now.clone(),
        });

        if batch.len() >= BATCH_SIZE {
            flush_batch(&mut batch, db, progress, &dir_path);
        }
    }

    // Flush remaining
    if !batch.is_empty() {
        let dir_path = batch.last().map(|b| b.dir_path.clone()).unwrap_or_default();
        flush_batch(&mut batch, db, progress, &dir_path);
    }
}

fn flush_batch(batch: &mut Vec<NewMedia>, db: &Database, progress: &ScanProgress, dir: &str) {
    let found = batch.len() as u64;
    progress.files_found.fetch_add(found, Ordering::Relaxed);
    *progress.current_dir.lock().unwrap() = dir.to_string();

    match db.batch_insert(batch) {
        Ok(n) => {
            progress.files_indexed.fetch_add(n as u64, Ordering::Relaxed);
        }
        Err(e) => {
            tracing::error!("Batch insert failed: {e}");
        }
    }
    batch.clear();
}
