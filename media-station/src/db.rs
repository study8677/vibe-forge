use crate::models::*;
use rusqlite::{params, Connection, Result as SqlResult};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tracing::info;

/// High-performance SQLite database optimized for 10M+ media files.
pub struct Database {
    path: PathBuf,
    writer: Mutex<Connection>,
}

impl Database {
    pub fn open(data_dir: &Path) -> SqlResult<Self> {
        let path = data_dir.join("media.db");
        let conn = Connection::open(&path)?;
        Self::configure(&conn)?;
        Self::create_schema(&conn)?;
        info!("Database opened: {}", path.display());
        Ok(Self {
            path,
            writer: Mutex::new(conn),
        })
    }

    fn configure(conn: &Connection) -> SqlResult<()> {
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous  = NORMAL;
             PRAGMA cache_size   = -64000;
             PRAGMA mmap_size    = 268435456;
             PRAGMA temp_store   = MEMORY;
             PRAGMA busy_timeout = 5000;",
        )
    }

    fn create_schema(conn: &Connection) -> SqlResult<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS media (
                id              INTEGER PRIMARY KEY,
                path            TEXT    NOT NULL UNIQUE,
                filename        TEXT    NOT NULL,
                dir_path        TEXT    NOT NULL,
                file_size       INTEGER NOT NULL,
                media_type      TEXT    NOT NULL,
                mime_type       TEXT,
                width           INTEGER,
                height          INTEGER,
                taken_at        TEXT,
                file_modified_at TEXT   NOT NULL,
                indexed_at      TEXT    NOT NULL,
                thumb_status    INTEGER NOT NULL DEFAULT 0,
                orientation     INTEGER NOT NULL DEFAULT 1
             );

             CREATE INDEX IF NOT EXISTS idx_media_dir      ON media(dir_path);
             CREATE INDEX IF NOT EXISTS idx_media_type     ON media(media_type);
             CREATE INDEX IF NOT EXISTS idx_media_taken    ON media(taken_at DESC);
             CREATE INDEX IF NOT EXISTS idx_media_modified ON media(file_modified_at DESC);
             CREATE INDEX IF NOT EXISTS idx_media_thumb    ON media(thumb_status) WHERE thumb_status = 0;
             CREATE INDEX IF NOT EXISTS idx_media_filename ON media(filename);",
        )
    }

    fn reader(&self) -> SqlResult<Connection> {
        let conn = Connection::open_with_flags(
            &self.path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        Self::configure(&conn)?;
        Ok(conn)
    }

    // ==================== Write operations ====================

    pub fn batch_insert(&self, items: &[NewMedia]) -> SqlResult<usize> {
        let mut conn = self.writer.lock().unwrap();
        let tx = conn.transaction()?;
        let mut count = 0usize;
        {
            let mut stmt = tx.prepare_cached(
                "INSERT OR IGNORE INTO media
                    (path, filename, dir_path, file_size, media_type, mime_type, file_modified_at, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            )?;
            for item in items {
                count += stmt.execute(params![
                    item.path,
                    item.filename,
                    item.dir_path,
                    item.file_size,
                    item.media_type,
                    item.mime_type,
                    item.file_modified_at,
                    item.indexed_at,
                ])?;
            }
        }
        tx.commit()?;
        Ok(count)
    }

    pub fn update_thumb_done(
        &self,
        id: i64,
        width: i32,
        height: i32,
        taken_at: Option<String>,
        orientation: i32,
    ) -> SqlResult<()> {
        let conn = self.writer.lock().unwrap();
        conn.execute(
            "UPDATE media SET thumb_status = 1, width = ?1, height = ?2,
                              taken_at = COALESCE(?3, taken_at), orientation = ?4
             WHERE id = ?5",
            params![width, height, taken_at, orientation, id],
        )?;
        Ok(())
    }

    pub fn update_thumb_error(&self, id: i64) -> SqlResult<()> {
        let conn = self.writer.lock().unwrap();
        conn.execute(
            "UPDATE media SET thumb_status = 2 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // ==================== Read operations ====================

    pub fn query_media(&self, q: &MediaQuery) -> SqlResult<MediaList> {
        let conn = self.reader()?;
        let page = q.page.unwrap_or(1).max(1);
        let per_page = q.per_page.unwrap_or(100).clamp(1, 500);
        let offset = ((page - 1) * per_page) as i64;

        // Build dynamic WHERE clause with positional parameters
        let mut where_parts: Vec<&str> = Vec::new();
        let mut p: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref t) = q.media_type {
            where_parts.push("media_type = ?");
            p.push(Box::new(t.clone()));
        }
        if let Some(ref d) = q.dir {
            where_parts.push("dir_path = ?");
            p.push(Box::new(d.clone()));
        }
        if let Some(ref s) = q.q {
            where_parts.push("filename LIKE ?");
            p.push(Box::new(format!("%{s}%")));
        }

        let where_clause = if where_parts.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_parts.join(" AND "))
        };

        // Total count
        let count_sql = format!("SELECT COUNT(*) FROM media {where_clause}");
        let count_refs: Vec<&dyn rusqlite::types::ToSql> = p.iter().map(|v| v.as_ref()).collect();
        let total: i64 = conn.query_row(&count_sql, count_refs.as_slice(), |r| r.get(0))?;

        // Sort
        let sort_col = match q.sort.as_deref() {
            Some("name") => "filename",
            Some("size") => "file_size",
            Some("taken") => "COALESCE(taken_at, file_modified_at)",
            _ => "file_modified_at",
        };
        let order = if q.order.as_deref() == Some("asc") { "ASC" } else { "DESC" };

        let items_sql = format!(
            "SELECT id, path, filename, dir_path, file_size, media_type, mime_type,
                    width, height, taken_at, file_modified_at, indexed_at, thumb_status, orientation
             FROM media {where_clause} ORDER BY {sort_col} {order} LIMIT ? OFFSET ?"
        );

        p.push(Box::new(per_page as i64));
        p.push(Box::new(offset));
        let item_refs: Vec<&dyn rusqlite::types::ToSql> = p.iter().map(|v| v.as_ref()).collect();

        let mut stmt = conn.prepare(&items_sql)?;
        let result = stmt.query_map(item_refs.as_slice(), map_row)?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(MediaList {
            has_more: (offset + per_page as i64) < total,
            items: result,
            total,
            page,
            per_page,
        })
    }

    pub fn get_by_id(&self, id: i64) -> SqlResult<Option<MediaItem>> {
        let conn = self.reader()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, filename, dir_path, file_size, media_type, mime_type,
                    width, height, taken_at, file_modified_at, indexed_at, thumb_status, orientation
             FROM media WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], map_row)?;
        match rows.next() {
            Some(r) => Ok(Some(r?)),
            None => Ok(None),
        }
    }

    pub fn get_pending_thumbs(&self, limit: usize) -> SqlResult<Vec<MediaItem>> {
        let conn = self.reader()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, filename, dir_path, file_size, media_type, mime_type,
                    width, height, taken_at, file_modified_at, indexed_at, thumb_status, orientation
             FROM media WHERE thumb_status = 0 AND media_type = 'photo' LIMIT ?1",
        )?;
        let result = stmt.query_map(params![limit as i64], map_row)?
            .collect::<SqlResult<Vec<_>>>()?;
        Ok(result)
    }

    pub fn list_subdirs(&self, parent: &str) -> SqlResult<Vec<DirEntry>> {
        let conn = self.reader()?;

        let parent_trimmed = parent.trim_end_matches('/');
        let prefix = format!("{parent_trimmed}/");

        let mut stmt = conn.prepare(
            "SELECT dir_path, COUNT(*) FROM media WHERE dir_path LIKE ?1 GROUP BY dir_path",
        )?;

        let like_pattern = format!("{parent_trimmed}%");
        let rows: Vec<(String, i64)> = stmt
            .query_map(params![like_pattern], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })?
            .collect::<SqlResult<Vec<(String, i64)>>>()?;

        let mut children: BTreeMap<String, (i64, bool)> = BTreeMap::new();

        for (dir_path, count) in &rows {
            if dir_path == parent_trimmed {
                continue;
            }
            if let Some(rest) = dir_path.strip_prefix(&prefix) {
                let child_name = rest.split('/').next().unwrap_or(rest);
                let child_path = format!("{prefix}{child_name}");
                let deeper = rest.contains('/');
                children
                    .entry(child_path)
                    .and_modify(|(c, hc)| {
                        *c += count;
                        if deeper { *hc = true; }
                    })
                    .or_insert((*count, deeper));
            }
        }

        let mut entries: Vec<DirEntry> = children
            .into_iter()
            .map(|(path, (count, has_children))| {
                let name = path.rsplit('/').next().unwrap_or(&path).to_string();
                DirEntry { path, name, count, has_children }
            })
            .collect();
        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(entries)
    }

    pub fn get_timeline(&self) -> SqlResult<Vec<TimelineGroup>> {
        let conn = self.reader()?;
        let mut stmt = conn.prepare(
            "SELECT SUBSTR(COALESCE(taken_at, file_modified_at), 1, 7) AS month, COUNT(*)
             FROM media GROUP BY month ORDER BY month DESC",
        )?;
        let result = stmt.query_map([], |row| {
            Ok(TimelineGroup {
                date: row.get(0)?,
                count: row.get(1)?,
            })
        })?
        .collect::<SqlResult<Vec<_>>>()?;
        Ok(result)
    }

    pub fn get_stats(&self) -> SqlResult<Stats> {
        let conn = self.reader()?;
        let mut stmt = conn.prepare(
            "SELECT
                COUNT(*),
                SUM(CASE WHEN media_type='photo' THEN 1 ELSE 0 END),
                SUM(CASE WHEN media_type='video' THEN 1 ELSE 0 END),
                COALESCE(SUM(file_size), 0),
                SUM(CASE WHEN thumb_status=1 THEN 1 ELSE 0 END),
                SUM(CASE WHEN thumb_status=0 THEN 1 ELSE 0 END)
             FROM media",
        )?;
        stmt.query_row([], |row| {
            Ok(Stats {
                total_files: row.get(0)?,
                total_photos: row.get(1)?,
                total_videos: row.get(2)?,
                total_size: row.get(3)?,
                thumbs_done: row.get(4)?,
                thumbs_pending: row.get(5)?,
            })
        })
    }
}

fn map_row(row: &rusqlite::Row) -> SqlResult<MediaItem> {
    Ok(MediaItem {
        id: row.get(0)?,
        path: row.get(1)?,
        filename: row.get(2)?,
        dir_path: row.get(3)?,
        file_size: row.get(4)?,
        media_type: row.get(5)?,
        mime_type: row.get(6)?,
        width: row.get(7)?,
        height: row.get(8)?,
        taken_at: row.get(9)?,
        file_modified_at: row.get(10)?,
        indexed_at: row.get(11)?,
        thumb_status: row.get(12)?,
        orientation: row.get(13)?,
    })
}
