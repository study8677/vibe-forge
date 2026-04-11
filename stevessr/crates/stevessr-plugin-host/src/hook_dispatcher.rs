use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use stevessr_plugin_api::hooks::*;
use stevessr_plugin_api::error::PluginError;

pub struct HookDispatcher {
    before_hooks: RwLock<HashMap<HookPoint, Vec<RegisteredHook<Arc<dyn BeforeHook<serde_json::Value>>>>>>,
    after_hooks: RwLock<HashMap<HookPoint, Vec<RegisteredHook<Arc<dyn AfterHook<serde_json::Value, serde_json::Value>>>>>>,
    next_id: std::sync::atomic::AtomicU64,
}

struct RegisteredHook<H> {
    id: HookRegistrationId,
    plugin_name: String,
    handler: H,
    priority: i32,
}

impl HookDispatcher {
    pub fn new() -> Self {
        Self {
            before_hooks: RwLock::new(HashMap::new()),
            after_hooks: RwLock::new(HashMap::new()),
            next_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    pub async fn dispatch_before(&self, point: HookPoint, mut input: serde_json::Value, ctx: &HookContext) -> Result<serde_json::Value, PluginError> {
        let hooks = self.before_hooks.read();
        if let Some(handlers) = hooks.get(&point) {
            for hook in handlers {
                match hook.handler.execute(input, ctx).await {
                    HookOutcome::Continue(v) => input = v,
                    HookOutcome::Halt(e) => return Err(e),
                    HookOutcome::ShortCircuit(v) => return Ok(v),
                }
            }
        }
        Ok(input)
    }

    pub async fn dispatch_after(&self, point: HookPoint, input: &serde_json::Value, mut output: serde_json::Value, ctx: &HookContext) -> serde_json::Value {
        let hooks = self.after_hooks.read();
        if let Some(handlers) = hooks.get(&point) {
            for hook in handlers {
                output = hook.handler.execute(input, output, ctx).await;
            }
        }
        output
    }
}

impl Default for HookDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl HookRegistry for HookDispatcher {
    fn register_before_hook(&self, point: HookPoint, plugin_name: &str, handler: Arc<dyn BeforeHook<serde_json::Value>>) -> HookRegistrationId {
        let id = HookRegistrationId(self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        let priority = handler.priority();
        let mut hooks = self.before_hooks.write();
        let entry = hooks.entry(point).or_default();
        entry.push(RegisteredHook { id, plugin_name: plugin_name.to_string(), handler, priority });
        entry.sort_by_key(|h| h.priority);
        id
    }

    fn register_after_hook(&self, point: HookPoint, plugin_name: &str, handler: Arc<dyn AfterHook<serde_json::Value, serde_json::Value>>) -> HookRegistrationId {
        let id = HookRegistrationId(self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        let priority = handler.priority();
        let mut hooks = self.after_hooks.write();
        let entry = hooks.entry(point).or_default();
        entry.push(RegisteredHook { id, plugin_name: plugin_name.to_string(), handler, priority });
        entry.sort_by_key(|h| h.priority);
        id
    }

    fn unregister_hook(&self, id: HookRegistrationId) {
        let mut before = self.before_hooks.write();
        for hooks in before.values_mut() {
            hooks.retain(|h| h.id != id);
        }
        let mut after = self.after_hooks.write();
        for hooks in after.values_mut() {
            hooks.retain(|h| h.id != id);
        }
    }
}

use stevessr_plugin_api::hooks::HookRegistry;
