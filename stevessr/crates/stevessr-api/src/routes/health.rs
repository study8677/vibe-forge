use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

/// GET /srv/status - Health check endpoint
pub async fn status(
    State(state): State<AppState>,
) -> Json<Value> {
    // Verify database connectivity
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    // Verify Redis connectivity
    let redis_ok = {
        let mut conn = state.redis.clone();
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .is_ok()
    };

    Json(json!({
        "status": if db_ok && redis_ok { "ok" } else { "degraded" },
        "database": if db_ok { "connected" } else { "error" },
        "redis": if redis_ok { "connected" } else { "error" },
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
