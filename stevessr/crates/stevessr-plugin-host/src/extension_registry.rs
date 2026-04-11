use stevessr_plugin_api::extension::*;
use std::collections::HashMap;
use parking_lot::RwLock;

pub struct PluginExtensionRegistry {
    // Store all registered extensions
    pub serializer_fields: RwLock<HashMap<SerializerTarget, Vec<(String, Box<dyn SerializerFieldExtractor>)>>>,
    pub site_settings: RwLock<Vec<SiteSettingDefinition>>,
    pub notification_types: RwLock<HashMap<String, i32>>,
    pub custom_fields: RwLock<Vec<CustomFieldDefinition>>,
}

impl PluginExtensionRegistry {
    pub fn new() -> Self {
        Self {
            serializer_fields: RwLock::new(HashMap::new()),
            site_settings: RwLock::new(Vec::new()),
            notification_types: RwLock::new(HashMap::new()),
            custom_fields: RwLock::new(Vec::new()),
        }
    }
}

impl Default for PluginExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
