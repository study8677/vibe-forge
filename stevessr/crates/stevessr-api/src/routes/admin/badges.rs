use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateBadgeRequest {
    pub name: String,
    pub badge_type_id: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub long_description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub badge_grouping_id: Option<i64>,
    #[serde(default)]
    pub allow_title: Option<bool>,
    #[serde(default)]
    pub multiple_grant: Option<bool>,
    #[serde(default)]
    pub listable: Option<bool>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub auto_revoke: Option<bool>,
    #[serde(default)]
    pub show_posts: Option<bool>,
    #[serde(default)]
    pub target_posts: Option<bool>,
}

/// GET /admin/badges - List all badges (admin view with extra details)
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let badges = stevessr_services::admin::badges::list_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let badge_list: Vec<Value> = badges
        .iter()
        .map(|b| {
            json!({
                "id": b.id,
                "name": b.name,
                "description": b.description,
                "long_description": b.long_description,
                "grant_count": b.grant_count,
                "allow_title": b.allow_title,
                "multiple_grant": b.multiple_grant,
                "icon": b.icon,
                "image_url": b.image_url,
                "listable": b.listable,
                "enabled": b.enabled,
                "badge_grouping_id": b.badge_grouping_id,
                "system": b.system,
                "badge_type_id": b.badge_type_id,
                "query": b.query,
                "auto_revoke": b.auto_revoke,
                "show_posts": b.show_posts,
                "target_posts": b.target_posts,
            })
        })
        .collect();

    let badge_groupings = vec![
        json!({"id": 1, "name": "Getting Started"}),
        json!({"id": 2, "name": "Community"}),
        json!({"id": 3, "name": "Posting"}),
        json!({"id": 4, "name": "Trust Level"}),
        json!({"id": 5, "name": "Other"}),
    ];

    let badge_types = vec![
        json!({"id": 1, "name": "Gold"}),
        json!({"id": 2, "name": "Silver"}),
        json!({"id": 3, "name": "Bronze"}),
    ];

    Ok(Json(json!({
        "badges": badge_list,
        "badge_groupings": badge_groupings,
        "badge_types": badge_types,
        "admin_badges": {
            "protected_system_fields": ["name", "badge_type_id", "query", "badge_grouping_id"],
        }
    })))
}

/// POST /admin/badges - Create a new badge
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateBadgeRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let badge = stevessr_services::admin::badges::create_badge(
        &state.db,
        &params.name,
        params.badge_type_id,
        params.description.as_deref(),
        params.long_description.as_deref(),
        params.icon.as_deref(),
        params.image_url.as_deref(),
        params.badge_grouping_id,
        params.allow_title.unwrap_or(false),
        params.multiple_grant.unwrap_or(false),
        params.listable.unwrap_or(true),
        params.enabled.unwrap_or(true),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "badge": {
                "id": badge.id,
                "name": badge.name,
                "description": badge.description,
                "badge_type_id": badge.badge_type_id,
                "grant_count": 0,
                "enabled": badge.enabled,
            }
        })),
    ))
}
