use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;

#[derive(Debug, Deserialize)]
pub struct ReviewParams {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub review_type: Option<String>,
    #[serde(default)]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub topic_id: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub from_date: Option<String>,
    #[serde(default)]
    pub to_date: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub min_score: Option<f64>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// GET /review - List reviewable items (flags, queued posts, etc.)
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<ReviewParams>,
) -> Result<Json<Value>, StatusCode> {
    // Only staff can access the review queue
    if !user.0.admin && !user.0.moderator {
        return Err(StatusCode::FORBIDDEN);
    }

    let items = stevessr_services::review::list_reviewables(
        &state.db,
        params.status.as_deref(),
        params.review_type.as_deref(),
        params.category_id,
        params.topic_id,
        params.username.as_deref(),
        params.sort_order.as_deref(),
        params.pagination.offset(),
        params.pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reviewable_list: Vec<Value> = items
        .iter()
        .map(|r| {
            json!({
                "id": r.id,
                "type": r.reviewable_type,
                "status": r.status,
                "score": r.score,
                "created_at": r.created_at,
                "category_id": r.category_id,
                "topic_id": r.topic_id,
                "target_id": r.target_id,
                "target_type": r.target_type,
                "created_by": r.created_by,
                "target_created_by": r.target_created_by,
                "payload": r.payload,
                "version": r.version,
                "reviewable_scores": r.reviewable_scores,
            })
        })
        .collect();

    Ok(Json(json!({
        "reviewables": reviewable_list,
        "meta": {
            "total_rows_reviewables": items.len(),
            "types": [
                {"id": "ReviewableFlaggedPost", "name": "Flagged Post"},
                {"id": "ReviewableQueuedPost", "name": "Queued Post"},
                {"id": "ReviewableUser", "name": "User"},
            ],
            "statuses": [
                {"id": "pending", "name": "Pending"},
                {"id": "approved", "name": "Approved"},
                {"id": "rejected", "name": "Rejected"},
                {"id": "ignored", "name": "Ignored"},
                {"id": "deleted", "name": "Deleted"},
            ],
        }
    })))
}

/// GET /review/{id} - Show a single reviewable item
pub async fn show(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin && !user.0.moderator {
        return Err(StatusCode::FORBIDDEN);
    }

    let item = stevessr_services::review::find_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "reviewable": {
            "id": item.id,
            "type": item.reviewable_type,
            "status": item.status,
            "score": item.score,
            "created_at": item.created_at,
            "category_id": item.category_id,
            "topic_id": item.topic_id,
            "target_id": item.target_id,
            "target_type": item.target_type,
            "created_by": item.created_by,
            "target_created_by": item.target_created_by,
            "payload": item.payload,
            "version": item.version,
            "reviewable_scores": item.reviewable_scores,
            "actions": item.actions,
            "editable_fields": item.editable_fields,
        }
    })))
}

/// PUT /review/{id}/perform/{action} - Perform action on reviewable
/// Actions: approve, reject, ignore, delete, agree, disagree
pub async fn perform_action(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((id, action)): Path<(i64, String)>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin && !user.0.moderator {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = stevessr_services::review::perform_action(
        &state.db,
        id,
        &action,
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "reviewable_perform_result": {
            "success": "OK",
            "transition_to": result.transition_to,
            "reviewable": result.reviewable,
        }
    })))
}
