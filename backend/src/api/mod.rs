mod auth;
mod health;
mod inquiries;

use axum::{middleware, Router};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use tower_http::cors::CorsLayer;

use crate::{middleware::require_auth, state::AppState};

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            state
                .config
                .cors_origin
                .parse::<http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let protected = Router::new()
        .merge(inquiries::router())
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .merge(auth::router())
        .merge(health::router())
        .merge(protected)
        .layer(cors)
        .with_state(state)
}
