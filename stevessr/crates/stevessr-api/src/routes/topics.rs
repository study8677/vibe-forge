use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::{AuthenticatedUser, OptionalUser};
use crate::extractors::json_or_form::JsonOrForm;
use crate::serializers::topic::serialize_topic;
use crate::serializers::post::serialize_post;

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub raw: String,
    #[serde(default)]
    pub category: Option<i64>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub archetype: Option<String>,
    #[serde(default)]
    pub target_recipients: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTopicRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
    pub enabled: bool,
    #[serde(default)]
    pub until: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetTimerRequest {
    pub status_type: String,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
    pub based_on_last_post: Option<bool>,
    #[serde(default)]
    pub category_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct InviteRequest {
    pub user: Option<String>,
    pub email: Option<String>,
    #[serde(default)]
    pub group_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub custom_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MovePostsRequest {
    pub post_ids: Vec<i64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub destination_topic_id: Option<i64>,
    #[serde(default)]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct MergeRequest {
    pub destination_topic_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct TimingsRequest {
    pub topic_id: i64,
    pub topic_time: i64,
    pub timings: std::collections::HashMap<String, i64>,
}

#[derive(Debug, Deserialize)]
pub struct TopicQueryParams {
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub print: Option<bool>,
}

/// POST /t - Create a new topic
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateTopicRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let topic = stevessr_services::topics::create_topic(
        &state.db,
        user.0.id,
        &params.title,
        &params.raw,
        params.category,
        params.tags.as_deref(),
        params.archetype.as_deref(),
        params.target_recipients.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "topic_id": topic.id,
            "topic_slug": topic.slug,
            "post": serialize_post(&topic.first_post),
        })),
    ))
}

/// GET /t/{id} - Show topic with posts
pub async fn show(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Path(id): Path<i64>,
    Query(params): Query<TopicQueryParams>,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);

    let topic = stevessr_services::topics::find_by_id(
        &state.db,
        id,
        current_user_id,
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    // Record topic view
    if let Some(uid) = current_user_id {
        let _ = stevessr_services::topics::record_view(&state.db, id, uid).await;
    }

    let page = params.page.unwrap_or(0);
    let posts = stevessr_services::posts::for_topic(
        &state.db,
        id,
        page as i64 * 20,
        20,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post_list: Vec<Value> = posts.iter().map(|p| serialize_post(p)).collect();

    Ok(Json(json!({
        "post_stream": {
            "posts": post_list,
            "stream": topic.post_stream,
        },
        "timeline_lookup": topic.timeline_lookup,
        "id": topic.id,
        "title": topic.title,
        "fancy_title": topic.fancy_title,
        "slug": topic.slug,
        "posts_count": topic.posts_count,
        "reply_count": topic.reply_count,
        "highest_post_number": topic.highest_post_number,
        "created_at": topic.created_at,
        "last_posted_at": topic.last_posted_at,
        "bumped": topic.bumped,
        "bumped_at": topic.bumped_at,
        "archetype": topic.archetype,
        "unseen": topic.unseen,
        "pinned": topic.pinned,
        "unpinned": topic.unpinned,
        "visible": topic.visible,
        "closed": topic.closed,
        "archived": topic.archived,
        "views": topic.views,
        "like_count": topic.like_count,
        "has_summary": topic.has_summary,
        "word_count": topic.word_count,
        "category_id": topic.category_id,
        "pinned_globally": topic.pinned_globally,
        "tags": topic.tags,
        "details": {
            "can_edit": topic.details.can_edit,
            "can_delete": topic.details.can_delete,
            "can_create_post": topic.details.can_create_post,
            "can_invite_to": topic.details.can_invite_to,
            "notification_level": topic.details.notification_level,
            "created_by": topic.details.created_by,
            "last_poster": topic.details.last_poster,
            "participants": topic.details.participants,
        },
    })))
}

/// PUT /t/{id} - Update topic title/category/tags
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateTopicRequest>,
) -> Result<Json<Value>, StatusCode> {
    let topic = stevessr_services::topics::update_topic(
        &state.db,
        id,
        user.0.id,
        params.title.as_deref(),
        params.category_id,
        params.tags.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "basic_topic": serialize_topic(&topic),
    })))
}

/// DELETE /t/{id} - Delete topic
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::topics::delete_topic(&state.db, id, user.0.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(StatusCode::OK)
}

/// PUT /t/{id}/status - Update topic status (close, archive, pin, etc.)
pub async fn update_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateStatusRequest>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::topics::update_status(
        &state.db,
        id,
        user.0.id,
        &params.status,
        params.enabled,
        params.until.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(Json(json!({
        "success": "OK",
        "topic_status_update": null,
    })))
}

/// PUT /t/{id}/timer - Set topic timer (auto-close, auto-delete, etc.)
pub async fn set_timer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<SetTimerRequest>,
) -> Result<Json<Value>, StatusCode> {
    let timer = stevessr_services::topics::set_timer(
        &state.db,
        id,
        user.0.id,
        &params.status_type,
        params.time.as_deref(),
        params.based_on_last_post.unwrap_or(false),
        params.category_id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
        "execute_at": timer.execute_at,
        "duration_minutes": timer.duration_minutes,
        "based_on_last_post": timer.based_on_last_post,
        "closed": timer.closed,
        "category_id": timer.category_id,
    })))
}

/// PUT /t/{id}/invite - Invite user to topic
pub async fn invite(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<InviteRequest>,
) -> Result<Json<Value>, StatusCode> {
    stevessr_services::topics::invite_to_topic(
        &state.db,
        id,
        user.0.id,
        params.user.as_deref(),
        params.email.as_deref(),
        params.custom_message.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// POST /t/{id}/move-posts - Move posts to another topic
pub async fn move_posts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<MovePostsRequest>,
) -> Result<Json<Value>, StatusCode> {
    let result = stevessr_services::topics::move_posts(
        &state.db,
        id,
        user.0.id,
        &params.post_ids,
        params.title.as_deref(),
        params.destination_topic_id,
        params.category_id,
        params.tags.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": true,
        "url": result.url,
    })))
}

/// POST /t/{id}/merge-topic - Merge topic into another
pub async fn merge(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<MergeRequest>,
) -> Result<Json<Value>, StatusCode> {
    let result = stevessr_services::topics::merge_topic(
        &state.db,
        id,
        user.0.id,
        params.destination_topic_id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": true,
        "url": result.url,
    })))
}

/// POST /t/{id}/timings - Record topic read timings
pub async fn timings(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    Json(params): Json<TimingsRequest>,
) -> Result<StatusCode, StatusCode> {
    stevessr_services::topics::record_timings(
        &state.db,
        id,
        user.0.id,
        params.topic_time,
        &params.timings,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

/// GET /t/{slug}/{id} - Show topic by slug and id
pub async fn show_with_slug(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Path((slug, id)): Path<(String, i64)>,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);

    let topic = stevessr_services::topics::find_by_id(&state.db, id, current_user_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Verify slug matches, redirect if not
    if topic.slug != slug {
        // In a real implementation this would be a redirect
        return Err(StatusCode::MOVED_PERMANENTLY);
    }

    if let Some(uid) = current_user_id {
        let _ = stevessr_services::topics::record_view(&state.db, id, uid).await;
    }

    let posts = stevessr_services::posts::for_topic(&state.db, id, 0, 20)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post_list: Vec<Value> = posts.iter().map(|p| serialize_post(p)).collect();

    Ok(Json(json!({
        "post_stream": {
            "posts": post_list,
            "stream": topic.post_stream,
        },
        "id": topic.id,
        "title": topic.title,
        "fancy_title": topic.fancy_title,
        "slug": topic.slug,
        "posts_count": topic.posts_count,
        "category_id": topic.category_id,
        "tags": topic.tags,
    })))
}

/// GET /t/{slug}/{id}/{post_number} - Show specific post in topic
pub async fn show_post(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Path((slug, id, post_number)): Path<(String, i64, i64)>,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);

    let topic = stevessr_services::topics::find_by_id(&state.db, id, current_user_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let post = stevessr_services::posts::find_by_topic_and_number(
        &state.db,
        id,
        post_number,
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "post_stream": {
            "posts": [serialize_post(&post)],
            "stream": topic.post_stream,
        },
        "id": topic.id,
        "title": topic.title,
        "slug": topic.slug,
        "current_post_number": post_number,
    })))
}
