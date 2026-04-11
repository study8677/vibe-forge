/// WASM sandbox configuration and resource limits.
pub struct SandboxConfig {
    pub fuel_limit: u64,
    pub memory_limit_bytes: usize,
    pub max_execution_time_ms: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            fuel_limit: 1_000_000,
            memory_limit_bytes: 64 * 1024 * 1024, // 64MB
            max_execution_time_ms: 5000,
        }
    }
}
