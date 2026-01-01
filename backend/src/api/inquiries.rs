use axum::{http::StatusCode, routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/inquiries", get(get_inquiries))
}

// TODO: Implement actual inquiry fetching from database
async fn get_inquiries() -> StatusCode {
    StatusCode::OK
}
