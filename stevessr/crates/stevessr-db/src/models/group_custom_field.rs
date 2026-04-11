use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupCustomField {
    pub id: i64,
    pub group_id: i64,
    pub name: String,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GroupCustomField {
    pub async fn find_by_group(pool: &PgPool, group_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM group_custom_fields WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, group_id: i64, name: &str, value: Option<&str>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO group_custom_fields (group_id, name, value)
               VALUES ($1, $2, $3)
               ON CONFLICT (group_id, name) DO UPDATE SET value = $3, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(group_id)
        .bind(name)
        .bind(value)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM group_custom_fields WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
