use async_trait::async_trait;
use std::collections::HashMap;
use parking_lot::RwLock;
use stevessr_plugin_api::context::{InterPluginMessenger, InterPluginHandler};
use stevessr_plugin_api::error::PluginError;
use std::sync::Arc;

pub struct InterPluginBus {
    handlers: RwLock<HashMap<String, Arc<dyn InterPluginHandler>>>,
}

impl InterPluginBus {
    pub fn new() -> Self {
        Self { handlers: RwLock::new(HashMap::new()) }
    }
}

impl Default for InterPluginBus {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl InterPluginMessenger for InterPluginBus {
    async fn send(&self, target_plugin: &str, message: serde_json::Value) -> Result<serde_json::Value, PluginError> {
        let handler = {
            let handlers = self.handlers.read();
            handlers.get(target_plugin)
                .ok_or_else(|| PluginError::Internal(format!("plugin '{}' has no message handler", target_plugin)))?
                .clone()
        };
        Ok(handler.handle("unknown", message).await)
    }

    fn on_message(&self, _handler: Box<dyn InterPluginHandler>) {
        // Registration happens during plugin init
    }
}
