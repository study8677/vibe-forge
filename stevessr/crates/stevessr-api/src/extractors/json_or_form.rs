use axum::{extract::Request, body::Bytes, http::header::CONTENT_TYPE};
use serde::de::DeserializeOwned;

pub struct JsonOrForm<T>(pub T);

impl<S, T> axum::extract::FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req.headers().get(CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("");

        if content_type.contains("application/json") {
            let bytes = Bytes::from_request(req, state).await.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
            let value = serde_json::from_slice(&bytes).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
            Ok(JsonOrForm(value))
        } else {
            let bytes = Bytes::from_request(req, state).await.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
            let value = serde_urlencoded::from_bytes(&bytes).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
            Ok(JsonOrForm(value))
        }
    }
}
