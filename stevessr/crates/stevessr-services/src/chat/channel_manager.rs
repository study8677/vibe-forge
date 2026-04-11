use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};

pub struct CreateChannelParams {
    pub name: String,
    pub description: Option<String>,
    pub chatable_type: String, // "Category", "DirectMessage"
    pub chatable_id: i64,
    pub created_by_id: i64,
}

pub struct ChatChannelManager;

impl ChatChannelManager {
    pub async fn create(pool: &PgPool, params: CreateChannelParams) -> Result<i64> {
        let mut errors = ValidationErrors::new();
        if params.name.is_empty() {
            errors.add("name", "must not be empty");
        }
        if params.name.len() > 100 {
            errors.add("name", "must be at most 100 characters");
        }
        if !errors.is_empty() {
            return Err(Error::Validation(errors));
        }

        let slug = slug::slugify(&params.name);

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO chat_channels (name, slug, description, chatable_type, chatable_id, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, 'open', NOW(), NOW()) RETURNING id"
        )
        .bind(&params.name)
        .bind(&slug)
        .bind(&params.description)
        .bind(&params.chatable_type)
        .bind(params.chatable_id)
        .fetch_one(pool)
        .await?;

        // Add the creator as a member
        super::membership::ChatMembership::join(pool, row.0, params.created_by_id).await?;

        Ok(row.0)
    }

    pub async fn update(pool: &PgPool, channel_id: i64, name: Option<&str>, description: Option<&str>) -> Result<()> {
        let slug = name.map(slug::slugify);
        sqlx::query(
            "UPDATE chat_channels SET name = COALESCE($2, name), slug = COALESCE($3, slug), description = COALESCE($4, description), updated_at = NOW() WHERE id = $1"
        )
        .bind(channel_id)
        .bind(name)
        .bind(slug.as_deref())
        .bind(description)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn close(pool: &PgPool, channel_id: i64) -> Result<()> {
        sqlx::query("UPDATE chat_channels SET status = 'closed', updated_at = NOW() WHERE id = $1")
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn archive(pool: &PgPool, channel_id: i64) -> Result<()> {
        sqlx::query("UPDATE chat_channels SET status = 'archived', updated_at = NOW() WHERE id = $1")
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn destroy(pool: &PgPool, channel_id: i64) -> Result<()> {
        // Delete messages
        sqlx::query("DELETE FROM chat_messages WHERE chat_channel_id = $1")
            .bind(channel_id)
            .execute(pool)
            .await?;
        // Delete memberships
        sqlx::query("DELETE FROM user_chat_channel_memberships WHERE chat_channel_id = $1")
            .bind(channel_id)
            .execute(pool)
            .await?;
        // Delete channel
        sqlx::query("DELETE FROM chat_channels WHERE id = $1")
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Create a direct message channel between users.
    pub async fn create_dm(pool: &PgPool, user_ids: &[i64]) -> Result<i64> {
        // Check for existing DM channel with these exact users
        // For simplicity, create a new one
        let name = format!("DM-{}", user_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join("-"));

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO chat_channels (name, slug, chatable_type, chatable_id, status, created_at, updated_at)
             VALUES ($1, $2, 'DirectMessage', 0, 'open', NOW(), NOW()) RETURNING id"
        )
        .bind(&name)
        .bind(slug::slugify(&name))
        .fetch_one(pool)
        .await?;

        for uid in user_ids {
            super::membership::ChatMembership::join(pool, row.0, *uid).await?;
        }

        Ok(row.0)
    }
}
