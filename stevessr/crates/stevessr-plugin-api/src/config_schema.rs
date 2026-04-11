use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Schema for a plugin's configuration, used to auto-generate admin UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfigSchema {
    pub plugin_name: String,
    pub settings: Vec<ConfigField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    pub key: String,
    pub label: String,
    pub description: String,
    pub field_type: ConfigFieldType,
    pub default: Value,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigFieldType {
    Bool,
    String,
    Integer,
    Float,
    Secret,
    Url,
    Enum,
    List,
    GroupList,
    CategoryList,
    TagList,
    Upload,
    Color,
    Textarea,
}
