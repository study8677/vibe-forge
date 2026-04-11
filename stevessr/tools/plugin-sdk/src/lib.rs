//! Stevessr Plugin SDK
//!
//! This crate re-exports everything a plugin developer needs.

pub use stevessr_plugin_api::*;
pub use stevessr_plugin_api::lifecycle::Plugin;
pub use stevessr_plugin_api::manifest::PluginManifest;
pub use stevessr_plugin_api::hooks::*;
pub use stevessr_plugin_api::events::*;
pub use stevessr_plugin_api::extension::*;
pub use stevessr_plugin_api::store::*;
pub use stevessr_plugin_api::context::*;
pub use stevessr_plugin_api::error::PluginError;
pub use stevessr_plugin_api::version::ApiVersion;
