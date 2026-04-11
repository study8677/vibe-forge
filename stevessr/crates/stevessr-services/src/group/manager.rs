use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};

pub struct CreateGroupParams {
    pub name: String,
    pub full_name: Option<String>,
    pub visibility_level: i32,
    pub mentionable_level: i32,
    pub messageable_level: i32,
    pub bio_raw: Option<String>,
}

pub struct GroupManager;

impl GroupManager {
    pub async fn create(pool: &PgPool, params: CreateGroupParams) -> Result<i64> {
        Self::validate(&params)?;

        // Check name uniqueness
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM groups WHERE name = $1"
        )
        .bind(&params.name)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(Error::AlreadyExists {
                resource: "group",
                detail: format!("group '{}' already exists", params.name),
            });
        }

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO groups (name, full_name, visibility_level, mentionable_level, messageable_level, bio_raw, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW()) RETURNING id"
        )
        .bind(&params.name)
        .bind(&params.full_name)
        .bind(params.visibility_level)
        .bind(params.mentionable_level)
        .bind(params.messageable_level)
        .bind(&params.bio_raw)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn add_member(pool: &PgPool, group_id: i64, user_id: i64) -> Result<()> {
        // Check not already a member
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM group_users WHERE group_id = $1 AND user_id = $2"
        )
        .bind(group_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Ok(()); // Idempotent
        }

        sqlx::query(
            "INSERT INTO group_users (group_id, user_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())"
        )
        .bind(group_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        // Update group user count
        sqlx::query("UPDATE groups SET user_count = user_count + 1 WHERE id = $1")
            .bind(group_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn remove_member(pool: &PgPool, group_id: i64, user_id: i64) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM group_users WHERE group_id = $1 AND user_id = $2"
        )
        .bind(group_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() > 0 {
            sqlx::query("UPDATE groups SET user_count = GREATEST(user_count - 1, 0) WHERE id = $1")
                .bind(group_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn set_owner(pool: &PgPool, group_id: i64, user_id: i64, owner: bool) -> Result<()> {
        sqlx::query(
            "UPDATE group_users SET owner = $3 WHERE group_id = $1 AND user_id = $2"
        )
        .bind(group_id)
        .bind(user_id)
        .bind(owner)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn destroy(pool: &PgPool, group_id: i64) -> Result<()> {
        // Remove all members
        sqlx::query("DELETE FROM group_users WHERE group_id = $1")
            .bind(group_id)
            .execute(pool)
            .await?;

        // Remove category permissions
        sqlx::query("DELETE FROM category_groups WHERE group_id = $1")
            .bind(group_id)
            .execute(pool)
            .await?;

        // Delete the group
        sqlx::query("DELETE FROM groups WHERE id = $1")
            .bind(group_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    fn validate(params: &CreateGroupParams) -> Result<()> {
        let mut errors = ValidationErrors::new();

        if params.name.is_empty() {
            errors.add("name", "must not be empty");
        }
        if params.name.len() > 50 {
            errors.add("name", "must be at most 50 characters");
        }
        if !params.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            errors.add("name", "must contain only letters, numbers, underscores, and hyphens");
        }

        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
