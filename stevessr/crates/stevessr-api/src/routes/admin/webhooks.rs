use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub payload_url: String,
    pub content_type: i32,
    pub secret: Option<String>,
    #[serde(default)]
    pub wildcard_web_hook: Option<bool>,
    #[serde(default)]
    pub verify_certificate: Option<bool>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub web_hook_event_type_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub category_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub tag_names: Option<Vec<String>>,
    #[serde(default)]
    pub group_ids: Option<Vec<i64>>,
}

/// GET /admin/web_hooks - List all webhooks
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let webhooks = stevessr_services::admin::webhooks::list_webhooks(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let webhook_list: Vec<Value> = webhooks
        .iter()
        .map(|w| {
            json!({
                "id": w.id,
                "payload_url": w.payload_url,
                "content_type": w.content_type,
                "last_delivery_status": w.last_delivery_status,
                "wildcard_web_hook": w.wildcard_web_hook,
                "verify_certificate": w.verify_certificate,
                "active": w.active,
                "web_hook_event_types": w.web_hook_event_types,
                "categories": w.categories,
                "tags": w.tags,
                "groups": w.groups,
                "created_at": w.created_at,
                "updated_at": w.updated_at,
            })
        })
        .collect();

    let event_types = vec![
        json!({"id": 1, "name": "topic_event"}),
        json!({"id": 2, "name": "post_event"}),
        json!({"id": 3, "name": "user_event"}),
        json!({"id": 4, "name": "group_event"}),
        json!({"id": 5, "name": "category_event"}),
        json!({"id": 6, "name": "tag_event"}),
        json!({"id": 7, "name": "flag_event"}),
        json!({"id": 8, "name": "notification_event"}),
        json!({"id": 9, "name": "reviewable_event"}),
        json!({"id": 10, "name": "like_event"}),
    ];

    Ok(Json(json!({
        "web_hooks": webhook_list,
        "extras": {
            "event_types": event_types,
            "content_types": [
                {"id": 1, "name": "application/json"},
                {"id": 2, "name": "application/x-www-form-urlencoded"},
            ],
            "default_event_types": [1, 2],
        }
    })))
}

/// POST /admin/web_hooks - Create a webhook
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateWebhookRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let webhook = stevessr_services::admin::webhooks::create_webhook(
        &state.db,
        &params.payload_url,
        params.content_type,
        params.secret.as_deref(),
        params.wildcard_web_hook.unwrap_or(false),
        params.verify_certificate.unwrap_or(true),
        params.active.unwrap_or(true),
        params.web_hook_event_type_ids.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "web_hook": {
                "id": webhook.id,
                "payload_url": webhook.payload_url,
                "content_type": webhook.content_type,
                "active": webhook.active,
                "created_at": webhook.created_at,
            }
        })),
    ))
}
