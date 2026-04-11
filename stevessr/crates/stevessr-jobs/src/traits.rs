use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Job: Send + Sync {
    fn name(&self) -> &str;
    fn queue(&self) -> &str { "default" }
    async fn execute(&self, payload: Value) -> Result<(), anyhow::Error>;
    fn max_retries(&self) -> u32 { 3 }
    fn retry_delay_seconds(&self) -> u64 { 60 }
}
