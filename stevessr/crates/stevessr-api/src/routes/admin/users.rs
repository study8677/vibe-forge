use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct AdminUserListParams {
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub show_emails: Option<bool>,
    #[serde(default)]
    pub order: Option<String>,
    #[serde(default)]
    pub asc: Option<bool>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct SuspendRequest {
    pub duration: String,
    pub reason: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub post_action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SilenceRequest {
    #[serde(default)]
    pub silenced_till: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub post_action: Option<String>,
}

/// GET /admin/users - List users (admin view)
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<AdminUserListParams>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let users = stevessr_services::admin::users::list_users(
        &state.db,
        params.filter.as_deref(),
        params.order.as_deref(),
        params.asc.unwrap_or(false),
        params.pagination.offset(),
        params.pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_list: Vec<Value> = users
        .iter()
        .map(|u| {
            json!({
                "id": u.id,
                "username": u.username,
                "email": u.email,
                "name": u.name,
                "avatar_template": u.avatar_template,
                "active": u.active,
                "admin": u.admin,
                "moderator": u.moderator,
                "trust_level": u.trust_level,
                "created_at": u.created_at,
                "last_seen_at": u.last_seen_at,
                "last_emailed_at": u.last_emailed_at,
                "suspended_at": u.suspended_at,
                "suspended_till": u.suspended_till,
                "silenced_till": u.silenced_till,
                "staged": u.staged,
                "approved": u.approved,
                "ip_address": u.ip_address,
                "registration_ip_address": u.registration_ip_address,
                "post_count": u.post_count,
                "topic_count": u.topic_count,
                "like_count": u.like_count,
                "days_visited": u.days_visited,
                "posts_read": u.posts_read,
                "topics_entered": u.topics_entered,
                "time_read": u.time_read,
            })
        })
        .collect();

    Ok(Json(json!(user_list)))
}

/// PUT /admin/users/{id}/suspend - Suspend a user
pub async fn suspend(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<SuspendRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = stevessr_services::admin::users::suspend_user(
        &state.db,
        id,
        &params.duration,
        &params.reason,
        params.message.as_deref(),
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "suspension": {
            "suspended_at": result.suspended_at,
            "suspended_till": result.suspended_till,
            "suspend_reason": result.suspend_reason,
        }
    })))
}

/// PUT /admin/users/{id}/unsuspend - Unsuspend a user
pub async fn unsuspend(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::admin::users::unsuspend_user(&state.db, id)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// PUT /admin/users/{id}/silence - Silence a user
pub async fn silence(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<SilenceRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = stevessr_services::admin::users::silence_user(
        &state.db,
        id,
        params.silenced_till.as_deref(),
        params.reason.as_deref(),
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "silence": {
            "silenced": true,
            "silenced_till": result.silenced_till,
            "silence_reason": result.silence_reason,
        }
    })))
}

/// PUT /admin/users/{id}/grant_admin - Grant admin privileges
pub async fn grant_admin(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::admin::users::grant_admin(&state.db, id)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// PUT /admin/users/{id}/revoke_admin - Revoke admin privileges
pub async fn revoke_admin(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::admin::users::revoke_admin(&state.db, id)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}
