use axum::{extract::{State, Query}, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::state::AppState;
use crate::extractors::current_user::AuthenticatedUser;
use crate::extractors::json_or_form::JsonOrForm;

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub post_id: i64,
    pub poll_name: String,
    pub options: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ToggleStatusRequest {
    pub post_id: i64,
    pub poll_name: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct VotersParams {
    pub post_id: i64,
    pub poll_name: String,
    #[serde(default)]
    pub option_id: Option<String>,
    #[serde(default)]
    pub page: Option<u32>,
}

/// PUT /polls/vote - Cast or update a vote on a poll
pub async fn vote(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<VoteRequest>,
) -> Result<Json<Value>, StatusCode> {
    let poll = stevessr_services::polls::cast_vote(
        &state.db,
        user.0.id,
        params.post_id,
        &params.poll_name,
        &params.options,
    )
    .await
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    let options: Vec<Value> = poll
        .options
        .iter()
        .map(|o| {
            json!({
                "id": o.id,
                "html": o.html,
                "votes": o.votes,
            })
        })
        .collect();

    Ok(Json(json!({
        "poll": {
            "name": poll.name,
            "type": poll.poll_type,
            "status": poll.status,
            "results": poll.results,
            "options": options,
            "voters": poll.voters,
            "chart_type": poll.chart_type,
            "preloaded_voters": poll.preloaded_voters,
        },
        "vote": params.options,
    })))
}

/// PUT /polls/toggle_status - Open or close a poll
pub async fn toggle_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    JsonOrForm(params): JsonOrForm<ToggleStatusRequest>,
) -> Result<Json<Value>, StatusCode> {
    let poll = stevessr_services::polls::toggle_status(
        &state.db,
        user.0.id,
        params.post_id,
        &params.poll_name,
        &params.status,
    )
    .await
    .map_err(|_| StatusCode::FORBIDDEN)?;

    let options: Vec<Value> = poll
        .options
        .iter()
        .map(|o| {
            json!({
                "id": o.id,
                "html": o.html,
                "votes": o.votes,
            })
        })
        .collect();

    Ok(Json(json!({
        "poll": {
            "name": poll.name,
            "type": poll.poll_type,
            "status": poll.status,
            "results": poll.results,
            "options": options,
            "voters": poll.voters,
        }
    })))
}

/// GET /polls/voters - List voters for a poll option
pub async fn voters(
    State(state): State<AppState>,
    Query(params): Query<VotersParams>,
) -> Result<Json<Value>, StatusCode> {
    let page = params.page.unwrap_or(1);

    let voter_list = stevessr_services::polls::get_voters(
        &state.db,
        params.post_id,
        &params.poll_name,
        params.option_id.as_deref(),
        page,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let voters: Vec<Value> = voter_list
        .iter()
        .map(|v| {
            json!({
                "id": v.id,
                "username": v.username,
                "name": v.name,
                "avatar_template": v.avatar_template,
            })
        })
        .collect();

    Ok(Json(json!({
        "voters": voters,
    })))
}
