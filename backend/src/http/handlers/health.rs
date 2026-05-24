use crate::error::AppResult;
use crate::http::AppState;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

pub fn router() -> Router<crate::http::AppState> {
    Router::new().route("/health", get(health))
}

#[derive(Serialize)]
pub struct HealthResponse {
    ok: bool,
    db: bool,
}

pub async fn health(State(state): State<AppState>) -> AppResult<Json<HealthResponse>> {
    let db_ok = sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(state.db.pool())
        .await
        .is_ok();

    Ok(Json(HealthResponse { ok: true, db: db_ok }))
}
