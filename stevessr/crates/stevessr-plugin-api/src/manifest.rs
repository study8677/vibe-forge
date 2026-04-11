use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin: PluginMeta,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub permissions: PluginPermissions,
    #[serde(default)]
    pub settings: HashMap<String, SettingDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
    #[serde(default = "default_api_version")]
    pub api_version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub repository: String,
    #[serde(default = "default_runtime")]
    pub runtime: PluginRuntime,
}

fn default_api_version() -> String {
    "1".to_string()
}

fn default_runtime() -> PluginRuntime {
    PluginRuntime::Native
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginRuntime {
    Native,
    Wasm,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginPermissions {
    #[serde(default)]
    pub database: bool,
    #[serde(default)]
    pub http_outbound: bool,
    #[serde(default)]
    pub filesystem: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDefinition {
    #[serde(rename = "type")]
    pub setting_type: SettingType,
    pub default: serde_json::Value,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub min: Option<serde_json::Value>,
    #[serde(default)]
    pub max: Option<serde_json::Value>,
    #[serde(default)]
    pub choices: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingType {
    Bool,
    String,
    Integer,
    Float,
    GroupList,
    CategoryList,
    TagList,
    Enum,
    List,
    Secret,
    Url,
    Upload,
}

impl PluginManifest {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}
