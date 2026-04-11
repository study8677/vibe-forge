use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CategoryGroup {
    pub id: i64,
    pub category_id: i64,
    pub group_id: i64,
    pub permission_type: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryGroup {
    pub async fn find_by_category(pool: &PgPool, category_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM category_groups WHERE category_id = $1")
            .bind(category_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_group(pool: &PgPool, group_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM category_groups WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, category_id: i64, group_id: i64, permission_type: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO category_groups (category_id, group_id, permission_type) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(category_id)
        .bind(group_id)
        .bind(permission_type)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, category_id: i64, group_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM category_groups WHERE category_id = $1 AND group_id = $2")
            .bind(category_id)
            .bind(group_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
