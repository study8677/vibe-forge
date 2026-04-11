use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use ipnetwork::IpNetwork;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub username_lower: String,
    pub name: Option<String>,
    pub active: bool,
    pub approved: bool,
    pub approved_by_id: Option<i64>,
    pub approved_at: Option<DateTime<Utc>>,
    pub admin: bool,
    pub moderator: bool,
    pub trust_level: i16,
    pub staged: bool,
    pub date_of_birth: Option<NaiveDate>,
    pub ip_address: Option<IpNetwork>,
    pub registration_ip_address: Option<IpNetwork>,
    pub primary_group_id: Option<i64>,
    pub flair_group_id: Option<i64>,
    pub locale: Option<String>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub last_emailed_at: Option<DateTime<Utc>>,
    pub silenced_till: Option<DateTime<Utc>>,
    pub suspended_till: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub views: i32,
    pub flag_level: i16,
    pub title: Option<String>,
    pub uploaded_avatar_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM users WHERE username_lower = $1")
            .bind(username.to_lowercase())
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        username: &str,
        name: Option<&str>,
        active: bool,
        trust_level: i16,
        ip_address: Option<IpNetwork>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO users (username, username_lower, name, active, trust_level, ip_address, registration_ip_address)
               VALUES ($1, $2, $3, $4, $5, $6, $6)
               RETURNING *"#,
        )
        .bind(username)
        .bind(username.to_lowercase())
        .bind(name)
        .bind(active)
        .bind(trust_level)
        .bind(ip_address)
        .fetch_one(pool)
        .await
    }

    pub async fn update_last_seen(pool: &PgPool, id: i64, ip: Option<IpNetwork>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET last_seen_at = NOW(), ip_address = COALESCE($2, ip_address), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(ip)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_admin(pool: &PgPool, id: i64, admin: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET admin = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(admin)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_moderator(pool: &PgPool, id: i64, moderator: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET moderator = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(moderator)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_trust_level(pool: &PgPool, id: i64, level: i16) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET trust_level = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(level)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn suspend(pool: &PgPool, id: i64, until: DateTime<Utc>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET suspended_till = $2, suspended_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(until)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn unsuspend(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET suspended_till = NULL, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn silence(pool: &PgPool, id: i64, until: DateTime<Utc>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET silenced_till = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(until)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn activate(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET active = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub fn is_staff(&self) -> bool {
        self.admin || self.moderator
    }

    pub fn is_suspended(&self) -> bool {
        self.suspended_till.map(|t| t > Utc::now()).unwrap_or(false)
    }

    pub fn is_silenced(&self) -> bool {
        self.silenced_till.map(|t| t > Utc::now()).unwrap_or(false)
    }
}
