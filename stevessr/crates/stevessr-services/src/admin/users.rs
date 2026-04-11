use sqlx::PgPool;
use stevessr_core::error::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct AdminUserRecord {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_template: Option<String>,
    pub active: bool,
    pub admin: bool,
    pub moderator: bool,
    pub trust_level: i16,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_seen_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_emailed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub approved: bool,
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub suspended_till: Option<chrono::DateTime<chrono::Utc>>,
    pub silenced_till: Option<chrono::DateTime<chrono::Utc>>,
    pub staged: bool,
    pub ip_address: Option<String>,
    pub registration_ip_address: Option<String>,
    pub topics_entered: i64,
    pub posts_read_count: i64,
    pub post_count: i64,
    pub topic_count: i64,
    pub flags_given: i64,
    pub flags_received: i64,
    pub time_read: i64,
    pub days_visited: i64,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SuspendResult {
    pub suspended_at: chrono::DateTime<chrono::Utc>,
    pub suspended_till: chrono::DateTime<chrono::Utc>,
    pub suspend_reason: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SilenceResult {
    pub silenced_at: chrono::DateTime<chrono::Utc>,
    pub silenced_till: chrono::DateTime<chrono::Utc>,
    pub silence_reason: String,
}

pub async fn list_users(_pool: &PgPool, _filter: Option<&str>, _order: Option<&str>, _asc: bool, _offset: i64, _limit: i64) -> Result<Vec<AdminUserRecord>> { todo!() }
pub async fn suspend_user(_pool: &PgPool, _user_id: i64, _duration: &str, _reason: &str, _message: Option<&str>, _acting_user_id: i64) -> Result<SuspendResult> { todo!() }
pub async fn unsuspend_user(_pool: &PgPool, _user_id: i64) -> Result<()> { todo!() }
pub async fn silence_user(_pool: &PgPool, _user_id: i64, _duration: &str, _reason: &str, _message: Option<&str>, _acting_user_id: i64) -> Result<SilenceResult> { todo!() }
pub async fn grant_admin(_pool: &PgPool, _user_id: i64) -> Result<()> { todo!() }
pub async fn revoke_admin(_pool: &PgPool, _user_id: i64) -> Result<()> { todo!() }
