use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::{AuthenticatedUser, OptionalUser};
use crate::extractors::pagination::Pagination;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub bio_raw: Option<String>,
    #[serde(default)]
    pub visibility_level: Option<i32>,
    #[serde(default)]
    pub mentionable_level: Option<i32>,
    #[serde(default)]
    pub messageable_level: Option<i32>,
    #[serde(default)]
    pub members_visibility_level: Option<i32>,
    #[serde(default)]
    pub allow_membership_requests: Option<bool>,
    #[serde(default)]
    pub default_notification_level: Option<i32>,
    #[serde(default)]
    pub primary_group: Option<bool>,
    #[serde(default)]
    pub flair_icon: Option<String>,
    #[serde(default)]
    pub flair_color: Option<String>,
    #[serde(default)]
    pub flair_bg_color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub bio_raw: Option<String>,
    #[serde(default)]
    pub visibility_level: Option<i32>,
    #[serde(default)]
    pub mentionable_level: Option<i32>,
    #[serde(default)]
    pub messageable_level: Option<i32>,
    #[serde(default)]
    pub primary_group: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ModifyMembersRequest {
    pub usernames: Option<String>,
    #[serde(default)]
    pub emails: Option<String>,
}

/// GET /g - List all groups
pub async fn index(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let groups = stevessr_services::groups::list_all(
        &state.db,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group_list: Vec<Value> = groups
        .iter()
        .map(|g| {
            json!({
                "id": g.id,
                "name": g.name,
                "full_name": g.full_name,
                "user_count": g.user_count,
                "mentionable_level": g.mentionable_level,
                "messageable_level": g.messageable_level,
                "visibility_level": g.visibility_level,
                "automatic": g.automatic,
                "primary_group": g.primary_group,
                "flair_icon": g.flair_icon,
                "flair_color": g.flair_color,
                "flair_bg_color": g.flair_bg_color,
                "bio_cooked": g.bio_cooked,
                "bio_excerpt": g.bio_excerpt,
                "has_messages": g.has_messages,
                "allow_membership_requests": g.allow_membership_requests,
            })
        })
        .collect();

    Ok(Json(json!({
        "groups": group_list,
        "extras": {
            "type_filters": ["my", "owner", "public", "close", "automatic"],
        },
        "total_rows_groups": groups.len(),
        "load_more_groups": null,
    })))
}

/// POST /g - Create a new group
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<CreateGroupRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let group = stevessr_services::groups::create_group(
        &state.db,
        &params.name,
        params.full_name.as_deref(),
        params.bio_raw.as_deref(),
        params.visibility_level.unwrap_or(0),
        params.primary_group.unwrap_or(false),
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "basic_group": {
                "id": group.id,
                "name": group.name,
                "full_name": group.full_name,
                "user_count": 0,
            }
        })),
    ))
}

/// GET /g/{name} - Show a group
pub async fn show(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let group = stevessr_services::groups::find_by_name(&state.db, &name)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "group": {
            "id": group.id,
            "name": group.name,
            "full_name": group.full_name,
            "user_count": group.user_count,
            "bio_raw": group.bio_raw,
            "bio_cooked": group.bio_cooked,
            "bio_excerpt": group.bio_excerpt,
            "mentionable_level": group.mentionable_level,
            "messageable_level": group.messageable_level,
            "visibility_level": group.visibility_level,
            "automatic": group.automatic,
            "primary_group": group.primary_group,
            "flair_icon": group.flair_icon,
            "flair_color": group.flair_color,
            "flair_bg_color": group.flair_bg_color,
            "has_messages": group.has_messages,
            "allow_membership_requests": group.allow_membership_requests,
            "can_see_members": true,
            "can_admin_group": false,
        }
    })))
}

/// PUT /g/{id} - Update a group
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<UpdateGroupRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let group = stevessr_services::groups::update_group(
        &state.db,
        id,
        params.name.as_deref(),
        params.full_name.as_deref(),
        params.bio_raw.as_deref(),
        params.visibility_level,
        params.primary_group,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// DELETE /g/{id} - Delete a group
pub async fn destroy(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    stevessr_services::groups::delete_group(&state.db, id)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}

/// GET /g/{name}/members - List group members
pub async fn members(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let group = stevessr_services::groups::find_by_name(&state.db, &name)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let member_list = stevessr_services::groups::list_members(
        &state.db,
        group.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let members: Vec<Value> = member_list
        .iter()
        .map(|m| {
            json!({
                "id": m.id,
                "username": m.username,
                "name": m.name,
                "avatar_template": m.avatar_template,
                "title": m.title,
                "last_posted_at": m.last_posted_at,
                "last_seen_at": m.last_seen_at,
                "added_at": m.added_at,
            })
        })
        .collect();

    Ok(Json(json!({
        "members": members,
        "meta": {
            "total": member_list.len(),
            "limit": pagination.limit(),
            "offset": pagination.offset(),
        }
    })))
}

/// PUT /g/{id}/members - Add members to group
pub async fn add_members(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<ModifyMembersRequest>,
) -> Result<Json<Value>, StatusCode> {
    let usernames = params.usernames.unwrap_or_default();

    let result = stevessr_services::groups::add_members(
        &state.db,
        id,
        &usernames,
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
        "usernames": result.added_usernames,
    })))
}

/// DELETE /g/{id}/members - Remove members from group
pub async fn remove_members(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    JsonOrForm(params): JsonOrForm<ModifyMembersRequest>,
) -> Result<Json<Value>, StatusCode> {
    let usernames = params.usernames.unwrap_or_default();

    stevessr_services::groups::remove_members(
        &state.db,
        id,
        &usernames,
        user.0.id,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(json!({
        "success": "OK",
    })))
}
