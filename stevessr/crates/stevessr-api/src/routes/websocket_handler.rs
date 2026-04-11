use axum::{
    extract::{State, Query, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::Response,
    Json, http::StatusCode,
};
use serde::Deserialize;
use serde_json::{json, Value};
use futures::{StreamExt, SinkExt};
use crate::state::AppState;
use crate::extractors::current_user::OptionalUser;

#[derive(Debug, Deserialize)]
pub struct MessageBusPollParams {
    #[serde(default)]
    pub dlp: Option<i64>,
    #[serde(flatten)]
    pub channels: std::collections::HashMap<String, i64>,
}

/// GET /ws - WebSocket upgrade for real-time messaging
pub async fn ws_upgrade(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    ws: WebSocketUpgrade,
) -> Response {
    let user_id = current.map(|u| u.id);

    ws.on_upgrade(move |socket| handle_websocket(socket, state, user_id))
}

async fn handle_websocket(socket: WebSocket, state: AppState, user_id: Option<i64>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to Redis pub/sub for real-time updates
    let mut redis_conn = state.redis.clone();

    // Determine channels to subscribe based on user
    let channels = match user_id {
        Some(uid) => vec![
            "/latest".to_string(),
            format!("/user/{}", uid),
            format!("/user-notifications/{}", uid),
        ],
        None => vec!["/latest".to_string()],
    };

    // Spawn task to forward Redis messages to WebSocket
    let redis_conn_clone = redis_conn.clone();
    let sender_handle = tokio::spawn(async move {
        // Subscribe to channels and forward messages
        let mut pubsub = stevessr_services::pubsub::subscribe(&channels)
            .await
            .expect("Failed to subscribe to channels");

        while let Some(msg) = pubsub.next().await {
            let payload = json!({
                "channel": msg.channel,
                "data": msg.data,
                "message_id": msg.message_id,
            });

            if sender
                .send(Message::Text(payload.to_string().into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Handle incoming WebSocket messages
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                // Handle client messages (heartbeat, subscribe, unsubscribe)
                if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                    if parsed.get("type").and_then(|v| v.as_str()) == Some("heartbeat") {
                        // Client heartbeat - update last seen
                        if let Some(uid) = user_id {
                            let _ = stevessr_services::users::update_last_seen(
                                &state.db,
                                uid,
                            )
                            .await;
                        }
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Clean up
    sender_handle.abort();
}

/// GET /message-bus/poll - Long-polling fallback for message bus
pub async fn message_bus_poll(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Query(params): Query<MessageBusPollParams>,
) -> Result<Json<Value>, StatusCode> {
    let user_id = current.map(|u| u.id);
    let dlp = params.dlp.unwrap_or(-1);

    let messages = stevessr_services::pubsub::poll(
        &state.db,
        user_id,
        &params.channels,
        dlp,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let message_list: Vec<Value> = messages
        .iter()
        .map(|m| {
            json!({
                "global_id": m.global_id,
                "message_id": m.message_id,
                "channel": m.channel,
                "data": m.data,
            })
        })
        .collect();

    Ok(Json(json!(message_list)))
}
