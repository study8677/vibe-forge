use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
    pub uploads: UploadConfig,
    pub email: EmailConfig,
    pub search: SearchConfig,
    pub plugins: PluginConfig,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub bind: String,
    #[serde(default)]
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_secs: u64,
}

fn default_max_connections() -> u32 { 20 }
fn default_min_connections() -> u32 { 5 }
fn default_connect_timeout() -> u64 { 10 }
fn default_idle_timeout() -> u64 { 300 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
    #[serde(default = "default_pool_size")]
    pub pool_size: usize,
}

fn default_pool_size() -> usize { 10 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    #[serde(default = "default_token_length")]
    pub session_token_length: usize,
    #[serde(default = "default_session_ttl")]
    pub session_ttl_days: u64,
    #[serde(default = "default_password_min")]
    pub password_min_length: usize,
    #[serde(default = "default_max_login_attempts")]
    pub max_login_attempts: u32,
    #[serde(default = "default_lockout_minutes")]
    pub login_lockout_minutes: u64,
}

fn default_token_length() -> usize { 32 }
fn default_session_ttl() -> u64 { 60 }
fn default_password_min() -> usize { 10 }
fn default_max_login_attempts() -> u32 { 5 }
fn default_lockout_minutes() -> u64 { 10 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadConfig {
    #[serde(default = "default_max_file_size")]
    pub max_file_size_mb: u64,
    #[serde(default = "default_max_image_size")]
    pub max_image_size_mb: u64,
    #[serde(default)]
    pub allowed_extensions: Vec<String>,
    #[serde(default = "default_storage")]
    pub storage: String,
    #[serde(default = "default_local_path")]
    pub local_path: String,
    #[serde(default)]
    pub s3: Option<S3Config>,
}

fn default_max_file_size() -> u64 { 10 }
fn default_max_image_size() -> u64 { 5 }
fn default_storage() -> String { "local".into() }
fn default_local_path() -> String { "./uploads".into() }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    #[serde(default)]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_from")]
    pub from: String,
    #[serde(default)]
    pub smtp: Option<SmtpConfig>,
}

fn default_from() -> String { "noreply@example.com".into() }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmtpConfig {
    pub host: String,
    #[serde(default = "default_smtp_port")]
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub tls: bool,
}

fn default_smtp_port() -> u16 { 25 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchConfig {
    #[serde(default = "default_index_path")]
    pub index_path: String,
    #[serde(default = "default_writer_memory")]
    pub writer_memory_mb: usize,
}

fn default_index_path() -> String { "./data/search_index".into() }
fn default_writer_memory() -> usize { 50 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_plugin_dir")]
    pub directory: String,
    #[serde(default = "default_fuel_limit")]
    pub wasm_fuel_limit: u64,
    #[serde(default)]
    pub hot_reload: bool,
}

fn default_true() -> bool { true }
fn default_plugin_dir() -> String { "./plugins".into() }
fn default_fuel_limit() -> u64 { 1_000_000 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitConfig {
    #[serde(default = "default_rps")]
    pub requests_per_second: u32,
    #[serde(default = "default_burst")]
    pub burst_size: u32,
}

fn default_rps() -> u32 { 10 }
fn default_burst() -> u32 { 30 }
