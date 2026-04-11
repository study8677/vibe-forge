use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

/// GET /about.json - Site statistics and information
pub async fn json(
    State(state): State<AppState>,
) -> Json<Value> {
    let stats = stevessr_services::site::get_statistics(&state.db)
        .await
        .unwrap_or_default();

    let admins = stevessr_services::users::list_admins(&state.db)
        .await
        .unwrap_or_default();

    let moderators = stevessr_services::users::list_moderators(&state.db)
        .await
        .unwrap_or_default();

    let admin_list: Vec<Value> = admins
        .iter()
        .map(|u| {
            json!({
                "id": u.id,
                "username": u.username,
                "avatar_template": u.avatar_template,
                "name": u.name,
                "title": u.title,
                "last_seen_at": u.last_seen_at,
            })
        })
        .collect();

    let moderator_list: Vec<Value> = moderators
        .iter()
        .map(|u| {
            json!({
                "id": u.id,
                "username": u.username,
                "avatar_template": u.avatar_template,
                "name": u.name,
                "title": u.title,
                "last_seen_at": u.last_seen_at,
            })
        })
        .collect();

    Json(json!({
        "about": {
            "can_see_about_stats": true,
            "stats": {
                "topic_count": stats.topic_count,
                "post_count": stats.post_count,
                "user_count": stats.user_count,
                "topics_7_days": stats.topics_7_days,
                "topics_30_days": stats.topics_30_days,
                "posts_7_days": stats.posts_7_days,
                "posts_30_days": stats.posts_30_days,
                "users_7_days": stats.users_7_days,
                "users_30_days": stats.users_30_days,
                "active_users_7_days": stats.active_users_7_days,
                "active_users_30_days": stats.active_users_30_days,
                "like_count": stats.like_count,
                "likes_7_days": stats.likes_7_days,
                "likes_30_days": stats.likes_30_days,
            },
            "description": &*state.config.site_description,
            "title": &*state.config.site_title,
            "locale": "en",
            "version": env!("CARGO_PKG_VERSION"),
            "admins": admin_list,
            "moderators": moderator_list,
        }
    }))
}
