use axum::{extract::{State, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct MarkReadRequest {
    #[serde(default)]
    pub id: Option<i64>,
}

/// GET /notifications - List notifications for current user
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let notifications = stevessr_services::notifications::for_user(
        &state.db,
        user.0.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let notification_list: Vec<Value> = notifications
        .iter()
        .map(|n| {
            json!({
                "id": n.id,
                "notification_type": n.notification_type,
                "read": n.read,
                "created_at": n.created_at,
                "post_number": n.post_number,
                "topic_id": n.topic_id,
                "slug": n.slug,
                "data": n.data,
                "fancy_title": n.fancy_title,
            })
        })
        .collect();

    Ok(Json(json!({
        "notifications": notification_list,
        "total_rows_notifications": notifications.len(),
        "seen_notification_id": notifications.first().map(|n| n.id),
        "load_more_notifications": format!("/notifications?offset={}", pagination.offset() + pagination.limit()),
    })))
}

/// PUT /notifications/mark-read - Mark notifications as read
pub async fn mark_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<MarkReadRequest>,
) -> Result<Json<Value>, StatusCode> {
    match params.id {
        Some(notification_id) => {
            // Mark a single notification as read
            stevessr_services::notifications::mark_as_read(
                &state.db,
                notification_id,
                user.0.id,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        None => {
            // Mark all notifications as read
            stevessr_services::notifications::mark_all_read(&state.db, user.0.id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    Ok(Json(json!({
        "success": "OK",
    })))
}
