use axum::{extract::{State, Path}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateInviteRequest {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub skip_email: Option<bool>,
    #[serde(default)]
    pub custom_message: Option<String>,
    #[serde(default)]
    pub max_redemptions_allowed: Option<i32>,
    #[serde(default)]
    pub topic_id: Option<i64>,
    #[serde(default)]
    pub group_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub group_names: Option<Vec<String>>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInviteRequest {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub custom_message: Option<String>,
    #[serde(default)]
    pub max_redemptions_allowed: Option<i32>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub send_email: Option<bool>,
}

/// POST /invites - Create an invite
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateInviteRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let invite = stevessr_services::invites::create_invite(
        &state.db,
        user.0.id,
        params.email.as_deref(),
        params.skip_email.unwrap_or(false),
        params.custom_message.as_deref(),
        params.max_redemptions_allowed.unwrap_or(1),
        params.topic_id,
        params.group_ids.as_deref(),
        params.expires_at.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": invite.id,
            "link": invite.link,
            "email": invite.email,
            "emailed": invite.emailed,
            "custom_message": invite.custom_message,
            "created_at": invite.created_at,
            "updated_at": invite.updated_at,
            "expires_at": invite.expires_at,
            "expired": invite.expired,
            "max_redemptions_allowed": invite.max_redemptions_allowed,
            "redemption_count": 0,
            "topics": invite.topics,
            "groups": invite.groups,
        })),
    ))
}

/// PUT /invites/{id} - Update an invite
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateInviteRequest>,
) -> Result<Json<Value>, StatusCode> {
    let invite = stevessr_services::invites::update_invite(
        &state.db,
        id,
        user.0.id,
        params.email.as_deref(),
        params.custom_message.as_deref(),
        params.max_redemptions_allowed,
        params.expires_at.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "id": invite.id,
        "link": invite.link,
        "email": invite.email,
        "expires_at": invite.expires_at,
        "max_redemptions_allowed": invite.max_redemptions_allowed,
    })))
}

/// DELETE /invites/{id} - Revoke an invite
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::invites::revoke_invite(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(StatusCode::OK)
}

/// GET /invites/show/{invite_key} - Show invite details (for redemption page)
pub async fn show(
    State(state): State<AppState>,
    Path(invite_key): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let invite = stevessr_services::invites::find_by_key(&state.db, &invite_key)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if invite.expired {
        return Err(StatusCode::GONE);
    }

    Ok(Json(json!({
        "invite": {
            "invited_by": {
                "id": invite.invited_by.id,
                "username": invite.invited_by.username,
                "name": invite.invited_by.name,
                "avatar_template": invite.invited_by.avatar_template,
            },
            "email": invite.email,
            "email_verified": invite.email_verified,
            "topic": invite.topic,
            "groups": invite.groups,
        }
    })))
}
