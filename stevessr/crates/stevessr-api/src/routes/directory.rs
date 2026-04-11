use axum::{extract::{State, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::pagination::Pagination;

#[derive(Debug, Deserialize)]
pub struct DirectoryParams {
    #[serde(default = "default_period")]
    pub period: String,
    #[serde(default = "default_order")]
    pub order: String,
    #[serde(default)]
    pub asc: Option<bool>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub exclude_usernames: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

fn default_period() -> String { "weekly".to_string() }
fn default_order() -> String { "likes_received".to_string() }

/// GET /directory_items - User directory with leaderboard stats
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<DirectoryParams>,
) -> Result<Json<Value>, StatusCode> {
    let items = stevessr_services::directory::list_items(
        &state.db,
        &params.period,
        &params.order,
        params.asc.unwrap_or(false),
        params.group.as_deref(),
        params.pagination.offset(),
        params.pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let directory_items: Vec<Value> = items
        .iter()
        .map(|item| {
            json!({
                "id": item.id,
                "likes_received": item.likes_received,
                "likes_given": item.likes_given,
                "topics_entered": item.topics_entered,
                "topic_count": item.topic_count,
                "post_count": item.post_count,
                "posts_read": item.posts_read,
                "days_visited": item.days_visited,
                "user": {
                    "id": item.user_id,
                    "username": item.username,
                    "name": item.name,
                    "avatar_template": item.avatar_template,
                    "title": item.title,
                    "primary_group_name": item.primary_group_name,
                    "primary_group_flair_url": item.primary_group_flair_url,
                    "primary_group_flair_color": item.primary_group_flair_color,
                    "primary_group_flair_bg_color": item.primary_group_flair_bg_color,
                },
            })
        })
        .collect();

    Ok(Json(json!({
        "directory_items": directory_items,
        "meta": {
            "total_rows_directory_items": items.len(),
            "load_more_directory_items": null,
        }
    })))
}
