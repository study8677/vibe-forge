use axum::{extract::State, Json, http::StatusCode};
use axum::extract::Multipart;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;

/// POST /uploads - Upload a file (image, attachment)
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<Json<Value>, StatusCode> {
    let mut file_name = String::new();
    let mut file_data = Vec::new();
    let mut file_type = String::new();
    let mut upload_type = String::from("composer");

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" | "files[]" => {
                file_name = field
                    .file_name()
                    .unwrap_or("upload")
                    .to_string();
                file_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                file_data = field
                    .bytes()
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?
                    .to_vec();
            }
            "type" => {
                upload_type = field
                    .text()
                    .await
                    .unwrap_or_else(|_| "composer".to_string());
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let upload = stevessr_services::uploads::create_upload(
        &state.db,
        user.0.id,
        &file_name,
        &file_type,
        &file_data,
        &upload_type,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "id": upload.id,
        "url": upload.url,
        "original_filename": upload.original_filename,
        "filesize": upload.filesize,
        "width": upload.width,
        "height": upload.height,
        "thumbnail_width": upload.thumbnail_width,
        "thumbnail_height": upload.thumbnail_height,
        "extension": upload.extension,
        "short_url": upload.short_url,
        "short_path": upload.short_path,
        "retain_hours": upload.retain_hours,
        "human_filesize": upload.human_filesize,
    })))
}
