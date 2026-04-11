use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagGroupMembership {
    pub id: i64,
    pub tag_id: i64,
    pub tag_group_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TagGroupMembership {
    pub async fn find_by_tag_group(pool: &PgPool, tag_group_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM tag_group_memberships WHERE tag_group_id = $1")
            .bind(tag_group_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, tag_id: i64, tag_group_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO tag_group_memberships (tag_id, tag_group_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(tag_id)
        .bind(tag_group_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, tag_id: i64, tag_group_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tag_group_memberships WHERE tag_id = $1 AND tag_group_id = $2")
            .bind(tag_id)
            .bind(tag_group_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
