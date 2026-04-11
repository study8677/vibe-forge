use async_trait::async_trait;
use crate::traits::Job;

pub struct GrantTrustLevel;

#[async_trait]
impl Job for GrantTrustLevel {
    fn name(&self) -> &str { "grant_trust_level" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
