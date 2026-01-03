use proto::blog::blog_admin_service_server::BlogAdminService as BlogAdminServiceTrait;
use proto::blog::blog_service_server::BlogService as BlogServiceTrait;
use proto::blog::{
    BlogPost, BlogPostSummary, CreateBlogPostRequest, CreateBlogPostResponse,
    DeleteBlogPostRequest, DeleteBlogPostResponse, GetBlogPostRequest, GetBlogPostResponse,
    GetBlogPostsRequest, GetBlogPostsResponse, RestoreBlogPostRequest, RestoreBlogPostResponse,
    UpdateBlogPostRequest, UpdateBlogPostResponse,
};
use tonic::{Request, Response, Status};

use crate::dao::blog::BlogPostSort;
use crate::state::AppState;

/// gRPC controller for the public BlogService (read operations).
pub struct BlogController {
    state: AppState,
}

impl BlogController {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl BlogServiceTrait for BlogController {
    async fn get_blog_posts(
        &self,
        request: Request<GetBlogPostsRequest>,
    ) -> Result<Response<GetBlogPostsResponse>, Status> {
        let req = request.into_inner();
        let sort: BlogPostSort = req.sort.into();

        let (posts, total) = self
            .state
            .blog_service
            .list(req.query.as_deref(), req.limit, req.offset, sort)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

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

        Ok(Response::new(GetBlogPostsResponse { posts, total }))
    }

    async fn get_blog_post(
        &self,
        request: Request<GetBlogPostRequest>,
    ) -> Result<Response<GetBlogPostResponse>, Status> {
        let req = request.into_inner();

        let post = self
            .state
            .blog_service
            .get(req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let post = post.map(|p| BlogPost {
            id: p.id,
            title: p.title,
            description: p.description,
            body: p.body,
            created_at: p.created_at,
            updated_at: p.updated_at,
        });

        Ok(Response::new(GetBlogPostResponse { post }))
    }
}

/// gRPC controller for the protected BlogAdminService (write operations).
/// Requires authentication via AuthMiddleware.
pub struct BlogAdminController {
    state: AppState,
}

impl BlogAdminController {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl BlogAdminServiceTrait for BlogAdminController {
    async fn create_blog_post(
        &self,
        request: Request<CreateBlogPostRequest>,
    ) -> Result<Response<CreateBlogPostResponse>, Status> {
        let req = request.into_inner();

        let id = self
            .state
            .blog_service
            .create(&req.title, &req.description, &req.body)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateBlogPostResponse { id }))
    }

    async fn update_blog_post(
        &self,
        request: Request<UpdateBlogPostRequest>,
    ) -> Result<Response<UpdateBlogPostResponse>, Status> {
        let req = request.into_inner();

        let updated = self
            .state
            .blog_service
            .update(
                req.id,
                req.title.as_deref(),
                req.description.as_deref(),
                req.body.as_deref(),
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if updated {
            Ok(Response::new(UpdateBlogPostResponse {}))
        } else {
            Err(Status::not_found("Blog post not found"))
        }
    }

    async fn delete_blog_post(
        &self,
        request: Request<DeleteBlogPostRequest>,
    ) -> Result<Response<DeleteBlogPostResponse>, Status> {
        let req = request.into_inner();

        let deleted = self
            .state
            .blog_service
            .delete(req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if deleted {
            Ok(Response::new(DeleteBlogPostResponse {}))
        } else {
            Err(Status::not_found("Blog post not found"))
        }
    }

    async fn restore_blog_post(
        &self,
        request: Request<RestoreBlogPostRequest>,
    ) -> Result<Response<RestoreBlogPostResponse>, Status> {
        let req = request.into_inner();

        let restored = self
            .state
            .blog_service
            .restore(req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if restored {
            Ok(Response::new(RestoreBlogPostResponse {}))
        } else {
            Err(Status::not_found("Blog post not found"))
        }
    }
}
