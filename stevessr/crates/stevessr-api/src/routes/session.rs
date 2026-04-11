use axum::{extract::State, extract::Path, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::{AuthenticatedUser, OptionalUser};
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
    #[serde(default)]
    pub second_factor_token: Option<String>,
    #[serde(default)]
    pub second_factor_method: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub login: String,
}

#[derive(Debug, Serialize)]
pub struct CsrfResponse {
    pub csrf: String,
}

/// POST /session - Create a new session (login)
pub async fn create(
    State(state): State<AppState>,
    JsonOrForm(params): JsonOrForm<LoginRequest>,
) -> Result<Json<Value>, StatusCode> {
    let result = stevessr_services::session::authenticate(
        &state.db,
        &params.login,
        &params.password,
        params.second_factor_token.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Store session token in Redis
    let mut redis_conn = state.redis.clone();
    let _: () = redis::cmd("SET")
        .arg(format!("session:{}", result.token))
        .arg(result.user_id)
        .arg("EX")
        .arg(86400 * 60) // 60-day session expiry
        .query_async(&mut redis_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Json(json!({
        "user": {
            "id": result.user_id,
            "username": result.username,
            "name": result.name,
            "avatar_template": result.avatar_template,
            "email": result.email,
            "admin": result.admin,
            "moderator": result.moderator,
            "trust_level": result.trust_level,
        },
        "token": result.token,
    }))
    .pipe(Ok)
}

/// DELETE /session/{id} - Destroy session (logout)
pub async fn destroy(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<String>,
) -> StatusCode {
    let mut redis_conn = state.redis.clone();
    let result: Result<(), _> = redis::cmd("DEL")
        .arg(format!("session:{}", id))
        .query_async(&mut redis_conn)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// GET /session/csrf - Get CSRF token
pub async fn csrf(
    State(_state): State<AppState>,
) -> Json<Value> {
    let token = stevessr_services::session::generate_csrf_token();
    Json(json!({
        "csrf": token,
    }))
}

/// GET /session/current - Get current user info
pub async fn current(
    State(state): State<AppState>,
    OptionalUser(user): OptionalUser,
) -> Json<Value> {
    match user {
        Some(u) => {
            let user_data = stevessr_services::users::find_by_id(&state.db, u.id)
                .await
                .ok();

            match user_data {
                Some(user_record) => Json(json!({
                    "current_user": {
                        "id": user_record.id,
                        "username": user_record.username,
                        "name": user_record.name,
                        "avatar_template": user_record.avatar_template,
                        "email": user_record.email,
                        "admin": user_record.admin,
                        "moderator": user_record.moderator,
                        "trust_level": user_record.trust_level,
                        "unread_notifications": user_record.unread_notifications,
                        "unread_high_priority_notifications": user_record.unread_high_priority_notifications,
                        "read_first_notification": user_record.read_first_notification,
                        "can_create_topic": user_record.trust_level >= 1,
                        "can_send_private_messages": user_record.trust_level >= 1,
                    }
                })),
                None => Json(json!({ "current_user": null })),
            }
        }
        None => Json(json!({ "current_user": null })),
    }
}

/// POST /session/forgot_password - Request password reset email
pub async fn forgot_password(
    State(state): State<AppState>,
    JsonOrForm(params): JsonOrForm<ForgotPasswordRequest>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::session::send_password_reset(&state.db, &params.login)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
        "user_found": true,
    })))
}

// Helper trait for method chaining
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}
impl<T> Pipe for T {}
