use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub chatable_type: Option<String>,
    #[serde(default)]
    pub chatable_id: Option<i64>,
    #[serde(default)]
    pub auto_join_users: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelListParams {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// GET /chat/channels - List chat channels
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<ChannelListParams>,
) -> Result<Json<Value>, StatusCode> {
    let channels = stevessr_services::chat::channels::list_for_user(
        &state.db,
        user.0.id,
        params.status.as_deref(),
        params.pagination.offset(),
        params.pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let channel_list: Vec<Value> = channels
        .iter()
        .map(|ch| {
            json!({
                "id": ch.id,
                "chatable_id": ch.chatable_id,
                "chatable_type": ch.chatable_type,
                "chatable_url": ch.chatable_url,
                "title": ch.title,
                "slug": ch.slug,
                "description": ch.description,
                "status": ch.status,
                "last_message_sent_at": ch.last_message_sent_at,
                "current_user_membership": {
                    "following": ch.following,
                    "muted": ch.muted,
                    "desktop_notification_level": ch.desktop_notification_level,
                    "mobile_notification_level": ch.mobile_notification_level,
                    "last_read_message_id": ch.last_read_message_id,
                },
                "meta": {
                    "message_bus_last_ids": ch.message_bus_last_ids,
                },
            })
        })
        .collect();

    Ok(Json(json!({
        "public_channels": channel_list,
        "direct_message_channels": [],
        "meta": {
            "message_bus_last_ids": {},
        },
    })))
}

/// POST /chat/channels - Create a new chat channel
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateChannelRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let channel = stevessr_services::chat::channels::create_channel(
        &state.db,
        &params.name,
        params.description.as_deref(),
        params.chatable_type.as_deref().unwrap_or("Category"),
        params.chatable_id,
        params.auto_join_users.unwrap_or(false),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "channel": {
                "id": channel.id,
                "title": channel.title,
                "slug": channel.slug,
                "description": channel.description,
                "status": "open",
            }
        })),
    ))
}

/// GET /chat/channels/{id} - Show a chat channel
pub async fn show(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    let channel = stevessr_services::chat::channels::find_by_id(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "channel": {
            "id": channel.id,
            "chatable_id": channel.chatable_id,
            "chatable_type": channel.chatable_type,
            "title": channel.title,
            "slug": channel.slug,
            "description": channel.description,
            "status": channel.status,
            "last_message_sent_at": channel.last_message_sent_at,
            "current_user_membership": {
                "following": channel.following,
                "muted": channel.muted,
                "last_read_message_id": channel.last_read_message_id,
            },
        }
    })))
}

/// PUT /chat/channels/{id} - Update a chat channel
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateChannelRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let channel = stevessr_services::chat::channels::update_channel(
        &state.db,
        id,
        params.name.as_deref(),
        params.description.as_deref(),
        params.status.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "channel": {
            "id": channel.id,
            "title": channel.title,
            "slug": channel.slug,
            "description": channel.description,
            "status": channel.status,
        }
    })))
}
