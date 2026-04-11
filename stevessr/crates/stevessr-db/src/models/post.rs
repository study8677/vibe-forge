use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
    pub post_number: i32,
    pub raw: String,
    pub cooked: String,
    pub reply_to_post_number: Option<i32>,
    pub reply_count: i32,
    pub quote_count: i32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub off_topic_count: i32,
    pub like_count: i32,
    pub incoming_link_count: i32,
    pub bookmark_count: i32,
    pub score: Option<f64>,
    pub reads: i32,
    pub post_type: i32,
    pub sort_order: Option<i32>,
    pub last_editor_id: Option<i64>,
    pub hidden: bool,
    pub hidden_reason_id: Option<i32>,
    pub notify_moderators_count: i32,
    pub spam_count: i32,
    pub illegal_count: i32,
    pub inappropriate_count: i32,
    pub last_version_at: DateTime<Utc>,
    pub user_deleted: bool,
    pub reply_to_user_id: Option<i64>,
    pub percent_rank: Option<f64>,
    pub notify_user_count: i32,
    pub like_score: i32,
    pub deleted_by_id: Option<i64>,
    pub edit_reason: Option<String>,
    pub word_count: Option<i32>,
    pub version: i32,
    pub cook_method: i32,
    pub wiki: bool,
    pub baked_at: Option<DateTime<Utc>>,
    pub baked_version: Option<i32>,
    pub hidden_at: Option<DateTime<Utc>>,
    pub self_edits: i32,
    pub reply_quoted: bool,
    pub via_email: bool,
    pub raw_email: Option<String>,
    pub public_version: i32,
    pub action_code: Option<String>,
    pub locked_by_id: Option<i64>,
    pub image_upload_id: Option<i64>,
    pub outbound_message_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_topic(pool: &PgPool, topic_id: i64, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE topic_id = $1 AND deleted_at IS NULL ORDER BY post_number ASC LIMIT $2 OFFSET $3",
        )
        .bind(topic_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_post_number(pool: &PgPool, topic_id: i64, post_number: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM posts WHERE topic_id = $1 AND post_number = $2")
            .bind(topic_id)
            .bind(post_number)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE user_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        topic_id: i64,
        post_number: i32,
        raw: &str,
        cooked: &str,
        reply_to_post_number: Option<i32>,
        post_type: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO posts (user_id, topic_id, post_number, raw, cooked, reply_to_post_number, post_type, last_version_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, NOW()) RETURNING *"#,
        )
        .bind(user_id)
        .bind(topic_id)
        .bind(post_number)
        .bind(raw)
        .bind(cooked)
        .bind(reply_to_post_number)
        .bind(post_type)
        .fetch_one(pool)
        .await
    }

    pub async fn update_content(pool: &PgPool, id: i64, raw: &str, cooked: &str, edit_reason: Option<&str>, editor_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE posts SET raw = $2, cooked = $3, edit_reason = $4, last_editor_id = $5, version = version + 1,
               public_version = public_version + 1, last_version_at = NOW(), self_edits = self_edits + 1, updated_at = NOW()
               WHERE id = $1"#,
        )
        .bind(id)
        .bind(raw)
        .bind(cooked)
        .bind(edit_reason)
        .bind(editor_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn soft_delete(pool: &PgPool, id: i64, deleted_by_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE posts SET deleted_at = NOW(), deleted_by_id = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(deleted_by_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn recover(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE posts SET deleted_at = NULL, deleted_by_id = NULL, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_like_count(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE posts SET like_count = like_count + 1, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn toggle_wiki(pool: &PgPool, id: i64, wiki: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE posts SET wiki = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(wiki)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn hide(pool: &PgPool, id: i64, reason_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE posts SET hidden = TRUE, hidden_reason_id = $2, hidden_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(reason_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn unhide(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE posts SET hidden = FALSE, hidden_reason_id = NULL, hidden_at = NULL, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
