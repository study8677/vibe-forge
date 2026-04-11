use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;
use crate::extractors::pagination::Pagination;

#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// GET /tags - List all tags
pub async fn index(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let tags = stevessr_services::tags::list_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tag_list: Vec<Value> = tags
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "name": t.name,
                "description": t.description,
                "topic_count": t.topic_count,
                "pm_topic_count": t.pm_topic_count,
                "staff": t.staff,
            })
        })
        .collect();

    Ok(Json(json!({
        "tags": tag_list,
        "extras": {
            "tag_groups": [],
        }
    })))
}

/// GET /tag/{name} - Show a tag with its topics
pub async fn show(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let tag = stevessr_services::tags::find_by_name(&state.db, &name)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let topics = stevessr_services::tags::topics_for_tag(
        &state.db,
        &name,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let topic_list: Vec<Value> = topics
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "title": t.title,
                "slug": t.slug,
                "posts_count": t.posts_count,
                "reply_count": t.reply_count,
                "created_at": t.created_at,
                "last_posted_at": t.last_posted_at,
                "bumped_at": t.bumped_at,
                "views": t.views,
                "like_count": t.like_count,
                "category_id": t.category_id,
                "tags": t.tags,
            })
        })
        .collect();

    Ok(Json(json!({
        "tag": {
            "id": tag.id,
            "name": tag.name,
            "description": tag.description,
            "topic_count": tag.topic_count,
            "staff": tag.staff,
        },
        "topic_list": {
            "can_create_topic": false,
            "per_page": pagination.limit(),
            "topics": topic_list,
        }
    })))
}

/// PUT /tag/{id} - Update a tag
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateTagRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let tag = stevessr_services::tags::update_tag(
        &state.db,
        id,
        params.name.as_deref(),
        params.description.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "tag": {
            "id": tag.id,
            "name": tag.name,
            "description": tag.description,
            "topic_count": tag.topic_count,
        }
    })))
}

/// DELETE /tag/{id} - Delete a tag
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::tags::delete_tag(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
