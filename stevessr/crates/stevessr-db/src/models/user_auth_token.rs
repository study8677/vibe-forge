use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuthToken {
    pub id: i64,
    pub user_id: i64,
    pub auth_token: String,
    pub prev_auth_token: String,
    pub user_agent: Option<String>,
    pub auth_token_seen: bool,
    pub client_ip: Option<String>,
    pub rotated_at: DateTime<Utc>,
    pub seen_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAuthToken {
    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_auth_tokens WHERE auth_token = $1 OR prev_auth_token = $1",
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_auth_tokens WHERE user_id = $1 ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        auth_token: &str,
        user_agent: Option<&str>,
        client_ip: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO user_auth_tokens (user_id, auth_token, prev_auth_token, user_agent, client_ip, rotated_at)
               VALUES ($1, $2, $2, $3, $4, NOW()) RETURNING *"#,
        )
        .bind(user_id)
        .bind(auth_token)
        .bind(user_agent)
        .bind(client_ip)
        .fetch_one(pool)
        .await
    }

    pub async fn rotate(pool: &PgPool, id: i64, new_token: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE user_auth_tokens SET prev_auth_token = auth_token, auth_token = $2, auth_token_seen = FALSE, rotated_at = NOW(), updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(new_token)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete_all_for_user(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_auth_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
