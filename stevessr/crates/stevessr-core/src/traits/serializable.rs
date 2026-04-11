use serde::Serialize;
use serde_json::Value;

/// Trait for types that can be serialized to API JSON responses.
/// Plugins can register additional fields via the extension registry.
pub trait ApiSerializable: Serialize {
    /// The serializer target name (e.g., "topic", "post", "user").
    fn serializer_target() -> &'static str;

    /// Serialize to JSON Value, allowing plugin fields to be merged.
    fn to_api_json(&self, extra_fields: Option<&serde_json::Map<String, Value>>) -> Value {
        let mut value = serde_json::to_value(self).unwrap_or(Value::Null);
        if let (Some(obj), Some(extra)) = (value.as_object_mut(), extra_fields) {
            for (k, v) in extra {
                obj.insert(k.clone(), v.clone());
            }
        }
        value
    }
}
