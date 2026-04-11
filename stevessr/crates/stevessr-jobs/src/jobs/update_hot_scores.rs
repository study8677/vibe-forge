use async_trait::async_trait;
use crate::traits::Job;

pub struct UpdateHotScores;

#[async_trait]
impl Job for UpdateHotScores {
    fn name(&self) -> &str { "update_hot_scores" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
