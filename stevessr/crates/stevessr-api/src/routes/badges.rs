use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct GrantBadgeRequest {
    pub username: String,
    pub badge_id: i64,
    #[serde(default)]
    pub reason: Option<String>,
}

/// GET /badges - List all available badges
pub async fn index(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let badges = stevessr_services::badges::list_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let badge_list: Vec<Value> = badges
        .iter()
        .map(|b| {
            json!({
                "id": b.id,
                "name": b.name,
                "description": b.description,
                "grant_count": b.grant_count,
                "allow_title": b.allow_title,
                "multiple_grant": b.multiple_grant,
                "icon": b.icon,
                "listable": b.listable,
                "enabled": b.enabled,
                "badge_grouping_id": b.badge_grouping_id,
                "system": b.system,
                "badge_type_id": b.badge_type_id,
                "long_description": b.long_description,
                "image_url": b.image_url,
            })
        })
        .collect();

    let badge_types: Vec<Value> = vec![
        json!({"id": 1, "name": "Gold"}),
        json!({"id": 2, "name": "Silver"}),
        json!({"id": 3, "name": "Bronze"}),
    ];

    Ok(Json(json!({
        "badges": badge_list,
        "badge_types": badge_types,
    })))
}

/// POST /user_badges - Grant a badge to a user
pub async fn grant(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<GrantBadgeRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_badge = stevessr_services::badges::grant_badge(
        &state.db,
        &params.username,
        params.badge_id,
        user.0.id,
        params.reason.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "user_badge": {
                "id": user_badge.id,
                "granted_at": user_badge.granted_at,
                "badge_id": user_badge.badge_id,
                "user_id": user_badge.user_id,
                "granted_by_id": user_badge.granted_by_id,
            }
        })),
    ))
}
