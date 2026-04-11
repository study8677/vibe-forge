use axum::{extract::{State, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;

#[derive(Debug, Deserialize)]
pub struct LogParams {
    #[serde(default)]
    pub action_id: Option<i64>,
    #[serde(default)]
    pub action_name: Option<String>,
    #[serde(default)]
    pub acting_user: Option<String>,
    #[serde(default)]
    pub target_user: Option<String>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// GET /admin/logs - List staff action logs
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<LogParams>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let logs = stevessr_services::admin::logs::list_staff_action_logs(
        &state.db,
        params.action_id,
        params.action_name.as_deref(),
        params.acting_user.as_deref(),
        params.target_user.as_deref(),
        params.subject.as_deref(),
        params.pagination.offset(),
        params.pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let log_list: Vec<Value> = logs
        .iter()
        .map(|l| {
            json!({
                "id": l.id,
                "action_name": l.action_name,
                "details": l.details,
                "context": l.context,
                "ip_address": l.ip_address,
                "email": l.email,
                "created_at": l.created_at,
                "acting_user": {
                    "id": l.acting_user_id,
                    "username": l.acting_username,
                    "avatar_template": l.acting_avatar_template,
                },
                "target_user": {
                    "id": l.target_user_id,
                    "username": l.target_username,
                    "avatar_template": l.target_avatar_template,
                },
                "subject": l.subject,
                "previous_value": l.previous_value,
                "new_value": l.new_value,
                "custom_type": l.custom_type,
            })
        })
        .collect();

    let action_types = stevessr_services::admin::logs::list_action_types()
        .iter()
        .map(|a| {
            json!({
                "id": a.id,
                "name": a.name,
            })
        })
        .collect::<Vec<Value>>();

    Ok(Json(json!({
        "staff_action_logs": log_list,
        "staff_action_types": action_types,
        "user_history_actions": [],
    })))
}
