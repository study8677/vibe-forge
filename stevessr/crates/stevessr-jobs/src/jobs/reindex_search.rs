use async_trait::async_trait;
use crate::traits::Job;

pub struct ReindexSearch;

#[async_trait]
impl Job for ReindexSearch {
    fn name(&self) -> &str { "reindex_search" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
