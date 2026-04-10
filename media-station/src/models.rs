use serde::{Deserialize, Serialize};

// ---------- Database row ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub dir_path: String,
    pub file_size: i64,
    pub media_type: String,
    pub mime_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub taken_at: Option<String>,
    pub file_modified_at: String,
    pub indexed_at: String,
    pub thumb_status: i32,
    pub orientation: i32,
}

// ---------- Insert payload ----------

pub struct NewMedia {
    pub path: String,
    pub filename: String,
    pub dir_path: String,
    pub file_size: i64,
    pub media_type: String,
    pub mime_type: String,
    pub file_modified_at: String,
    pub indexed_at: String,
}

// ---------- API query ----------

#[derive(Debug, Deserialize)]
pub struct MediaQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub dir: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DirQuery {
    pub path: Option<String>,
}

// ---------- API responses ----------

#[derive(Debug, Serialize)]
pub struct MediaList {
    pub items: Vec<MediaItem>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct DirEntry {
    pub path: String,
    pub name: String,
    pub count: i64,
    pub has_children: bool,
}

#[derive(Debug, Serialize)]
pub struct TimelineGroup {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct Stats {
    pub total_files: i64,
    pub total_photos: i64,
    pub total_videos: i64,
    pub total_size: i64,
    pub thumbs_done: i64,
    pub thumbs_pending: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct ScanStatus {
    pub scanning: bool,
    pub files_found: u64,
    pub files_indexed: u64,
    pub current_dir: String,
}
