use axum::{extract::{State, Path}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateBookmarkRequest {
    pub bookmarkable_id: i64,
    pub bookmarkable_type: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub reminder_at: Option<String>,
    #[serde(default)]
    pub auto_delete_preference: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookmarkRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub reminder_at: Option<String>,
    #[serde(default)]
    pub auto_delete_preference: Option<i32>,
}

/// POST /bookmarks - Create a new bookmark
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateBookmarkRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let bookmark = stevessr_services::bookmarks::create_bookmark(
        &state.db,
        user.0.id,
        params.bookmarkable_id,
        &params.bookmarkable_type,
        params.name.as_deref(),
        params.reminder_at.as_deref(),
        params.auto_delete_preference.unwrap_or(0),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": bookmark.id,
            "success": "OK",
        })),
    ))
}

/// PUT /bookmarks/{id} - Update a bookmark
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateBookmarkRequest>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::bookmarks::update_bookmark(
        &state.db,
        id,
        user.0.id,
        params.name.as_deref(),
        params.reminder_at.as_deref(),
        params.auto_delete_preference,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// DELETE /bookmarks/{id} - Delete a bookmark
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::bookmarks::delete_bookmark(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(StatusCode::OK)
}
