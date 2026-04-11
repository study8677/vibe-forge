use async_trait::async_trait;
use crate::traits::Job;

pub struct UpdateBadges;

#[async_trait]
impl Job for UpdateBadges {
    fn name(&self) -> &str { "update_badges" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
