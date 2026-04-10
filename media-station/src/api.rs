use crate::config::Config;
use crate::db::Database;
use crate::models::*;
use crate::scanner::ScanProgress;
use crate::thumbnail;

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

// ---------- Shared state ----------

pub struct AppState {
    pub db: Arc<Database>,
    pub progress: Arc<ScanProgress>,
    pub config: Config,
}

// ---------- Router ----------

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        // Static assets (embedded in binary)
        .route("/", get(index_html))
        .route("/style.css", get(style_css))
        .route("/app.js", get(app_js))
        // API
        .route("/api/media", get(list_media))
        .route("/api/media/:id", get(get_media))
        .route("/api/thumb/:id/:size", get(serve_thumb))
        .route("/api/file/:id", get(serve_file))
        .route("/api/dirs", get(list_dirs))
        .route("/api/timeline", get(timeline))
        .route("/api/stats", get(stats))
        .route("/api/scan", post(trigger_scan))
        .route("/api/scan/status", get(scan_status))
        .with_state(state)
}

// ---------- Static assets ----------

async fn index_html() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn style_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../static/style.css"),
    )
}

async fn app_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        include_str!("../static/app.js"),
    )
}

// ---------- Media list / detail ----------

async fn list_media(
    State(st): State<Arc<AppState>>,
    Query(q): Query<MediaQuery>,
) -> Result<Json<MediaList>, StatusCode> {
    st.db
        .query_media(&q)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_media(
    State(st): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<MediaItem>, StatusCode> {
    st.db
        .get_by_id(id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// ---------- Thumbnails ----------

async fn serve_thumb(
    State(st): State<Arc<AppState>>,
    Path((id, size)): Path<(i64, String)>,
) -> Result<Response, StatusCode> {
    if size != "sm" && size != "md" {
        return Err(StatusCode::BAD_REQUEST);
    }

    let path = thumbnail::thumb_path(&st.config.data_dir, id, &size);
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        [
            (header::CONTENT_TYPE, "image/jpeg"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        bytes,
    )
        .into_response())
}

// ---------- Original file (with HTTP range support for video seeking) ----------

async fn serve_file(
    State(st): State<Arc<AppState>>,
    Path(id): Path<i64>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let item = st
        .db
        .get_by_id(id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let file_path = std::path::Path::new(&item.path);
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Security: ensure the file is under one of the configured media dirs
    let allowed = st.config.media_dirs.iter().any(|d| {
        file_path
            .to_str()
            .map(|p| p.starts_with(d.to_string_lossy().as_ref()))
            .unwrap_or(false)
    });
    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }

    let meta = tokio::fs::metadata(file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_size = meta.len();
    let mime = item
        .mime_type
        .as_deref()
        .unwrap_or("application/octet-stream");

    // Range request?
    if let Some(range_hdr) = headers.get(header::RANGE) {
        if let Ok(range_str) = range_hdr.to_str() {
            if let Some((start, end)) = parse_range(range_str, file_size) {
                let length = end - start + 1;

                let mut file = tokio::fs::File::open(file_path)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                file.seek(std::io::SeekFrom::Start(start))
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let limited = file.take(length);
                let stream = ReaderStream::new(limited);
                let body = Body::from_stream(stream);

                return Ok(Response::builder()
                    .status(StatusCode::PARTIAL_CONTENT)
                    .header(header::CONTENT_TYPE, mime)
                    .header(header::CONTENT_LENGTH, length.to_string())
                    .header(
                        header::CONTENT_RANGE,
                        format!("bytes {start}-{end}/{file_size}"),
                    )
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(body)
                    .unwrap());
            }
        }
    }

    // Full file — stream to avoid loading into memory
    let file = tokio::fs::File::open(file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, mime)
        .header(header::CONTENT_LENGTH, file_size.to_string())
        .header(header::ACCEPT_RANGES, "bytes")
        .body(body)
        .unwrap())
}

fn parse_range(header: &str, file_size: u64) -> Option<(u64, u64)> {
    let range = header.strip_prefix("bytes=")?;
    let mut parts = range.splitn(2, '-');
    let start_str = parts.next()?;
    let end_str = parts.next()?;

    let start: u64 = if start_str.is_empty() {
        // suffix range: -500 means last 500 bytes
        let suffix: u64 = end_str.parse().ok()?;
        file_size.saturating_sub(suffix)
    } else {
        start_str.parse().ok()?
    };

    let end: u64 = if end_str.is_empty() {
        file_size - 1
    } else {
        end_str.parse().ok()?
    };

    if start > end || end >= file_size {
        return None;
    }
    Some((start, end))
}

// ---------- Directories ----------

async fn list_dirs(
    State(st): State<Arc<AppState>>,
    Query(q): Query<DirQuery>,
) -> Result<Json<Vec<DirEntry>>, StatusCode> {
    let parent = q.path.as_deref().unwrap_or("");

    if parent.is_empty() {
        // Return configured root media directories
        let roots: Vec<DirEntry> = st
            .config
            .media_dirs
            .iter()
            .map(|d| {
                let path = d.to_string_lossy().to_string();
                let name = d
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&path)
                    .to_string();
                DirEntry {
                    path,
                    name,
                    count: 0,
                    has_children: true,
                }
            })
            .collect();
        return Ok(Json(roots));
    }

    // Security: must be under a configured media dir
    let allowed = st
        .config
        .media_dirs
        .iter()
        .any(|d| parent.starts_with(d.to_string_lossy().as_ref()));
    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }

    st.db
        .list_subdirs(parent)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ---------- Timeline / Stats ----------

async fn timeline(State(st): State<Arc<AppState>>) -> Result<Json<Vec<TimelineGroup>>, StatusCode> {
    st.db
        .get_timeline()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn stats(State(st): State<Arc<AppState>>) -> Result<Json<Stats>, StatusCode> {
    st.db
        .get_stats()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ---------- Scan control ----------

async fn trigger_scan(State(st): State<Arc<AppState>>) -> Json<ScanStatus> {
    crate::scanner::start_scan(st.db.clone(), st.config.media_dirs.clone(), st.progress.clone());
    Json(st.progress.to_status())
}

async fn scan_status(State(st): State<Arc<AppState>>) -> Json<ScanStatus> {
    Json(st.progress.to_status())
}
