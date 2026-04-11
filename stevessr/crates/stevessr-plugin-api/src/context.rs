use std::sync::Arc;
use crate::events::EventBus;
use crate::hooks::HookRegistry;
use crate::extension::ExtensionRegistry;
use crate::store::PluginStore;

/// The context object passed to plugins during lifecycle and hook execution.
/// Provides access to all host services.
pub struct PluginContext {
    pub plugin_name: String,
    pub store: Arc<dyn PluginStore>,
    pub event_bus: Arc<dyn EventBus>,
    pub hook_registry: Arc<dyn HookRegistry>,
    pub extensions: Box<dyn ExtensionRegistry>,
    pub inter_plugin: Arc<dyn InterPluginMessenger>,
    pub logger: PluginLogger,
}

/// Inter-plugin messaging.
#[async_trait::async_trait]
pub trait InterPluginMessenger: Send + Sync {
    /// Send a message to another plugin.
    async fn send(&self, target_plugin: &str, message: serde_json::Value) -> Result<serde_json::Value, crate::PluginError>;

    /// Register a handler for messages from other plugins.
    fn on_message(&self, handler: Box<dyn InterPluginHandler>);
}

#[async_trait::async_trait]
pub trait InterPluginHandler: Send + Sync {
    async fn handle(&self, from_plugin: &str, message: serde_json::Value) -> serde_json::Value;
}

/// Plugin-scoped logger that prefixes all log lines with the plugin name.
#[derive(Clone)]
pub struct PluginLogger {
    pub plugin_name: String,
}

impl PluginLogger {
    pub fn info(&self, msg: &str) {
        tracing::info!(plugin = %self.plugin_name, "{}", msg);
    }

    pub fn warn(&self, msg: &str) {
        tracing::warn!(plugin = %self.plugin_name, "{}", msg);
    }

    pub fn error(&self, msg: &str) {
        tracing::error!(plugin = %self.plugin_name, "{}", msg);
    }

    pub fn debug(&self, msg: &str) {
        tracing::debug!(plugin = %self.plugin_name, "{}", msg);
    }
}
