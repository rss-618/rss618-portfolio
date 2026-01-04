mod config;
mod dao;
mod db;
mod dto;
mod grpc;
mod middleware;
mod repositories;
mod services;
mod state;
mod utils;

use http::header::HeaderName;
use proto::auth::auth_service_server::AuthServiceServer;
use proto::blog::blog_admin_service_server::BlogAdminServiceServer;
use proto::blog::blog_service_server::BlogServiceServer;
use tonic::service::Routes;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;
use crate::grpc::{AuthController, BlogAdminController, BlogController};
use crate::middleware::auth_interceptor;
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
    let cors_origin = config.cors_origin.clone();

    let pool = db::init(&config.database_path).await;
    let state = AppState::new(config, pool);

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(cors_origin.parse().unwrap()))
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("connect-protocol-version"),
            HeaderName::from_static("connect-timeout-ms"),
            HeaderName::from_static("grpc-timeout"),
            HeaderName::from_static("x-grpc-web"),
            HeaderName::from_static("x-user-agent"),
        ])
        .expose_headers([
            HeaderName::from_static("grpc-status"),
            HeaderName::from_static("grpc-message"),
            HeaderName::from_static("grpc-status-details-bin"),
        ])
        .allow_methods([http::Method::POST])
        .max_age(std::time::Duration::from_secs(7200))
        .allow_credentials(true);

    // Pre-fetch Firebase public keys at startup
    if let Err(e) = state.firebase_auth.prefetch_keys().await {
        tracing::warn!("Failed to prefetch Firebase keys: {:?}", e);
    }

    let routes = Routes::new(AuthServiceServer::new(AuthController::new(state.clone())))
        .add_service(BlogServiceServer::new(BlogController::new(state.clone())))
        .add_service(BlogAdminServiceServer::with_interceptor(
            BlogAdminController::new(state.clone()),
            auth_interceptor(state.clone()),
        ));

    tracing::info!("gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_routes(routes)
        .serve(addr)
        .await
        .unwrap();
}
