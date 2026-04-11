//! FFI boundary types for native plugins.
//! These types are used across the dynamic library boundary.

/// Symbol names that native plugins must export.
pub const PLUGIN_CREATE_SYMBOL: &[u8] = b"_stevessr_plugin_create";
pub const PLUGIN_API_VERSION_SYMBOL: &[u8] = b"_stevessr_plugin_api_version";

/// Type signatures for the exported functions.
pub type PluginCreateFn = unsafe extern "C" fn() -> *mut dyn crate::Plugin;
pub type PluginApiVersionFn = unsafe extern "C" fn() -> (u32, u32);
