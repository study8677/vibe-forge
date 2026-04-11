use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::pagination::Pagination;

/// GET /chat/channels/{id}/threads - List threads in a channel
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, StatusCode> {
    let threads = stevessr_services::chat::threads::list_threads(
        &state.db,
        id,
        user.0.id,
        pagination.offset(),
        pagination.limit(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let thread_list: Vec<Value> = threads
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "channel_id": t.channel_id,
                "title": t.title,
                "status": t.status,
                "original_message_id": t.original_message_id,
                "original_message": {
                    "id": t.original_message.id,
                    "message": t.original_message.message,
                    "cooked": t.original_message.cooked,
                    "created_at": t.original_message.created_at,
                    "user": {
                        "id": t.original_message.user_id,
                        "username": t.original_message.username,
                        "avatar_template": t.original_message.avatar_template,
                    },
                },
                "reply_count": t.reply_count,
                "last_reply_created_at": t.last_reply_created_at,
                "participant_count": t.participant_count,
                "participants": t.participants,
                "current_user_membership": {
                    "notification_level": t.notification_level,
                    "last_read_message_id": t.last_read_message_id,
                },
            })
        })
        .collect();

    Ok(Json(json!({
        "threads": thread_list,
        "meta": {
            "channel_id": id,
            "load_more_url": null,
        }
    })))
}
