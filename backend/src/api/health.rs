use axum::{extract::State, routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check(State(_state): State<AppState>) -> &'static str {
    "ok"
}
