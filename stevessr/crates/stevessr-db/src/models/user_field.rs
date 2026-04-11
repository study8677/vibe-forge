use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserField {
    pub id: i64,
    pub name: String,
    pub field_type: String,
    pub description: Option<String>,
    pub required: bool,
    pub editable: bool,
    pub show_on_profile: bool,
    pub show_on_user_card: bool,
    pub position: i32,
    pub searchable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserField {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_fields ORDER BY position ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_fields WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        field_type: &str,
        description: Option<&str>,
        required: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_fields (name, field_type, description, required) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(name)
        .bind(field_type)
        .bind(description)
        .bind(required)
        .fetch_one(pool)
        .await
    }
}
