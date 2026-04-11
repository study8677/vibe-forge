use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateDraftRequest {
    pub draft_key: String,
    pub data: String,
    #[serde(default)]
    pub sequence: Option<i64>,
    #[serde(default)]
    pub owner: Option<String>,
}

/// GET /drafts - List drafts for current user
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let user_drafts = stevessr_services::drafts::for_user(
        &state.db,
        user.0.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let draft_list: Vec<Value> = user_drafts
        .iter()
        .map(|d| {
            json!({
                "id": d.id,
                "draft_key": d.draft_key,
                "sequence": d.sequence,
                "data": d.data,
                "created_at": d.created_at,
                "updated_at": d.updated_at,
                "excerpt": d.excerpt,
                "title": d.title,
                "category_id": d.category_id,
                "archetype": d.archetype,
            })
        })
        .collect();

    Ok(Json(json!({
        "drafts": draft_list,
    })))
}

/// POST /drafts - Create or update a draft
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateDraftRequest>,
) -> Result<Json<Value>, StatusCode> {
    let draft = stevessr_services::drafts::upsert_draft(
        &state.db,
        user.0.id,
        &params.draft_key,
        &params.data,
        params.sequence.unwrap_or(0),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "draft_sequence": draft.sequence,
    })))
}

/// DELETE /drafts/{id} - Delete a draft
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::drafts::delete_draft(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(StatusCode::OK)
}
