use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub message: String,
    #[serde(default)]
    pub in_reply_to_id: Option<i64>,
    #[serde(default)]
    pub thread_id: Option<i64>,
    #[serde(default)]
    pub upload_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageRequest {
    pub message: String,
    #[serde(default)]
    pub upload_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
pub struct MessageListParams {
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub target_message_id: Option<i64>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// GET /chat/channels/{id}/messages - List messages in a channel
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    Query(params): Query<MessageListParams>,
) -> Result<Json<Value>, StatusCode> {
    let page_size = params.page_size.unwrap_or(50);

    let messages = stevessr_services::chat::messages::list_messages(
        &state.db,
        id,
        user.0.id,
        page_size,
        params.target_message_id,
        params.direction.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let message_list: Vec<Value> = messages
        .iter()
        .map(|m| {
            json!({
                "id": m.id,
                "message": m.message,
                "cooked": m.cooked,
                "created_at": m.created_at,
                "edited_at": m.edited_at,
                "deleted_at": m.deleted_at,
                "in_reply_to_id": m.in_reply_to_id,
                "thread_id": m.thread_id,
                "chat_channel_id": m.chat_channel_id,
                "user": {
                    "id": m.user_id,
                    "username": m.username,
                    "name": m.user_name,
                    "avatar_template": m.avatar_template,
                },
                "uploads": m.uploads,
                "reactions": m.reactions,
                "excerpt": m.excerpt,
                "available_flags": m.available_flags,
            })
        })
        .collect();

    // Update last read for this user in this channel
    if let Some(last_msg) = messages.last() {
        let _ = stevessr_services::chat::messages::update_last_read(
            &state.db,
            id,
            user.0.id,
            last_msg.id,
        )
        .await;
    }

    Ok(Json(json!({
        "messages": message_list,
        "meta": {
            "can_flag": true,
            "channel_message_bus_last_id": messages.last().map(|m| m.id).unwrap_or(0),
            "can_moderate": user.0.admin || user.0.moderator,
            "can_delete_self": true,
            "can_delete_others": user.0.admin || user.0.moderator,
        }
    })))
}

/// POST /chat/channels/{id}/messages - Send a message in a channel
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<CreateMessageRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let message = stevessr_services::chat::messages::create_message(
        &state.db,
        id,
        user.0.id,
        &params.message,
        params.in_reply_to_id,
        params.thread_id,
        params.upload_ids.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    // Publish to real-time channel
    let mut redis_conn = state.redis.clone();
    let _ = stevessr_services::pubsub::publish_chat_message(
        &mut redis_conn,
        id,
        &message,
    )
    .await;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": {
                "id": message.id,
                "message": message.message,
                "cooked": message.cooked,
                "created_at": message.created_at,
                "chat_channel_id": id,
                "user": {
                    "id": message.user_id,
                    "username": message.username,
                    "name": message.user_name,
                    "avatar_template": message.avatar_template,
                },
                "in_reply_to_id": message.in_reply_to_id,
                "thread_id": message.thread_id,
                "uploads": message.uploads,
            }
        })),
    ))
}

/// PUT /chat/channels/{id}/messages/{message_id} - Edit a message
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((id, message_id)): Path<(i64, i64)>,
    JsonOrForm(params): JsonOrForm<UpdateMessageRequest>,
) -> Result<Json<Value>, StatusCode> {
    let message = stevessr_services::chat::messages::update_message(
        &state.db,
        message_id,
        user.0.id,
        &params.message,
        params.upload_ids.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "message": {
            "id": message.id,
            "message": message.message,
            "cooked": message.cooked,
            "edited_at": message.edited_at,
        }
    })))
}

/// DELETE /chat/channels/{id}/messages/{message_id} - Delete a message
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((id, message_id)): Path<(i64, i64)>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::chat::messages::delete_message(
        &state.db,
        message_id,
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(StatusCode::OK)
}
