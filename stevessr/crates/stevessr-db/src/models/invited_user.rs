use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InvitedUser {
    pub id: i64,
    pub user_id: Option<i64>,
    pub invite_id: i64,
    pub redeemed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InvitedUser {
    pub async fn find_by_invite(pool: &PgPool, invite_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM invited_users WHERE invite_id = $1")
            .bind(invite_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, invite_id: i64, user_id: Option<i64>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO invited_users (invite_id, user_id, redeemed_at) VALUES ($1, $2, NOW()) RETURNING *",
        )
        .bind(invite_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }
}
