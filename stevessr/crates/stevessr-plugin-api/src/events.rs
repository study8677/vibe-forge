use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Marker trait for plugin events.
pub trait Event: Send + Sync + Serialize + DeserializeOwned + 'static {
    fn event_name() -> &'static str;
}

/// Subscription identifier for later removal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(pub u64);

/// Handler for events.
#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle(&self, event: E);
}

/// The event bus trait. Plugins use this for fire-and-forget async events.
#[async_trait]
pub trait EventBus: Send + Sync {
    /// Publish an event. All subscribers receive it asynchronously.
    async fn publish_raw(&self, event_name: &str, payload: serde_json::Value);

    /// Subscribe to events of a given name.
    fn subscribe_raw(
        &self,
        event_name: &str,
        handler: Box<dyn RawEventHandler>,
    ) -> SubscriptionId;

    /// Unsubscribe from events.
    fn unsubscribe(&self, id: SubscriptionId);
}

/// Type-erased event handler for the bus.
#[async_trait]
pub trait RawEventHandler: Send + Sync {
    async fn handle(&self, payload: serde_json::Value);
}
