use std::path::Path;
use stevessr_plugin_api::{Plugin, PluginManifest};
use stevessr_plugin_api::ffi::{PluginCreateFn, PluginApiVersionFn, PLUGIN_CREATE_SYMBOL, PLUGIN_API_VERSION_SYMBOL};
use stevessr_plugin_api::error::PluginError;
use stevessr_plugin_api::version::ApiVersion;

pub struct NativePluginLoader;

impl NativePluginLoader {
    pub fn new() -> Self { Self }

    pub fn load(&self, plugin_dir: &Path, manifest: &PluginManifest) -> Result<Box<dyn Plugin>, PluginError> {
        let lib_name = format!("lib{}", manifest.plugin.name.replace('-', "_"));
        let lib_path = plugin_dir
            .join("target")
            .join("release")
            .join(format!("{}.dylib", lib_name));

        if !lib_path.exists() {
            return Err(PluginError::Internal(format!("native library not found: {:?}", lib_path)));
        }

        unsafe {
            let lib = libloading::Library::new(&lib_path)
                .map_err(|e| PluginError::Internal(format!("failed to load library: {}", e)))?;

            // Check API version
            let version_fn: libloading::Symbol<PluginApiVersionFn> = lib
                .get(PLUGIN_API_VERSION_SYMBOL)
                .map_err(|e| PluginError::Internal(format!("missing API version symbol: {}", e)))?;

            let (major, minor) = version_fn();
            let plugin_version = ApiVersion { major, minor };
            if !ApiVersion::CURRENT.is_compatible(&plugin_version) {
                return Err(PluginError::ApiVersionIncompatible {
                    required: plugin_version.to_string(),
                    available: ApiVersion::CURRENT.to_string(),
                });
            }

            // Create plugin instance
            let create_fn: libloading::Symbol<PluginCreateFn> = lib
                .get(PLUGIN_CREATE_SYMBOL)
                .map_err(|e| PluginError::Internal(format!("missing create symbol: {}", e)))?;

            let raw = create_fn();
            let plugin = Box::from_raw(raw);

            // Leak the library so it stays loaded
            std::mem::forget(lib);

            Ok(plugin)
        }
    }
}
