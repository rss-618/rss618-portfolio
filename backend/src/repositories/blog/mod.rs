use std::time::{SystemTime, UNIX_EPOCH};

use crate::dao::blog::{BlogPost, BlogPostSort, BlogPostSummary};
use crate::db::DbPool;

pub struct BlogRepository {
    pool: DbPool,
}

impl BlogRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list(
        &self,
        query: Option<&str>,
        limit: i32,
        offset: i32,
        sort: BlogPostSort,
    ) -> Result<(Vec<BlogPostSummary>, i32), sqlx::Error> {
        let limit = if limit <= 0 { 10 } else { limit };

        let (posts, total) = if let Some(q) = query.filter(|s| !s.is_empty()) {
            let order_by = match sort {
                BlogPostSort::Relevance => "rank",
                BlogPostSort::CreatedAsc => "bp.created_at ASC",
                BlogPostSort::CreatedDesc => "bp.created_at DESC",
                BlogPostSort::UpdatedAsc => "bp.updated_at ASC",
                BlogPostSort::UpdatedDesc => "bp.updated_at DESC",
            };

            let sql = format!(
                r#"
                SELECT bp.id, bp.title, bp.description, bp.created_at, bp.updated_at
                FROM blog_posts_fts fts
                INNER JOIN blog_posts bp ON bp.id = fts.rowid
                WHERE blog_posts_fts MATCH ?
                  AND bp.deleted_at IS NULL
                ORDER BY {order_by}
                LIMIT ? OFFSET ?
                "#
            );

            let posts = sqlx::query_as::<_, BlogPostSummary>(&sql)
                .bind(q)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

            let total: i32 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM blog_posts_fts fts
                INNER JOIN blog_posts bp ON bp.id = fts.rowid
                WHERE blog_posts_fts MATCH ?
                  AND bp.deleted_at IS NULL
                "#,
            )
            .bind(q)
            .fetch_one(&self.pool)
            .await?;

            (posts, total)
        } else {
            // Without a search query, relevance doesn't apply - default to created_at DESC
            let order_by = match sort {
                BlogPostSort::Relevance | BlogPostSort::CreatedDesc => "created_at DESC",
                BlogPostSort::CreatedAsc => "created_at ASC",
                BlogPostSort::UpdatedAsc => "updated_at ASC",
                BlogPostSort::UpdatedDesc => "updated_at DESC",
            };

            let sql = format!(
                r#"
                SELECT id, title, description, created_at, updated_at
                FROM blog_posts
                WHERE deleted_at IS NULL
                ORDER BY {order_by}
                LIMIT ? OFFSET ?
                "#
            );

            let posts = sqlx::query_as::<_, BlogPostSummary>(&sql)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

            let total: i32 =
                sqlx::query_scalar("SELECT COUNT(*) FROM blog_posts WHERE deleted_at IS NULL")
                    .fetch_one(&self.pool)
                    .await?;

            (posts, total)
        };

        Ok((posts, total))
    }

    pub async fn get(&self, id: i64) -> Result<Option<BlogPost>, sqlx::Error> {
        sqlx::query_as::<_, BlogPost>(
            r#"
            SELECT id, title, description, body, created_at, updated_at, deleted_at
            FROM blog_posts
            WHERE id = ? AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(
        &self,
        title: &str,
        description: &str,
        body: &str,
    ) -> Result<i64, sqlx::Error> {
        let now = now_timestamp();

        let result = sqlx::query(
            r#"
            INSERT INTO blog_posts (title, description, body, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(body)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn update(
        &self,
        id: i64,
        title: Option<&str>,
        description: Option<&str>,
        body: Option<&str>,
    ) -> Result<bool, sqlx::Error> {
        let now = now_timestamp();

        let current = self.get(id).await?;
        let Some(current) = current else {
            return Ok(false);
        };

        let title = title.unwrap_or(&current.title);
        let description = description.unwrap_or(&current.description);
        let body = body.unwrap_or(&current.body);

        let result = sqlx::query(
            r#"
            UPDATE blog_posts
            SET title = ?, description = ?, body = ?, updated_at = ?
            WHERE id = ? AND deleted_at IS NULL
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(body)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let now = now_timestamp();

        let result = sqlx::query(
            r#"
            UPDATE blog_posts
            SET deleted_at = ?, updated_at = ?
            WHERE id = ? AND deleted_at IS NULL
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn restore(&self, id: i64) -> Result<bool, sqlx::Error> {
        let now = now_timestamp();

        let result = sqlx::query(
            r#"
            UPDATE blog_posts
            SET deleted_at = NULL, updated_at = ?
            WHERE id = ? AND deleted_at IS NOT NULL
            "#,
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
