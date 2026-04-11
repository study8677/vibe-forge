use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostAction {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub post_action_type_id: i32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_id: Option<i64>,
    pub related_post_id: Option<i64>,
    pub staff_took_action: bool,
    pub deferred_by_id: Option<i64>,
    pub targets_topic: bool,
    pub agreed_at: Option<DateTime<Utc>>,
    pub agreed_by_id: Option<i64>,
    pub deferred_at: Option<DateTime<Utc>>,
    pub disagreed_at: Option<DateTime<Utc>>,
    pub disagreed_by_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostAction {
    pub async fn find_by_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_actions WHERE post_id = $1 AND deleted_at IS NULL")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_user_and_post(pool: &PgPool, user_id: i64, post_id: i64, action_type: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM post_actions WHERE user_id = $1 AND post_id = $2 AND post_action_type_id = $3 AND deleted_at IS NULL",
        )
        .bind(user_id)
        .bind(post_id)
        .bind(action_type)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        post_id: i64,
        user_id: i64,
        post_action_type_id: i32,
        staff_took_action: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO post_actions (post_id, user_id, post_action_type_id, staff_took_action)
               VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(post_id)
        .bind(user_id)
        .bind(post_action_type_id)
        .bind(staff_took_action)
        .fetch_one(pool)
        .await
    }

    pub async fn soft_delete(pool: &PgPool, id: i64, deleted_by_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE post_actions SET deleted_at = NOW(), deleted_by_id = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(deleted_by_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
