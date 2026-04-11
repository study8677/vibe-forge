use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    #[serde(default)]
    pub with_uploads: Option<bool>,
}

/// GET /admin/backups - List available backups
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let backups = stevessr_services::admin::backups::list_backups(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let backup_list: Vec<Value> = backups
        .iter()
        .map(|b| {
            json!({
                "filename": b.filename,
                "size": b.size,
                "human_size": b.human_size,
                "link": b.link,
                "last_modified": b.last_modified,
            })
        })
        .collect();

    Ok(Json(json!({
        "backups": backup_list,
        "is_operation_running": false,
        "can_rollback": true,
        "allow_restore": true,
    })))
}

/// POST /admin/backups - Start a new backup
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateBackupRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let with_uploads = params.with_uploads.unwrap_or(true);

    stevessr_services::admin::backups::start_backup(&state.db, with_uploads)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": "OK",
        "message": "Backup started. You will receive a notification when the backup completes.",
    })))
}
