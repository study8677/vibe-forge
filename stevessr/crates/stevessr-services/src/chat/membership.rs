use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct ChatMembership;

impl ChatMembership {
    pub async fn join(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO user_chat_channel_memberships (chat_channel_id, user_id, following, muted, created_at, updated_at)
             VALUES ($1, $2, TRUE, FALSE, NOW(), NOW())
             ON CONFLICT (chat_channel_id, user_id) DO UPDATE SET following = TRUE, updated_at = NOW()"
        )
        .bind(channel_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn leave(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE user_chat_channel_memberships SET following = FALSE, updated_at = NOW()
             WHERE chat_channel_id = $1 AND user_id = $2"
        )
        .bind(channel_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mute(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE user_chat_channel_memberships SET muted = TRUE, updated_at = NOW()
             WHERE chat_channel_id = $1 AND user_id = $2"
        )
        .bind(channel_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unmute(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE user_chat_channel_memberships SET muted = FALSE, updated_at = NOW()
             WHERE chat_channel_id = $1 AND user_id = $2"
        )
        .bind(channel_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_read(pool: &PgPool, channel_id: i64, user_id: i64, message_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE user_chat_channel_memberships SET last_read_message_id = $3, updated_at = NOW()
             WHERE chat_channel_id = $1 AND user_id = $2 AND (last_read_message_id IS NULL OR last_read_message_id < $3)"
        )
        .bind(channel_id)
        .bind(user_id)
        .bind(message_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unread_count(pool: &PgPool, channel_id: i64, user_id: i64) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM chat_messages cm
             JOIN user_chat_channel_memberships ucm ON ucm.chat_channel_id = cm.chat_channel_id AND ucm.user_id = $2
             WHERE cm.chat_channel_id = $1
               AND cm.deleted_at IS NULL
               AND (ucm.last_read_message_id IS NULL OR cm.id > ucm.last_read_message_id)"
        )
        .bind(channel_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }
}
