pub struct PluginHotReloader;

impl PluginHotReloader {
    pub fn new() -> Self { Self }

    pub async fn watch(&self, _plugin_dir: &std::path::Path) {
        // TODO: use notify crate to watch plugin directory for changes
        // On change: unload old plugin, load new plugin
        tracing::info!("plugin hot reload watcher started");
    }
}

impl Default for PluginHotReloader {
    fn default() -> Self { Self::new() }
}
