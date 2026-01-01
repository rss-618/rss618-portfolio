mod api;
mod config;
mod db;
mod dto;
mod middleware;
mod repositories;
mod services;
mod state;

use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::services::FirebaseAuth;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let addr = config.socket_addr();

    let pool = db::connect(&config.database_path).await;
    db::run_migrations(&pool).await;

    let firebase_auth = Arc::new(FirebaseAuth::new(config.firebase_project_id.clone()));

    let cors = CorsLayer::new()
        .allow_origin(config.cors_origin.parse::<http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let state = AppState::new(config, pool, firebase_auth);

    let app = api::router().layer(cors).with_state(state);

    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
