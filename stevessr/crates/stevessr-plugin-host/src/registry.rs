use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use stevessr_plugin_api::{Plugin, PluginManifest};
use stevessr_plugin_api::manifest::PluginRuntime;
use stevessr_plugin_api::error::PluginError;
use crate::native_runtime::NativePluginLoader;
use crate::wasm_runtime::WasmPluginLoader;

pub struct PluginRegistry {
    plugins: RwLock<HashMap<String, LoadedPlugin>>,
    native_loader: NativePluginLoader,
    wasm_loader: WasmPluginLoader,
}

pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub instance: Box<dyn Plugin>,
    pub enabled: bool,
    pub healthy: bool,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            native_loader: NativePluginLoader::new(),
            wasm_loader: WasmPluginLoader::new(1_000_000),
        }
    }

    pub async fn discover_and_load(&self, plugin_dir: &Path) -> Result<Vec<String>, PluginError> {
        let mut loaded = Vec::new();

        if !plugin_dir.exists() {
            return Ok(loaded);
        }

        let entries = std::fs::read_dir(plugin_dir)
            .map_err(|e| PluginError::Internal(format!("failed to read plugin directory: {}", e)))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("Plugin.toml");
                if manifest_path.exists() {
                    match self.load_plugin(&path).await {
                        Ok(name) => loaded.push(name),
                        Err(e) => tracing::error!("Failed to load plugin at {:?}: {}", path, e),
                    }
                }
            }
        }

        Ok(loaded)
    }

    async fn load_plugin(&self, plugin_dir: &Path) -> Result<String, PluginError> {
        let manifest_content = std::fs::read_to_string(plugin_dir.join("Plugin.toml"))
            .map_err(|e| PluginError::Internal(format!("failed to read Plugin.toml: {}", e)))?;

        let manifest = PluginManifest::from_toml(&manifest_content)
            .map_err(|e| PluginError::ConfigError(format!("invalid Plugin.toml: {}", e)))?;

        let name = manifest.plugin.name.clone();

        let instance: Box<dyn Plugin> = match manifest.plugin.runtime {
            PluginRuntime::Native => {
                self.native_loader.load(plugin_dir, &manifest)?
            }
            PluginRuntime::Wasm => {
                self.wasm_loader.load(plugin_dir, &manifest)?
            }
        };

        self.plugins.write().insert(name.clone(), LoadedPlugin {
            manifest,
            instance,
            enabled: true,
            healthy: true,
        });

        Ok(name)
    }

    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.read().keys().cloned().collect()
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        self.plugins.read().get(name).map(|p| p.enabled).unwrap_or(false)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
