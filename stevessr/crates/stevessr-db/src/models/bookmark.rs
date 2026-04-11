use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bookmark {
    pub id: i64,
    pub user_id: i64,
    pub bookmarkable_id: i64,
    pub bookmarkable_type: String,
    pub name: Option<String>,
    pub reminder_at: Option<DateTime<Utc>>,
    pub auto_delete_preference: i16,
    pub pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Bookmark {
    pub async fn find_by_user(pool: &PgPool, user_id: i64, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM bookmarks WHERE user_id = $1 ORDER BY pinned DESC, updated_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM bookmarks WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_pending_reminders(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM bookmarks WHERE reminder_at IS NOT NULL AND reminder_at <= NOW() ORDER BY reminder_at ASC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        bookmarkable_id: i64,
        bookmarkable_type: &str,
        name: Option<&str>,
        reminder_at: Option<DateTime<Utc>>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO bookmarks (user_id, bookmarkable_id, bookmarkable_type, name, reminder_at)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(user_id)
        .bind(bookmarkable_id)
        .bind(bookmarkable_type)
        .bind(name)
        .bind(reminder_at)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM bookmarks WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
