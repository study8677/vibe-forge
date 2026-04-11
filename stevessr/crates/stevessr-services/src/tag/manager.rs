use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};

pub struct TagManager;

impl TagManager {
    pub async fn create(pool: &PgPool, name: &str, description: Option<&str>) -> Result<i64> {
        Self::validate_name(name)?;

        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM tags WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(Error::AlreadyExists {
                resource: "tag",
                detail: format!("tag '{}' already exists", name),
            });
        }

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO tags (name, description, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) RETURNING id"
        )
        .bind(name)
        .bind(description)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn rename(pool: &PgPool, tag_id: i64, new_name: &str) -> Result<()> {
        Self::validate_name(new_name)?;

        sqlx::query("UPDATE tags SET name = $2, updated_at = NOW() WHERE id = $1")
            .bind(tag_id)
            .bind(new_name)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn destroy(pool: &PgPool, tag_id: i64) -> Result<()> {
        // Remove from topics
        sqlx::query("DELETE FROM topic_tags WHERE tag_id = $1")
            .bind(tag_id)
            .execute(pool)
            .await?;

        // Remove from tag groups
        sqlx::query("DELETE FROM tag_group_memberships WHERE tag_id = $1")
            .bind(tag_id)
            .execute(pool)
            .await?;

        // Delete the tag
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(tag_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn add_to_topic(pool: &PgPool, topic_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO topic_tags (topic_id, tag_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) ON CONFLICT DO NOTHING"
        )
        .bind(topic_id)
        .bind(tag_id)
        .execute(pool)
        .await?;

        // Update tag topic count
        sqlx::query("UPDATE tags SET topic_count = (SELECT COUNT(*) FROM topic_tags WHERE tag_id = $1) WHERE id = $1")
            .bind(tag_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn remove_from_topic(pool: &PgPool, topic_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM topic_tags WHERE topic_id = $1 AND tag_id = $2")
            .bind(topic_id)
            .bind(tag_id)
            .execute(pool)
            .await?;

        sqlx::query("UPDATE tags SET topic_count = (SELECT COUNT(*) FROM topic_tags WHERE tag_id = $1) WHERE id = $1")
            .bind(tag_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    fn validate_name(name: &str) -> Result<()> {
        let mut errors = ValidationErrors::new();

        if name.is_empty() {
            errors.add("name", "must not be empty");
        }
        if name.len() > 100 {
            errors.add("name", "must be at most 100 characters");
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            errors.add("name", "must contain only letters, numbers, hyphens, and underscores");
        }

        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
