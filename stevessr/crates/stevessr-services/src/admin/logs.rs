use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_staff_action_logs(_pool: &PgPool, _action: Option<&str>, _acting_user: Option<&str>, _target_user: Option<&str>, _offset: i64, _limit: i64) -> Result<Vec<Value>> { todo!() }
pub fn list_action_types() -> Result<Vec<Value>> { todo!() }
