use async_trait::async_trait;
use crate::traits::Job;

pub struct DirectoryRefresh;

#[async_trait]
impl Job for DirectoryRefresh {
    fn name(&self) -> &str { "directory_refresh" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
