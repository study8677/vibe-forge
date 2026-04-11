use serde::{Deserialize, Serialize};

/// Plugin permission declaration, checked at load time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPermissionSet {
    /// Access to plugin key-value store.
    pub database: bool,
    /// Ability to make outbound HTTP requests.
    pub http_outbound: bool,
    /// File system access (only enforced in WASM sandbox).
    pub filesystem: bool,
}

impl Default for PluginPermissionSet {
    fn default() -> Self {
        Self {
            database: false,
            http_outbound: false,
            filesystem: false,
        }
    }
}

/// Check if a plugin's requested permissions are within the allowed set.
pub fn check_permissions(requested: &PluginPermissionSet, allowed: &PluginPermissionSet) -> Result<(), Vec<String>> {
    let mut denied = Vec::new();
    if requested.database && !allowed.database {
        denied.push("database".to_string());
    }
    if requested.http_outbound && !allowed.http_outbound {
        denied.push("http_outbound".to_string());
    }
    if requested.filesystem && !allowed.filesystem {
        denied.push("filesystem".to_string());
    }
    if denied.is_empty() { Ok(()) } else { Err(denied) }
}
