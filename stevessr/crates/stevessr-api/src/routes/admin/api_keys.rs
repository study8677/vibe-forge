use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub description: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub scopes: Option<Vec<ApiKeyScope>>,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyScope {
    pub resource: String,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub allowed_parameters: Option<std::collections::HashMap<String, Vec<String>>>,
}

/// GET /admin/api - List API keys
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let keys = stevessr_services::admin::api_keys::list_keys(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let key_list: Vec<Value> = keys
        .iter()
        .map(|k| {
            json!({
                "id": k.id,
                "key": k.truncated_key,
                "description": k.description,
                "created_at": k.created_at,
                "updated_at": k.updated_at,
                "last_used_at": k.last_used_at,
                "revoked_at": k.revoked_at,
                "user": k.user,
                "api_key_scopes": k.scopes,
            })
        })
        .collect();

    Ok(Json(json!({
        "keys": key_list,
    })))
}

/// POST /admin/api - Create a new API key
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let key = stevessr_services::admin::api_keys::create_key(
        &state.db,
        &params.description,
        params.username.as_deref(),
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "key": {
                "id": key.id,
                "key": key.key,
                "description": key.description,
                "created_at": key.created_at,
                "user": key.user,
            }
        })),
    ))
}
