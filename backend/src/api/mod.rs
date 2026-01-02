mod auth;
mod blog;
mod health;
mod inquiries;

use axum::{middleware, Router};
use http::header::{HeaderName, AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{middleware::require_auth, state::AppState};

/// Header name for request correlation ID
const X_REQUEST_ID: &str = "x-request-id";

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

    let x_request_id = HeaderName::from_static(X_REQUEST_ID);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .level(Level::INFO)
                .include_headers(true),
        )
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let protected = Router::new()
        .merge(inquiries::router())
        .merge(blog::protected_router())
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .merge(auth::router())
        .merge(health::router())
        .merge(blog::public_router())
        .merge(protected)
        .layer(cors)
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
        .with_state(state)
}
