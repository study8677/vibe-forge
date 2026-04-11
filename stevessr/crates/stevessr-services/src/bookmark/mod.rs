pub mod reminder;

use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct BookmarkManager;

impl BookmarkManager {
    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        bookmarkable_type: &str,
        bookmarkable_id: i64,
        name: Option<&str>,
        reminder_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<i64> {
        // Check for duplicate bookmark
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM bookmarks WHERE user_id = $1 AND bookmarkable_type = $2 AND bookmarkable_id = $3"
        )
        .bind(user_id)
        .bind(bookmarkable_type)
        .bind(bookmarkable_id)
        .fetch_optional(pool)
        .await?;

        if let Some((id,)) = existing {
            // Update the existing bookmark
            sqlx::query(
                "UPDATE bookmarks SET name = COALESCE($2, name), reminder_at = $3, updated_at = NOW() WHERE id = $1"
            )
            .bind(id)
            .bind(name)
            .bind(reminder_at)
            .execute(pool)
            .await?;
            return Ok(id);
        }

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO bookmarks (user_id, bookmarkable_type, bookmarkable_id, name, reminder_at, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id"
        )
        .bind(user_id)
        .bind(bookmarkable_type)
        .bind(bookmarkable_id)
        .bind(name)
        .bind(reminder_at)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn destroy(pool: &PgPool, bookmark_id: i64, user_id: i64) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM bookmarks WHERE id = $1 AND user_id = $2"
        )
        .bind(bookmark_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound {
                resource: "bookmark",
                id: bookmark_id.to_string(),
            });
        }

        Ok(())
    }

    pub async fn list_for_user(pool: &PgPool, user_id: i64, page: i64, per_page: i64) -> Result<Vec<(i64, String, i64, Option<String>, Option<chrono::DateTime<chrono::Utc>>)>> {
        let offset = (page - 1) * per_page;
        let rows = sqlx::query_as(
            "SELECT id, bookmarkable_type, bookmarkable_id, name, reminder_at
             FROM bookmarks
             WHERE user_id = $1
             ORDER BY updated_at DESC
             LIMIT $2 OFFSET $3"
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}
