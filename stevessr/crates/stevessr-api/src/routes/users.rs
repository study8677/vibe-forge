use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::{AuthenticatedUser, OptionalUser};
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;
use crate::serializers::user::serialize_user;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    #[serde(default)]
    pub invite_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub bio_raw: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub date_of_birth: Option<String>,
    #[serde(default)]
    pub card_background: Option<String>,
    #[serde(default)]
    pub profile_background: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserSearchParams {
    #[serde(default)]
    pub term: String,
    #[serde(default)]
    pub topic_allowed_users: Option<bool>,
    #[serde(default)]
    pub topic_id: Option<i64>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub include_groups: Option<bool>,
    #[serde(default)]
    pub limit: Option<i64>,
}

/// POST /u - Create a new user (registration)
pub async fn create(
    State(state): State<AppState>,
    JsonOrForm(params): JsonOrForm<CreateUserRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let user = stevessr_services::users::create_user(
        &state.db,
        &params.name,
        &params.email,
        &params.password,
        &params.username,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "active": user.active,
            "message": "Your account has been created successfully.",
            "user_id": user.id,
        })),
    ))
}

/// GET /u/{username} - Show user profile
pub async fn show(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Path(username): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let user = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let is_own_profile = current.as_ref().map(|c| c.id == user.id).unwrap_or(false);

    Ok(Json(json!({
        "user": serialize_user(&user, is_own_profile),
    })))
}

/// PUT /u/{username} - Update user profile
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(username): Path<String>,
    JsonOrForm(params): JsonOrForm<UpdateUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Ensure user can only edit their own profile or is admin
    let target = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if target.id != user.0.id && !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let updated = stevessr_services::users::update_profile(
        &state.db,
        target.id,
        stevessr_services::users::UpdateProfileParams {
            name: params.name,
            bio_raw: params.bio_raw,
            website: params.website,
            location: params.location,
            title: params.title,
            date_of_birth: params.date_of_birth,
            card_background: params.card_background,
            profile_background: params.profile_background,
        },
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "user": serialize_user(&updated, true),
    })))
}

/// GET /u/{username}/summary - User activity summary
pub async fn summary(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let user = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let summary = stevessr_services::users::get_summary(&state.db, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "user_summary": {
            "likes_given": summary.likes_given,
            "likes_received": summary.likes_received,
            "topics_entered": summary.topics_entered,
            "posts_read_count": summary.posts_read_count,
            "days_visited": summary.days_visited,
            "topic_count": summary.topic_count,
            "post_count": summary.post_count,
            "time_read": summary.time_read,
            "recent_time_read": summary.recent_time_read,
            "bookmark_count": summary.bookmark_count,
            "top_replies": summary.top_replies,
            "top_topics": summary.top_topics,
            "top_categories": summary.top_categories,
            "badges": summary.badges,
        }
    })))
}

/// GET /u/{username}/activity - User activity stream
pub async fn activity(
    State(state): State<AppState>,
    Path(username): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let user = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let actions = stevessr_services::users::get_activity(
        &state.db,
        user.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let action_list: Vec<Value> = actions
        .iter()
        .map(|a| {
            json!({
                "action_type": a.action_type,
                "created_at": a.created_at,
                "post_id": a.post_id,
                "post_number": a.post_number,
                "topic_id": a.topic_id,
                "slug": a.slug,
                "title": a.title,
                "excerpt": a.excerpt,
            })
        })
        .collect();

    Ok(Json(json!({
        "user_actions": action_list,
    })))
}

/// GET /u/{username}/badges - User badges
pub async fn badges(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let user = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let user_badges = stevessr_services::badges::for_user(&state.db, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let badge_list: Vec<Value> = user_badges
        .iter()
        .map(|b| {
            json!({
                "id": b.id,
                "granted_at": b.granted_at,
                "badge_id": b.badge_id,
                "badge": {
                    "id": b.badge_id,
                    "name": b.badge_name,
                    "description": b.badge_description,
                    "badge_type_id": b.badge_type_id,
                    "icon": b.icon,
                },
            })
        })
        .collect();

    Ok(Json(json!({
        "badges": badge_list,
    })))
}

/// GET /u/{username}/notifications - User notifications
pub async fn notifications(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(username): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let target = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if target.id != user.0.id && !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let notifs = stevessr_services::notifications::for_user(
        &state.db,
        target.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let notification_list: Vec<Value> = notifs
        .iter()
        .map(|n| {
            json!({
                "id": n.id,
                "notification_type": n.notification_type,
                "read": n.read,
                "created_at": n.created_at,
                "post_number": n.post_number,
                "topic_id": n.topic_id,
                "slug": n.slug,
                "data": n.data,
            })
        })
        .collect();

    Ok(Json(json!({
        "notifications": notification_list,
    })))
}

/// GET /u/{username}/bookmarks - User bookmarks
pub async fn bookmarks(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(username): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let target = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if target.id != user.0.id && !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let marks = stevessr_services::bookmarks::for_user(
        &state.db,
        target.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let bookmark_list: Vec<Value> = marks
        .iter()
        .map(|b| {
            json!({
                "id": b.id,
                "created_at": b.created_at,
                "updated_at": b.updated_at,
                "name": b.name,
                "reminder_at": b.reminder_at,
                "bookmarkable_id": b.bookmarkable_id,
                "bookmarkable_type": b.bookmarkable_type,
                "title": b.title,
                "excerpt": b.excerpt,
            })
        })
        .collect();

    Ok(Json(json!({
        "user_bookmark_list": {
            "bookmarks": bookmark_list,
        }
    })))
}

/// GET /u/{username}/drafts - User drafts
pub async fn drafts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(username): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let target = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if target.id != user.0.id && !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_drafts = stevessr_services::drafts::for_user(
        &state.db,
        target.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let draft_list: Vec<Value> = user_drafts
        .iter()
        .map(|d| {
            json!({
                "id": d.id,
                "draft_key": d.draft_key,
                "sequence": d.sequence,
                "data": d.data,
                "created_at": d.created_at,
                "updated_at": d.updated_at,
            })
        })
        .collect();

    Ok(Json(json!({
        "drafts": draft_list,
    })))
}

/// DELETE /u/{username} - Deactivate user account
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(username): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let target = stevessr_services::users::find_by_username(&state.db, &username)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if target.id != user.0.id && !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::users::deactivate(&state.db, target.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

/// GET /u/search/users - Search users
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<UserSearchParams>,
) -> Result<Json<Value>, StatusCode> {
    let limit = params.limit.unwrap_or(8);

    let users = stevessr_services::users::search(
        &state.db,
        &params.term,
        limit,
        params.topic_id,
        params.group.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_list: Vec<Value> = users
        .iter()
        .map(|u| {
            json!({
                "username": u.username,
                "name": u.name,
                "avatar_template": u.avatar_template,
            })
        })
        .collect();

    Ok(Json(json!({
        "users": user_list,
    })))
}
