//! WebSocket hub for real-time messaging.
//!
//! This module manages WebSocket connections and broadcasts messages
//! from the message bus (backed by Redis pub/sub) to connected clients.
//!
//! # Architecture
//!
//! ```text
//! Client <--WebSocket--> WsConnection --> Hub --> Redis PubSub
//!                                          |
//!                                          +--> Broadcast to subscribers
//! ```
//!
//! Each connected client is represented by a `WsConnection`. The `Hub`
//! maintains a set of active connections and routes messages from Redis
//! pub/sub channels to the appropriate WebSocket clients.
//!
//! # Channels
//!
//! The message bus uses the following channel patterns:
//!
//! - `/latest` - New and updated topics
//! - `/new` - New topics only
//! - `/unread` - Unread topic updates (per-user)
//! - `/unread/{user_id}` - User-specific unread
//! - `/user-notifications/{user_id}` - Notification updates
//! - `/topic/{topic_id}` - Topic-specific updates (new posts, edits)
//! - `/chat/channel/{channel_id}` - Chat channel messages
//! - `/presence/channel/{channel_id}` - Presence (who is typing)
//! - `/categories` - Category updates
//! - `/site/banner` - Site-wide banner changes

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// A message routed through the hub to WebSocket clients.
#[derive(Clone, Debug)]
pub struct BusMessage {
    pub channel: String,
    pub data: serde_json::Value,
    pub message_id: i64,
    pub global_id: i64,
}

/// Manages all active WebSocket connections and channel subscriptions.
pub struct Hub {
    /// Map of channel name to broadcast sender.
    channels: RwLock<HashMap<String, broadcast::Sender<BusMessage>>>,
    /// Capacity of each channel's broadcast buffer.
    channel_capacity: usize,
}

impl Hub {
    pub fn new(channel_capacity: usize) -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            channel_capacity,
        }
    }

    /// Subscribe to a channel, creating it if it doesn't exist.
    pub async fn subscribe(&self, channel: &str) -> broadcast::Receiver<BusMessage> {
        let channels = self.channels.read().await;
        if let Some(tx) = channels.get(channel) {
            return tx.subscribe();
        }
        drop(channels);

        let mut channels = self.channels.write().await;
        // Double-check after acquiring write lock
        if let Some(tx) = channels.get(channel) {
            return tx.subscribe();
        }

        let (tx, rx) = broadcast::channel(self.channel_capacity);
        channels.insert(channel.to_string(), tx);
        rx
    }

    /// Publish a message to a channel.
    pub async fn publish(&self, message: BusMessage) {
        let channels = self.channels.read().await;
        if let Some(tx) = channels.get(&message.channel) {
            // Ignore errors (no active receivers)
            let _ = tx.send(message);
        }
    }

    /// Remove channels with no active subscribers to free memory.
    pub async fn cleanup_empty_channels(&self) {
        let mut channels = self.channels.write().await;
        channels.retain(|_, tx| tx.receiver_count() > 0);
    }
}

/// Create a shared Hub instance wrapped in Arc for use across the application.
pub fn create_hub(channel_capacity: usize) -> Arc<Hub> {
    Arc::new(Hub::new(channel_capacity))
}
