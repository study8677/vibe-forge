use axum::{extract::{State, Path}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct UpdateSettingRequest {
    #[serde(flatten)]
    pub values: std::collections::HashMap<String, Value>,
}

/// GET /admin/site_settings - List all site settings
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let settings = stevessr_services::admin::site_settings::list_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let setting_list: Vec<Value> = settings
        .iter()
        .map(|s| {
            json!({
                "setting": s.setting,
                "value": s.value,
                "default": s.default_value,
                "description": s.description,
                "type": s.setting_type,
                "category": s.category,
                "preview": s.preview,
                "secret": s.secret,
                "placeholder": s.placeholder,
                "mandatory_values": s.mandatory_values,
            })
        })
        .collect();

    Ok(Json(json!({
        "site_settings": setting_list,
        "diags": {
            "last_message_processed": null,
        }
    })))
}

/// PUT /admin/site_settings/{id} - Update a site setting
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<String>,
    JsonOrForm(params): JsonOrForm<UpdateSettingRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // The setting value is typically sent as { "setting_name": "value" }
    let value = params
        .values
        .values()
        .next()
        .cloned()
        .unwrap_or(Value::Null);

    stevessr_services::admin::site_settings::update_setting(
        &state.db,
        &id,
        &value,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}
