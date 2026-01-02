use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use proto::blog::{
    CreateBlogPostRequest, CreateBlogPostResponse, DeleteBlogPostResponse, GetBlogPostResponse,
    GetBlogPostsRequest, GetBlogPostsResponse, RestoreBlogPostResponse, UpdateBlogPostRequest,
    UpdateBlogPostResponse,
};

use crate::state::AppState;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/blog", get(list_posts))
        .route("/blog/{id}", get(get_post))
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/blog", post(create_post))
        .route("/blog/{id}", patch(update_post))
        .route("/blog/{id}", delete(delete_post))
        .route("/blog/{id}/restore", post(restore_post))
}

async fn list_posts(
    State(state): State<AppState>,
    Query(req): Query<GetBlogPostsRequest>,
) -> Result<Json<GetBlogPostsResponse>, StatusCode> {
    let response = state
        .blog_service
        .list(&req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(response))
}

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<GetBlogPostResponse>, StatusCode> {
    let response = state
        .blog_service
        .get(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(response))
}

async fn create_post(
    State(state): State<AppState>,
    Json(req): Json<CreateBlogPostRequest>,
) -> Result<Json<CreateBlogPostResponse>, StatusCode> {
    let response = state
        .blog_service
        .create(&req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(response))
}

async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateBlogPostRequest>,
) -> Result<Json<UpdateBlogPostResponse>, StatusCode> {
    let response = state
        .blog_service
        .update(id, &req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match response {
        Some(r) => Ok(Json(r)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<DeleteBlogPostResponse>, StatusCode> {
    let response = state
        .blog_service
        .delete(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match response {
        Some(r) => Ok(Json(r)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn restore_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<RestoreBlogPostResponse>, StatusCode> {
    let response = state
        .blog_service
        .restore(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match response {
        Some(r) => Ok(Json(r)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
