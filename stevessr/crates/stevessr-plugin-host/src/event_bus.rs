use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use stevessr_plugin_api::events::*;

pub struct AsyncEventBus {
    subscribers: RwLock<HashMap<String, Vec<(SubscriptionId, Arc<dyn RawEventHandler>)>>>,
    next_id: std::sync::atomic::AtomicU64,
}

impl AsyncEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(HashMap::new()),
            next_id: std::sync::atomic::AtomicU64::new(1),
        }
    }
}

impl Default for AsyncEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl EventBus for AsyncEventBus {
    async fn publish_raw(&self, event_name: &str, payload: serde_json::Value) {
        let subs = self.subscribers.read();
        if let Some(handlers) = subs.get(event_name) {
            for (_, handler) in handlers {
                let h = handler.clone();
                let p = payload.clone();
                tokio::spawn(async move { h.handle(p).await });
            }
        }
    }

    fn subscribe_raw(&self, event_name: &str, handler: Box<dyn RawEventHandler>) -> SubscriptionId {
        let id = SubscriptionId(self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        let mut subs = self.subscribers.write();
        subs.entry(event_name.to_string()).or_default().push((id, Arc::from(handler)));
        id
    }

    fn unsubscribe(&self, id: SubscriptionId) {
        let mut subs = self.subscribers.write();
        for handlers in subs.values_mut() {
            handlers.retain(|(sub_id, _)| *sub_id != id);
        }
    }
}
