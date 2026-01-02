use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use proto::blog::{
    BlogPost, BlogPostSummary, CreateBlogPostRequest, CreateBlogPostResponse,
    DeleteBlogPostResponse, GetBlogPostResponse, GetBlogPostsRequest, GetBlogPostsResponse,
    RestoreBlogPostResponse, UpdateBlogPostRequest, UpdateBlogPostResponse,
};

use crate::state::AppState;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/blog", post(list_posts))
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
    Json(req): Json<GetBlogPostsRequest>,
) -> Result<Json<GetBlogPostsResponse>, StatusCode> {
    let (posts, total) = state
        .blog_service
        .list(req.query.as_deref(), req.limit, req.offset, req.sort.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let posts = posts
        .into_iter()
        .map(|p| BlogPostSummary {
            id: p.id,
            title: p.title,
            description: p.description,
            created_at: p.created_at,
            updated_at: p.updated_at,
        })
        .collect();

    Ok(Json(GetBlogPostsResponse { posts, total }))
}

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<GetBlogPostResponse>, StatusCode> {
    let post = state
        .blog_service
        .get(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post = post.map(|p| BlogPost {
        id: p.id,
        title: p.title,
        description: p.description,
        body: p.body,
        created_at: p.created_at,
        updated_at: p.updated_at,
    });

    Ok(Json(GetBlogPostResponse { post }))
}

async fn create_post(
    State(state): State<AppState>,
    Json(req): Json<CreateBlogPostRequest>,
) -> Result<Json<CreateBlogPostResponse>, StatusCode> {
    let id = state
        .blog_service
        .create(&req.title, &req.description, &req.body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateBlogPostResponse { id }))
}

async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateBlogPostRequest>,
) -> Result<Json<UpdateBlogPostResponse>, StatusCode> {
    let updated = state
        .blog_service
        .update(
            id,
            req.title.as_deref(),
            req.description.as_deref(),
            req.body.as_deref(),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if updated {
        Ok(Json(UpdateBlogPostResponse {}))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<DeleteBlogPostResponse>, StatusCode> {
    let deleted = state
        .blog_service
        .delete(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if deleted {
        Ok(Json(DeleteBlogPostResponse {}))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn restore_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<RestoreBlogPostResponse>, StatusCode> {
    let restored = state
        .blog_service
        .restore(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if restored {
        Ok(Json(RestoreBlogPostResponse {}))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
