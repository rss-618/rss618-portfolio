mod config;
mod dao;
mod db;
mod grpc;
mod repositories;
mod services;
mod state;

// TODO: Migrate these to gRPC (currently broken - proto types don't have serde)
// mod api;
// mod dto;
// mod middleware;
// mod utils;

use proto::blog::blog_service_server::BlogServiceServer;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;
use crate::grpc::BlogController;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("backend=info,tower_http=info")),
        )
        .init();

    let config = Config::from_env();
    let addr = config.socket_addr();

    let pool = db::init(&config.database_path).await;
    let state = AppState::new(config, pool);

    // TODO: Migrate auth and other REST endpoints to gRPC
    tracing::info!("gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(BlogServiceServer::new(BlogController::new(state.clone())))
        .serve(addr)
        .await
        .unwrap();
}
