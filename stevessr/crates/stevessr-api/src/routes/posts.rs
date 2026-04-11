use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;
use crate::serializers::post::serialize_post;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub raw: String,
    pub topic_id: Option<i64>,
    #[serde(default)]
    pub reply_to_post_number: Option<i64>,
    #[serde(default)]
    pub category: Option<i64>,
    #[serde(default)]
    pub archetype: Option<String>,
    #[serde(default)]
    pub target_recipients: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub whisper: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub raw: String,
    #[serde(default)]
    pub edit_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WikiToggleRequest {
    pub wiki: bool,
}

#[derive(Debug, Deserialize)]
pub struct LockedToggleRequest {
    pub locked: bool,
}

/// POST /posts - Create a new post (reply or new topic)
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreatePostRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    // If no topic_id and a title is provided, create a new topic + post
    // Otherwise, reply to existing topic
    let post = stevessr_services::posts::create_post(
        &state.db,
        user.0.id,
        &params.raw,
        params.topic_id,
        params.reply_to_post_number,
        params.category,
        params.title.as_deref(),
        params.tags.as_deref(),
        params.archetype.as_deref(),
        params.target_recipients.as_deref(),
        params.whisper.unwrap_or(false),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(serialize_post(&post)),
    ))
}

/// GET /posts/{id} - Show a single post
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    let post = stevessr_services::posts::find_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serialize_post(&post)))
}

/// PUT /posts/{id} - Update a post
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdatePostRequest>,
) -> Result<Json<Value>, StatusCode> {
    let post = stevessr_services::posts::update_post(
        &state.db,
        id,
        user.0.id,
        &params.raw,
        params.edit_reason.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "post": serialize_post(&post),
    })))
}

/// DELETE /posts/{id} - Delete a post
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::posts::delete_post(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(StatusCode::OK)
}

/// GET /posts/{id}/revisions/{revision} - Show a specific revision of a post
pub async fn show_revision(
    State(state): State<AppState>,
    Path((id, revision)): Path<(i64, i64)>,
) -> Result<Json<Value>, StatusCode> {
    let rev = stevessr_services::posts::get_revision(&state.db, id, revision)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "post_revision": {
            "post_id": id,
            "current_revision": rev.current_revision,
            "first_revision": rev.first_revision,
            "previous_revision": rev.previous_revision,
            "next_revision": rev.next_revision,
            "last_revision": rev.last_revision,
            "current_version": rev.current_version,
            "version_count": rev.version_count,
            "username": rev.username,
            "display_username": rev.display_username,
            "avatar_template": rev.avatar_template,
            "created_at": rev.created_at,
            "edit_reason": rev.edit_reason,
            "body_changes": {
                "inline": rev.body_changes_inline,
                "side_by_side": rev.body_changes_side_by_side,
            },
            "title_changes": rev.title_changes,
            "can_edit": rev.can_edit,
        }
    })))
}

/// PUT /posts/{id}/wiki - Toggle wiki mode on a post
pub async fn toggle_wiki(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<WikiToggleRequest>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::posts::set_wiki(&state.db, id, user.0.id, params.wiki)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// PUT /posts/{id}/locked - Toggle locked state on a post
pub async fn toggle_locked(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<LockedToggleRequest>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::posts::set_locked(&state.db, id, user.0.id, params.locked)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// GET /posts/{id}/replies - Get replies to a post
pub async fn replies(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    let reply_list = stevessr_services::posts::get_replies(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let serialized: Vec<Value> = reply_list.iter().map(|p| serialize_post(p)).collect();

    Ok(Json(json!(serialized)))
}
