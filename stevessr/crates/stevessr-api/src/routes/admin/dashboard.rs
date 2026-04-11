use axum::{extract::State, Json, http::StatusCode};
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;

/// GET /admin/dashboard - Admin dashboard overview
pub async fn index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let stats = stevessr_services::admin::dashboard::get_dashboard_stats(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "global_reports": [
            {
                "type": "visits",
                "title": "Page Views",
                "xaxis": "Day",
                "yaxis": "Views",
                "data": stats.page_view_data,
                "total": stats.total_page_views,
                "prev30Days": stats.prev_30_days_page_views,
            },
            {
                "type": "signups",
                "title": "Signups",
                "data": stats.signup_data,
                "total": stats.total_signups,
                "prev30Days": stats.prev_30_days_signups,
            },
            {
                "type": "topics",
                "title": "New Topics",
                "data": stats.topic_data,
                "total": stats.total_topics,
                "prev30Days": stats.prev_30_days_topics,
            },
            {
                "type": "posts",
                "title": "New Posts",
                "data": stats.post_data,
                "total": stats.total_posts,
                "prev30Days": stats.prev_30_days_posts,
            },
            {
                "type": "dau_by_mau",
                "title": "DAU/MAU",
                "data": stats.dau_by_mau_data,
            },
            {
                "type": "daily_engaged_users",
                "title": "Daily Engaged Users",
                "data": stats.daily_engaged_users_data,
            },
            {
                "type": "new_contributors",
                "title": "New Contributors",
                "data": stats.new_contributors_data,
            },
        ],
        "updated_at": stats.updated_at,
        "top_referred_topics": stats.top_referred_topics,
        "top_traffic_sources": stats.top_traffic_sources,
        "top_users": stats.top_users,
        "activity_metrics": {
            "active_users_last_day": stats.active_users_last_day,
            "active_users_7_days": stats.active_users_7_days,
            "active_users_30_days": stats.active_users_30_days,
        },
        "moderator_activity": stats.moderator_activity,
    })))
}
