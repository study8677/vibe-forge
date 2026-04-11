use axum::{extract::{State, Path}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreatePostActionRequest {
    pub id: i64,
    pub post_action_type_id: i64,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub is_warning: Option<bool>,
    #[serde(default)]
    pub take_action: Option<bool>,
    #[serde(default)]
    pub flag_topic: Option<bool>,
}

/// POST /post_actions - Create a post action (like, flag, bookmark, etc.)
///
/// Action type IDs:
/// 1 = bookmark, 2 = like, 3 = off_topic, 4 = inappropriate,
/// 6 = notify_user, 7 = notify_moderators, 8 = spam
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreatePostActionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let action = stevessr_services::post_actions::create_action(
        &state.db,
        user.0.id,
        params.id,
        params.post_action_type_id,
        params.message.as_deref(),
        params.is_warning.unwrap_or(false),
        params.take_action.unwrap_or(false),
        params.flag_topic.unwrap_or(false),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "id": action.id,
        "count": action.count,
        "acted": true,
        "can_undo": action.can_undo,
    })))
}

/// DELETE /post_actions/{id} - Remove a post action (unlike, unflag, etc.)
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::post_actions::delete_action(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}
