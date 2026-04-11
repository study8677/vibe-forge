use crate::registry::PluginRegistry;

pub struct PluginHealthChecker;

impl PluginHealthChecker {
    pub async fn check_all(registry: &PluginRegistry) -> Vec<(String, bool)> {
        let plugins = registry.list_plugins();
        let mut results = Vec::new();
        for name in plugins {
            results.push((name, true)); // TODO: call health_check on each plugin
        }
        results
    }
}
