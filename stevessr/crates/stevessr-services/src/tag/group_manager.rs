use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Manages tag groups, which restrict which tags can be applied to
/// topics in certain categories.
pub struct TagGroupManager;

impl TagGroupManager {
    pub async fn create(pool: &PgPool, name: &str) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO tag_groups (name, created_at, updated_at) VALUES ($1, NOW(), NOW()) RETURNING id"
        )
        .bind(name)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    pub async fn add_tag(pool: &PgPool, tag_group_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO tag_group_memberships (tag_group_id, tag_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) ON CONFLICT DO NOTHING"
        )
        .bind(tag_group_id)
        .bind(tag_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn remove_tag(pool: &PgPool, tag_group_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query(
            "DELETE FROM tag_group_memberships WHERE tag_group_id = $1 AND tag_id = $2"
        )
        .bind(tag_group_id)
        .bind(tag_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn destroy(pool: &PgPool, tag_group_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM tag_group_memberships WHERE tag_group_id = $1")
            .bind(tag_group_id)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM tag_groups WHERE id = $1")
            .bind(tag_group_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Get all tag IDs that belong to a tag group.
    pub async fn tag_ids_for_group(pool: &PgPool, tag_group_id: i64) -> Result<Vec<i64>> {
        let rows: Vec<(i64,)> = sqlx::query_as(
            "SELECT tag_id FROM tag_group_memberships WHERE tag_group_id = $1"
        )
        .bind(tag_group_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    /// Validate that the given tags are allowed for a category.
    pub async fn validate_tags_for_category(
        pool: &PgPool,
        category_id: i64,
        tag_ids: &[i64],
    ) -> Result<()> {
        // Find tag groups restricted to this category
        let restricted: Vec<(i64,)> = sqlx::query_as(
            "SELECT tag_group_id FROM category_tag_groups WHERE category_id = $1"
        )
        .bind(category_id)
        .fetch_all(pool)
        .await?;

        if restricted.is_empty() {
            return Ok(()); // No restrictions
        }

        // Collect all allowed tag IDs
        let mut allowed_tag_ids = Vec::new();
        for (tg_id,) in &restricted {
            let mut ids = Self::tag_ids_for_group(pool, *tg_id).await?;
            allowed_tag_ids.append(&mut ids);
        }

        for tid in tag_ids {
            if !allowed_tag_ids.contains(tid) {
                return Err(Error::Forbidden(
                    format!("tag {} is not allowed in this category", tid),
                ));
            }
        }

        Ok(())
    }
}
