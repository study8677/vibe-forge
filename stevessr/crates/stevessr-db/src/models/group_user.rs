use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupUser {
    pub id: i64,
    pub group_id: i64,
    pub user_id: i64,
    pub owner: bool,
    pub notification_level: i16,
    pub first_unread_pm_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GroupUser {
    pub async fn find_by_group(pool: &PgPool, group_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM group_users WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM group_users WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_membership(pool: &PgPool, group_id: i64, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM group_users WHERE group_id = $1 AND user_id = $2")
            .bind(group_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, group_id: i64, user_id: i64, owner: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO group_users (group_id, user_id, owner) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(group_id)
        .bind(user_id)
        .bind(owner)
        .fetch_one(pool)
        .await
    }

    pub async fn find_group_ids_for_user(pool: &PgPool, user_id: i64) -> Result<Vec<i64>, sqlx::Error> {
        let rows: Vec<(i64,)> = sqlx::query_as("SELECT group_id FROM group_users WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    pub async fn delete(pool: &PgPool, group_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM group_users WHERE group_id = $1 AND user_id = $2")
            .bind(group_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
