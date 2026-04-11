use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("initialization failed: {0}")]
    InitFailed(String),
    #[error("configuration error: {0}")]
    ConfigError(String),
    #[error("hook execution failed: {0}")]
    HookFailed(String),
    #[error("store error: {0}")]
    StoreError(String),
    #[error("migration error: {0}")]
    MigrationError(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("dependency not found: {0}")]
    DependencyNotFound(String),
    #[error("API version incompatible: required {required}, available {available}")]
    ApiVersionIncompatible { required: String, available: String },
    #[error("WASM execution error: {0}")]
    WasmError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for PluginError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerializationError(e.to_string())
    }
}
