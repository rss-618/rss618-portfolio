mod api;
mod config;
mod db;
mod dto;
mod middleware;
mod repositories;
mod services;
mod state;
mod utils;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let addr = config.socket_addr();

    let pool = db::init(&config.database_path).await;
    let state = AppState::new(config, pool);
    let app = api::router(state);

    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
