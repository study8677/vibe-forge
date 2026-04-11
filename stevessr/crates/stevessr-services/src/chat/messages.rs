use sqlx::PgPool;
use stevessr_core::error::Result;
use serde_json::Value;

pub async fn list_messages(_pool: &PgPool, _channel_id: i64, _user_id: i64, _page_size: i64, _target_message_id: Option<i64>, _direction: Option<&str>) -> Result<Vec<Value>> { todo!() }
pub async fn create_message(_pool: &PgPool, _channel_id: i64, _user_id: i64, _content: &str, _in_reply_to_id: Option<i64>, _thread_id: Option<i64>, _upload_ids: Option<&[i64]>) -> Result<Value> { todo!() }
pub async fn update_message(_pool: &PgPool, _message_id: i64, _user_id: i64, _content: &str, _upload_ids: Option<&[i64]>) -> Result<Value> { todo!() }
pub async fn delete_message(_pool: &PgPool, _message_id: i64, _user_id: i64) -> Result<()> { todo!() }
pub async fn update_last_read(_pool: &PgPool, _channel_id: i64, _message_id: i64, _user_id: i64) -> Result<()> { todo!() }
