use async_trait::async_trait;
use crate::traits::Job;

pub struct SendEmail;

#[async_trait]
impl Job for SendEmail {
    fn name(&self) -> &str { "send_email" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
