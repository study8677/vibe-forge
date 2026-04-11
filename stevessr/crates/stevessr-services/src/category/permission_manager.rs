use sqlx::PgPool;
use stevessr_core::error::Result;

/// Permission levels for category access.
pub const PERMISSION_CREATE_REPLY_SEE: i32 = 1;
pub const PERMISSION_REPLY_SEE: i32 = 2;
pub const PERMISSION_SEE: i32 = 3;

pub struct CategoryPermissionManager;

impl CategoryPermissionManager {
    pub async fn set_permissions(
        pool: &PgPool,
        category_id: i64,
        permissions: &[(i64, i32)], // (group_id, permission_level)
    ) -> Result<()> {
        // Remove existing permissions
        sqlx::query("DELETE FROM category_groups WHERE category_id = $1")
            .bind(category_id)
            .execute(pool)
            .await?;

        // Insert new permissions
        for (group_id, permission_type) in permissions {
            sqlx::query(
                "INSERT INTO category_groups (category_id, group_id, permission_type, created_at, updated_at)
                 VALUES ($1, $2, $3, NOW(), NOW())"
            )
            .bind(category_id)
            .bind(group_id)
            .bind(permission_type)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn can_see(pool: &PgPool, category_id: i64, group_ids: &[i64]) -> Result<bool> {
        if group_ids.is_empty() {
            // Anonymous users: check if "everyone" (group 0) has permission
            let row: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM category_groups WHERE category_id = $1 AND group_id = 0"
            )
            .bind(category_id)
            .fetch_optional(pool)
            .await?;
            return Ok(row.is_some());
        }

        // Check if any of the user's groups have at least SEE permission
        for gid in group_ids {
            let row: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM category_groups WHERE category_id = $1 AND group_id = $2"
            )
            .bind(category_id)
            .bind(gid)
            .fetch_optional(pool)
            .await?;
            if row.is_some() {
                return Ok(true);
            }
        }

        // If no category_groups rows exist, category is open to everyone
        let any_restriction: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM category_groups WHERE category_id = $1 LIMIT 1"
        )
        .bind(category_id)
        .fetch_optional(pool)
        .await?;

        Ok(any_restriction.is_none())
    }

    pub async fn can_create(pool: &PgPool, category_id: i64, group_ids: &[i64]) -> Result<bool> {
        for gid in group_ids {
            let row: Option<(i32,)> = sqlx::query_as(
                "SELECT permission_type FROM category_groups WHERE category_id = $1 AND group_id = $2"
            )
            .bind(category_id)
            .bind(gid)
            .fetch_optional(pool)
            .await?;
            if let Some((perm,)) = row {
                if perm <= PERMISSION_CREATE_REPLY_SEE {
                    return Ok(true);
                }
            }
        }

        // If no restrictions, everyone can create
        let any_restriction: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM category_groups WHERE category_id = $1 LIMIT 1"
        )
        .bind(category_id)
        .fetch_optional(pool)
        .await?;

        Ok(any_restriction.is_none())
    }
}
