use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};

pub struct CreateChatMessageParams {
    pub channel_id: i64,
    pub user_id: i64,
    pub message: String,
    pub in_reply_to_id: Option<i64>,
    pub thread_id: Option<i64>,
}

pub struct ChatMessageCreator;

impl ChatMessageCreator {
    pub async fn create(pool: &PgPool, params: CreateChatMessageParams) -> Result<i64> {
        Self::validate(&params)?;

        // Verify channel is open
        let channel_status: Option<(String,)> = sqlx::query_as(
            "SELECT status FROM chat_channels WHERE id = $1"
        )
        .bind(params.channel_id)
        .fetch_optional(pool)
        .await?;

        match channel_status {
            Some((status,)) if status != "open" => {
                return Err(Error::Forbidden(format!("channel is {}", status)));
            }
            None => {
                return Err(Error::NotFound {
                    resource: "chat_channel",
                    id: params.channel_id.to_string(),
                });
            }
            _ => {}
        }

        // Verify user is a member
        let membership: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM user_chat_channel_memberships WHERE chat_channel_id = $1 AND user_id = $2"
        )
        .bind(params.channel_id)
        .bind(params.user_id)
        .fetch_optional(pool)
        .await?;

        if membership.is_none() {
            return Err(Error::Forbidden("you are not a member of this channel".into()));
        }

        // Create the message
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO chat_messages (chat_channel_id, user_id, message, in_reply_to_id, thread_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id"
        )
        .bind(params.channel_id)
        .bind(params.user_id)
        .bind(&params.message)
        .bind(params.in_reply_to_id)
        .bind(params.thread_id)
        .fetch_one(pool)
        .await?;

        // Update channel last message
        sqlx::query(
            "UPDATE chat_channels SET messages_count = messages_count + 1, updated_at = NOW() WHERE id = $1"
        )
        .bind(params.channel_id)
        .execute(pool)
        .await?;

        // Update membership last read
        sqlx::query(
            "UPDATE user_chat_channel_memberships SET last_read_message_id = $3, updated_at = NOW()
             WHERE chat_channel_id = $1 AND user_id = $2"
        )
        .bind(params.channel_id)
        .bind(params.user_id)
        .bind(row.0)
        .execute(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn edit(pool: &PgPool, message_id: i64, user_id: i64, new_message: &str) -> Result<()> {
        let result = sqlx::query(
            "UPDATE chat_messages SET message = $3, updated_at = NOW() WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL"
        )
        .bind(message_id)
        .bind(user_id)
        .bind(new_message)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::Forbidden("you can only edit your own messages".into()));
        }

        Ok(())
    }

    pub async fn delete(pool: &PgPool, message_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE chat_messages SET deleted_at = NOW(), deleted_by_id = $2 WHERE id = $1"
        )
        .bind(message_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    fn validate(params: &CreateChatMessageParams) -> Result<()> {
        let mut errors = ValidationErrors::new();
        if params.message.is_empty() {
            errors.add("message", "must not be empty");
        }
        if params.message.len() > 6000 {
            errors.add("message", "must be at most 6000 characters");
        }
        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
