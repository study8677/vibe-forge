use async_trait::async_trait;
use crate::traits::Job;

pub struct ProcessPost;

#[async_trait]
impl Job for ProcessPost {
    fn name(&self) -> &str { "process_post" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
