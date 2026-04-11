use async_trait::async_trait;
use crate::traits::Job;

pub struct EmitWebhook;

#[async_trait]
impl Job for EmitWebhook {
    fn name(&self) -> &str { "emit_webhook" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
