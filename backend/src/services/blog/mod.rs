use proto::blog::{
    BlogPost, BlogPostSummary, CreateBlogPostRequest, CreateBlogPostResponse,
    DeleteBlogPostResponse, GetBlogPostResponse, GetBlogPostsRequest, GetBlogPostsResponse,
    RestoreBlogPostResponse, UpdateBlogPostRequest, UpdateBlogPostResponse,
};

use crate::dao::blog::{BlogPost as DaoBlogPost, BlogPostSummary as DaoBlogPostSummary};
use crate::repositories::BlogRepository;

pub struct BlogService {
    repo: BlogRepository,
}

impl BlogService {
    pub fn new(repo: BlogRepository) -> Self {
        Self { repo }
    }

    pub async fn list(&self, req: &GetBlogPostsRequest) -> Result<GetBlogPostsResponse, sqlx::Error> {
        let (posts, total): (Vec<DaoBlogPostSummary>, i32) = self
            .repo
            .list(req.query.as_deref(), req.limit, req.offset)
            .await?;

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

        Ok(GetBlogPostsResponse { posts, total })
    }

    pub async fn get(&self, id: i64) -> Result<GetBlogPostResponse, sqlx::Error> {
        let post: Option<DaoBlogPost> = self.repo.get(id).await?;

        let post = post.map(|p| BlogPost {
            id: p.id,
            title: p.title,
            description: p.description,
            body: p.body,
            created_at: p.created_at,
            updated_at: p.updated_at,
        });

        Ok(GetBlogPostResponse { post })
    }

    pub async fn create(&self, req: &CreateBlogPostRequest) -> Result<CreateBlogPostResponse, sqlx::Error> {
        let id = self
            .repo
            .create(&req.title, &req.description, &req.body)
            .await?;

        Ok(CreateBlogPostResponse { id })
    }

    pub async fn update(&self, id: i64, req: &UpdateBlogPostRequest) -> Result<Option<UpdateBlogPostResponse>, sqlx::Error> {
        let updated = self
            .repo
            .update(
                id,
                req.title.as_deref(),
                req.description.as_deref(),
                req.body.as_deref(),
            )
            .await?;

        if updated {
            Ok(Some(UpdateBlogPostResponse {}))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: i64) -> Result<Option<DeleteBlogPostResponse>, sqlx::Error> {
        let deleted = self.repo.delete(id).await?;

        if deleted {
            Ok(Some(DeleteBlogPostResponse {}))
        } else {
            Ok(None)
        }
    }

    pub async fn restore(&self, id: i64) -> Result<Option<RestoreBlogPostResponse>, sqlx::Error> {
        let restored = self.repo.restore(id).await?;

        if restored {
            Ok(Some(RestoreBlogPostResponse {}))
        } else {
            Ok(None)
        }
    }
}
