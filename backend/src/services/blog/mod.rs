use crate::dao::blog::{BlogPost, BlogPostSummary};
use crate::repositories::BlogRepository;

pub struct BlogService {
    repo: BlogRepository,
}

impl BlogService {
    pub fn new(repo: BlogRepository) -> Self {
        Self { repo }
    }

    pub async fn list(
        &self,
        query: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<(Vec<BlogPostSummary>, i32), sqlx::Error> {
        self.repo.list(query, limit, offset).await
    }

    pub async fn get(&self, id: i64) -> Result<Option<BlogPost>, sqlx::Error> {
        self.repo.get(id).await
    }

    pub async fn create(
        &self,
        title: &str,
        description: &str,
        body: &str,
    ) -> Result<i64, sqlx::Error> {
        self.repo.create(title, description, body).await
    }

    pub async fn update(
        &self,
        id: i64,
        title: Option<&str>,
        description: Option<&str>,
        body: Option<&str>,
    ) -> Result<bool, sqlx::Error> {
        self.repo.update(id, title, description, body).await
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        self.repo.delete(id).await
    }

    pub async fn restore(&self, id: i64) -> Result<bool, sqlx::Error> {
        self.repo.restore(id).await
    }
}
