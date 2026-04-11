use async_trait::async_trait;
use crate::traits::Job;

pub struct Cleanup;

#[async_trait]
impl Job for Cleanup {
    fn name(&self) -> &str { "cleanup" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
