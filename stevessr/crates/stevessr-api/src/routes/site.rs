use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

/// GET /site.json - Bootstrap payload for site configuration, categories, and trust levels
pub async fn show(
    State(state): State<AppState>,
) -> Json<Value> {
    let categories = stevessr_services::categories::list_all(&state.db)
        .await
        .unwrap_or_default();

    let category_list: Vec<Value> = categories
        .iter()
        .map(|c| {
            json!({
                "id": c.id,
                "name": c.name,
                "slug": c.slug,
                "color": c.color,
                "text_color": c.text_color,
                "description": c.description,
                "topic_count": c.topic_count,
                "post_count": c.post_count,
                "position": c.position,
                "parent_category_id": c.parent_category_id,
                "read_restricted": c.read_restricted,
            })
        })
        .collect();

    Json(json!({
        "default_archetype": "regular",
        "notification_types": {
            "mentioned": 1,
            "replied": 2,
            "quoted": 3,
            "edited": 4,
            "liked": 5,
            "private_message": 6,
            "invited_to_private_message": 7,
            "invitee_accepted": 8,
            "posted": 9,
            "moved_post": 10,
            "linked": 11,
            "granted_badge": 12,
            "invited_to_topic": 13,
            "custom": 14,
            "group_mentioned": 15,
            "group_message_summary": 16,
            "watching_first_post": 17,
            "topic_reminder": 18,
            "liked_consolidated": 19,
            "post_approved": 20,
            "code_review_commit_approved": 21,
            "membership_request_accepted": 22,
            "membership_request_consolidated": 23,
            "bookmark_reminder": 24,
            "reaction": 25,
            "votes_released": 26,
            "event_reminder": 27,
            "event_invitation": 28,
        },
        "post_types": {
            "regular": 1,
            "moderator_action": 2,
            "small_action": 3,
            "whisper": 4,
        },
        "trust_levels": {
            "newuser": 0,
            "basic": 1,
            "member": 2,
            "regular": 3,
            "leader": 4,
        },
        "categories": category_list,
        "can_create_tag": false,
        "topic_flag_types": {
            "off_topic": 3,
            "inappropriate": 4,
            "spam": 8,
            "notify_user": 6,
            "notify_moderators": 7,
        },
        "uncategorized_category_id": 1,
    }))
}
