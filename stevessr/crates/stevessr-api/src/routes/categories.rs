use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::{AuthenticatedUser, OptionalUser};
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;
use crate::serializers::category::serialize_category;

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub color: String,
    pub text_color: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parent_category_id: Option<i64>,
    #[serde(default)]
    pub permissions: Option<std::collections::HashMap<String, i32>>,
    #[serde(default)]
    pub allow_badges: Option<bool>,
    #[serde(default)]
    pub topic_template: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub sort_ascending: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub text_color: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parent_category_id: Option<i64>,
    #[serde(default)]
    pub permissions: Option<std::collections::HashMap<String, i32>>,
    #[serde(default)]
    pub topic_template: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderRequest {
    pub mapping: std::collections::HashMap<String, i32>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryTopicParams {
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub order: Option<String>,
    #[serde(default)]
    pub ascending: Option<bool>,
}

/// GET /categories - List all categories
pub async fn index(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);

    let categories = stevessr_services::categories::list_visible(&state.db, current_user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let category_list: Vec<Value> = categories.iter().map(|c| serialize_category(c)).collect();

    Ok(Json(json!({
        "category_list": {
            "can_create_category": current.as_ref().map(|u| u.admin).unwrap_or(false),
            "can_create_topic": current.is_some(),
            "categories": category_list,
        }
    })))
}

/// POST /categories - Create a new category
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let category = stevessr_services::categories::create_category(
        &state.db,
        &params.name,
        &params.color,
        &params.text_color,
        params.slug.as_deref(),
        params.description.as_deref(),
        params.parent_category_id,
        params.topic_template.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "category": serialize_category(&category),
        })),
    ))
}

/// PUT /categories/{id} - Update a category
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateCategoryRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let category = stevessr_services::categories::update_category(
        &state.db,
        id,
        stevessr_services::categories::UpdateCategoryParams {
            name: params.name,
            color: params.color,
            text_color: params.text_color,
            slug: params.slug,
            description: params.description,
            parent_category_id: params.parent_category_id,
            topic_template: params.topic_template,
        },
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "category": serialize_category(&category),
    })))
}

/// DELETE /categories/{id} - Delete a category
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::categories::delete_category(&state.db, id)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// POST /categories/reorder - Reorder categories
pub async fn reorder(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<ReorderRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::categories::reorder(&state.db, &params.mapping)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// GET /c/{slug}/{id} - Show a single category
pub async fn show(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Path((slug, id)): Path<(String, i64)>,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);

    let category = stevessr_services::categories::find_by_id(&state.db, id, current_user_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "category": serialize_category(&category),
    })))
}

/// GET /c/{slug}/{id}/l/latest - List topics in category
pub async fn topic_list(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Path((slug, id)): Path<(String, i64)>,
    Query(params): Query<CategoryTopicParams>,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);
    let page = params.page.unwrap_or(0);

    let topics = stevessr_services::topics::list_by_category(
        &state.db,
        id,
        current_user_id,
        page as i64 * 30,
        30,
        params.order.as_deref(),
        params.ascending.unwrap_or(false),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let topic_list: Vec<Value> = topics
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "title": t.title,
                "fancy_title": t.fancy_title,
                "slug": t.slug,
                "posts_count": t.posts_count,
                "reply_count": t.reply_count,
                "highest_post_number": t.highest_post_number,
                "created_at": t.created_at,
                "last_posted_at": t.last_posted_at,
                "bumped_at": t.bumped_at,
                "unseen": t.unseen,
                "pinned": t.pinned,
                "visible": t.visible,
                "closed": t.closed,
                "archived": t.archived,
                "views": t.views,
                "like_count": t.like_count,
                "category_id": t.category_id,
                "tags": t.tags,
                "posters": t.posters,
            })
        })
        .collect();

    Ok(Json(json!({
        "topic_list": {
            "can_create_topic": current.is_some(),
            "per_page": 30,
            "topics": topic_list,
        }
    })))
}
