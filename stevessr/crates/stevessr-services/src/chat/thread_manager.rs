use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct ChatThreadManager;

impl ChatThreadManager {
    /// Create a thread from a chat message.
    pub async fn create(pool: &PgPool, channel_id: i64, original_message_id: i64, title: Option<&str>) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO chat_threads (channel_id, original_message_id, title, status, created_at, updated_at)
             VALUES ($1, $2, $3, 'open', NOW(), NOW()) RETURNING id"
        )
        .bind(channel_id)
        .bind(original_message_id)
        .bind(title)
        .fetch_one(pool)
        .await?;

        // Link the original message to this thread
        sqlx::query("UPDATE chat_messages SET thread_id = $2 WHERE id = $1")
            .bind(original_message_id)
            .bind(row.0)
            .execute(pool)
            .await?;

        Ok(row.0)
    }

    /// Close a thread.
    pub async fn close(pool: &PgPool, thread_id: i64) -> Result<()> {
        sqlx::query("UPDATE chat_threads SET status = 'closed', updated_at = NOW() WHERE id = $1")
            .bind(thread_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Get the message count in a thread.
    pub async fn message_count(pool: &PgPool, thread_id: i64) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM chat_messages WHERE thread_id = $1 AND deleted_at IS NULL"
        )
        .bind(thread_id)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }
}
