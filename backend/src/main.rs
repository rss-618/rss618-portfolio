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

use proto::auth::auth_service_server::AuthServiceServer;
use proto::blog::blog_admin_service_server::BlogAdminServiceServer;
use proto::blog::blog_service_server::BlogServiceServer;
use tonic::service::Routes;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;
use crate::grpc::{AuthController, BlogAdminController, BlogController};
use crate::middleware::GrpcAuth;
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

    // Public services (no authentication required)
    let public_router = Routes::default()
        .add_service(AuthServiceServer::new(AuthController::new(state.clone())))
        .add_service(BlogServiceServer::new(BlogController::new(state.clone())))
        .into_axum_router();

    // Protected services (authentication required)
    let protected_router = Routes::default()
        .add_service(BlogAdminServiceServer::new(BlogAdminController::new(
            state.clone(),
        )))
        .into_axum_router()
        .layer(AsyncRequireAuthorizationLayer::new(GrpcAuth::new(
            state.clone(),
        )));

    // Combine routes (tonic Router doesn't support multiple add_routes calls)
    let all_routes: Routes = public_router.merge(protected_router).into();

    tracing::info!("gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_routes(all_routes)
        .serve(addr)
        .await
        .unwrap();
}
