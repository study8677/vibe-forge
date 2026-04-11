use axum::{extract::State, Json, http::StatusCode};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;

/// GET /admin/plugins - List installed plugins
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let plugins = stevessr_services::admin::plugins::list_plugins(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let plugin_list: Vec<Value> = plugins
        .iter()
        .map(|p| {
            json!({
                "id": p.id,
                "name": p.name,
                "version": p.version,
                "url": p.url,
                "enabled": p.enabled,
                "enabled_setting": p.enabled_setting,
                "has_settings": p.has_settings,
                "is_official": p.is_official,
                "commit_hash": p.commit_hash,
                "commit_url": p.commit_url,
                "authors": p.authors,
                "about": p.about,
            })
        })
        .collect();

    Ok(Json(json!({
        "plugins": plugin_list,
    })))
}
