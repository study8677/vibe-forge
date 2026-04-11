use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_backups(_pool: &PgPool) -> Result<Vec<Value>> { todo!() }
pub async fn start_backup(_pool: &PgPool, _with_uploads: bool) -> Result<Value> { todo!() }
