use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_webhooks(_pool: &PgPool) -> Result<Vec<Value>> { todo!() }
pub async fn create_webhook(_pool: &PgPool, _payload_url: &str, _secret: Option<&str>, _wildcard: bool, _event_types: &[String]) -> Result<Value> { todo!() }
