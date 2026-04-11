use async_trait::async_trait;
use crate::context::PluginContext;
use crate::error::PluginError;
use crate::manifest::PluginManifest;

/// The core plugin trait. Every plugin (native or WASM) must implement this.
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// Plugin metadata, parsed from Plugin.toml at load time.
    fn manifest(&self) -> &PluginManifest;

    /// Called once when the plugin is first loaded.
    /// Register hooks, extensions, and event listeners here.
    async fn init(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError>;

    /// Called after init, with access to the full configuration.
    /// Register routes, serializer fields, middleware here.
    async fn configure(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError>;

    /// Called when the plugin is activated (enabled by admin).
    async fn activate(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError>;

    /// Called when the plugin is deactivated (disabled by admin).
    async fn deactivate(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError>;

    /// Called when the plugin is being unloaded. Clean up resources.
    async fn destroy(&mut self) -> Result<(), PluginError>;

    /// Health check. Return Ok(()) if the plugin is healthy.
    async fn health_check(&self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// Macro to export the plugin constructor for native plugins (cdylib).
/// Generates the `extern "C" fn _stevessr_plugin_create()` and
/// `extern "C" fn _stevessr_plugin_api_version()` symbols.
///
/// Usage:
/// ```ignore
/// declare_plugin!(MyPlugin, MyPlugin::new);
/// ```
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:expr) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn _stevessr_plugin_create() -> *mut dyn $crate::Plugin {
            let plugin: $plugin_type = $constructor;
            let boxed: Box<dyn $crate::Plugin> = Box::new(plugin);
            Box::into_raw(boxed)
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn _stevessr_plugin_api_version() -> (u32, u32) {
            ($crate::ApiVersion::CURRENT.major, $crate::ApiVersion::CURRENT.minor)
        }
    };
}
