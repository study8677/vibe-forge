use axum::{extract::FromRequestParts, http::request::Parts};
use stevessr_core::traits::guardian::CurrentUser;
use crate::state::AppState;

pub struct AuthenticatedUser(pub CurrentUser);
pub struct OptionalUser(pub Option<CurrentUser>);

impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract auth token from cookie or header
        let _auth_header = parts.headers.get("authorization");
        let _cookie = parts.headers.get("cookie");
        // TODO: validate token against user_auth_tokens table
        Ok(OptionalUser(None))
    }
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let OptionalUser(user) = OptionalUser::from_request_parts(parts, state).await.unwrap();
        user.map(AuthenticatedUser).ok_or(axum::http::StatusCode::UNAUTHORIZED)
    }
}
