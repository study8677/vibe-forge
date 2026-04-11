use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_threads(_pool: &PgPool, _channel_id: i64, _user_id: i64, _offset: i64, _limit: i64) -> Result<Vec<Value>> { todo!() }
