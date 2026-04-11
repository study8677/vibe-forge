use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_for_user(_pool: &PgPool, _user_id: i64, _status: Option<&str>, _offset: i64, _limit: i64) -> Result<Vec<Value>> { todo!() }
pub async fn find_by_id(_pool: &PgPool, _id: i64, _user_id: i64) -> Result<Value> { todo!() }
pub async fn create_channel(_pool: &PgPool, _name: &str, _description: Option<&str>, _chatable_type: &str, _chatable_id: i64, _user_id: i64) -> Result<Value> { todo!() }
pub async fn update_channel(_pool: &PgPool, _id: i64, _name: Option<&str>, _description: Option<&str>, _user_id: i64) -> Result<Value> { todo!() }
