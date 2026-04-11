use axum::{extract::{State, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::OptionalUser;

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: String,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default = "default_context")]
    pub context: String,
    #[serde(rename = "type_filter")]
    #[serde(default)]
    pub type_filter: Option<String>,
    #[serde(default)]
    pub search_for_id: Option<bool>,
}

fn default_context() -> String {
    "default".to_string()
}

/// GET /search - Full-text search across topics, posts, users, categories, and tags
pub async fn query(
    State(state): State<AppState>,
    OptionalUser(current): OptionalUser,
    Query(params): Query<SearchParams>,
) -> Result<Json<Value>, StatusCode> {
    let current_user_id = current.as_ref().map(|u| u.id);
    let page = params.page.unwrap_or(1);

    let results = stevessr_services::search::query(
        &state.db,
        &params.q,
        current_user_id,
        page,
        params.type_filter.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let posts: Vec<Value> = results
        .posts
        .iter()
        .map(|p| {
            json!({
                "id": p.id,
                "name": p.name,
                "username": p.username,
                "avatar_template": p.avatar_template,
                "created_at": p.created_at,
                "like_count": p.like_count,
                "blurb": p.blurb,
                "post_number": p.post_number,
                "topic_title_headline": p.topic_title_headline,
                "topic_id": p.topic_id,
            })
        })
        .collect();

    let topics: Vec<Value> = results
        .topics
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "title": t.title,
                "fancy_title": t.fancy_title,
                "slug": t.slug,
                "posts_count": t.posts_count,
                "reply_count": t.reply_count,
                "created_at": t.created_at,
                "last_posted_at": t.last_posted_at,
                "views": t.views,
                "like_count": t.like_count,
                "category_id": t.category_id,
                "tags": t.tags,
                "archetype": t.archetype,
            })
        })
        .collect();

    let users: Vec<Value> = results
        .users
        .iter()
        .map(|u| {
            json!({
                "id": u.id,
                "username": u.username,
                "name": u.name,
                "avatar_template": u.avatar_template,
            })
        })
        .collect();

    let categories: Vec<Value> = results
        .categories
        .iter()
        .map(|c| {
            json!({
                "id": c.id,
                "name": c.name,
                "slug": c.slug,
                "color": c.color,
            })
        })
        .collect();

    let tags: Vec<Value> = results
        .tags
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "name": t.name,
                "topic_count": t.topic_count,
            })
        })
        .collect();

    Ok(Json(json!({
        "posts": posts,
        "topics": topics,
        "users": users,
        "categories": categories,
        "tags": tags,
        "grouped_search_result": {
            "more_posts": results.more_posts,
            "more_users": results.more_users,
            "more_categories": results.more_categories,
            "term": params.q,
            "search_log_id": results.search_log_id,
            "more_full_page_results": results.more_full_page_results,
            "can_create_topic": current.is_some(),
            "error": null,
            "type_filter": params.type_filter,
            "post_ids": results.post_ids,
            "user_ids": results.user_ids,
            "category_ids": results.category_ids,
            "tag_ids": results.tag_ids,
        }
    })))
}
