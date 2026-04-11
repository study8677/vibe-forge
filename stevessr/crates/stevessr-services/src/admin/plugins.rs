use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_plugins(_pool: &PgPool) -> Result<Vec<Value>> { todo!() }
