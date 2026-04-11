use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct CategoryDestroyer;

impl CategoryDestroyer {
    pub async fn destroy(pool: &PgPool, category_id: i64) -> Result<()> {
        // Check if category has topics
        let topic_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM topics WHERE category_id = $1 AND deleted_at IS NULL"
        )
        .bind(category_id)
        .fetch_one(pool)
        .await?;

        if topic_count.0 > 0 {
            return Err(Error::Forbidden(
                format!("cannot delete category with {} topics; move or delete them first", topic_count.0),
            ));
        }

        // Check for subcategories
        let sub_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM categories WHERE parent_category_id = $1"
        )
        .bind(category_id)
        .fetch_one(pool)
        .await?;

        if sub_count.0 > 0 {
            return Err(Error::Forbidden(
                "cannot delete category with subcategories".into(),
            ));
        }

        // Delete category permissions
        sqlx::query("DELETE FROM category_groups WHERE category_id = $1")
            .bind(category_id)
            .execute(pool)
            .await?;

        // Delete the category
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(category_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
