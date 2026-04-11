use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_keys(_pool: &PgPool) -> Result<Vec<Value>> { todo!() }
pub async fn create_key(_pool: &PgPool, _description: &str, _user_id: Option<i64>, _created_by_id: i64) -> Result<Value> { todo!() }
