use async_trait::async_trait;
use crate::traits::Job;

pub struct BookmarkReminder;

#[async_trait]
impl Job for BookmarkReminder {
    fn name(&self) -> &str { "bookmark_reminder" }
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
