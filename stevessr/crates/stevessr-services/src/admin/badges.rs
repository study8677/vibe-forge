use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_all(_pool: &PgPool) -> Result<Vec<Value>> { todo!() }
pub async fn create_badge(_pool: &PgPool, _name: &str, _badge_type_id: i32, _description: Option<&str>, _icon: Option<&str>, _query: Option<&str>) -> Result<Value> { todo!() }
