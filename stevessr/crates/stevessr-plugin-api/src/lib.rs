pub mod manifest;
pub mod lifecycle;
pub mod hooks;
pub mod events;
pub mod extension;
pub mod context;
pub mod store;
pub mod permissions;
pub mod config_schema;
pub mod version;
pub mod ffi;
pub mod error;

pub use error::PluginError;
pub use lifecycle::Plugin;
pub use manifest::PluginManifest;
pub use context::PluginContext;
pub use version::ApiVersion;
