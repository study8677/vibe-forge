use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;

#[derive(Debug, Deserialize)]
pub struct ReportParams {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub group_id: Option<i64>,
    #[serde(default)]
    pub facets: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
}

/// GET /admin/reports/{type} - Fetch a specific report
///
/// Report types: visits, signups, topics, posts, likes, flags,
/// bookmarks, emails, user_to_user_private_messages_with_replies,
/// dau_by_mau, daily_engaged_users, new_contributors, etc.
pub async fn show(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(report_type): Path<String>,
    Query(params): Query<ReportParams>,
) -> Result<Json<Value>, StatusCode> {
    if !user.0.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let report = stevessr_services::admin::reports::generate_report(
        &state.db,
        &report_type,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
        params.category_id,
        params.group_id,
        params.limit,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "report": {
            "type": report.report_type,
            "title": report.title,
            "xaxis": report.xaxis,
            "yaxis": report.yaxis,
            "description": report.description,
            "data": report.data,
            "total": report.total,
            "prev30Days": report.prev_30_days,
            "prev_period": report.prev_period,
            "start_date": report.start_date,
            "end_date": report.end_date,
            "category_id": report.category_id,
            "group_id": report.group_id,
            "prev_data": report.prev_data,
            "prev_start_date": report.prev_start_date,
            "prev_end_date": report.prev_end_date,
            "labels": report.labels,
            "average": report.average,
            "percent": report.percent,
            "higher_is_better": report.higher_is_better,
            "modes": report.modes,
        }
    })))
}
