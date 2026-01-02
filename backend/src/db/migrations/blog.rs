use crate::db::DbPool;

pub async fn migrate(pool: &DbPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS blog_posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            deleted_at INTEGER
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create blog_posts table");

    sqlx::query(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS blog_posts_fts USING fts5(
            title,
            description,
            body,
            content='blog_posts',
            content_rowid='id',
            tokenize='porter'
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create blog_posts_fts table");

    // Set default bm25 weights: title=10.0, description=1.0, body=1.0
    sqlx::query(
        r#"
        INSERT INTO blog_posts_fts(blog_posts_fts, rank) VALUES('rank', 'bm25(10.0, 1.0, 1.0)')
        "#,
    )
    .execute(pool)
    .await
    .ok();

    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS blog_posts_ai AFTER INSERT ON blog_posts BEGIN
            INSERT INTO blog_posts_fts(rowid, title, description, body) VALUES (new.id, new.title, new.description, new.body);
        END
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create blog_posts insert trigger");

    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS blog_posts_ad AFTER DELETE ON blog_posts BEGIN
            INSERT INTO blog_posts_fts(blog_posts_fts, rowid, title, description, body) VALUES ('delete', old.id, old.title, old.description, old.body);
        END
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create blog_posts delete trigger");

    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS blog_posts_au AFTER UPDATE ON blog_posts BEGIN
            INSERT INTO blog_posts_fts(blog_posts_fts, rowid, title, description, body) VALUES ('delete', old.id, old.title, old.description, old.body);
            INSERT INTO blog_posts_fts(rowid, title, description, body) VALUES (new.id, new.title, new.description, new.body);
        END
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create blog_posts update trigger");

    // Index for soft delete filtering
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_blog_posts_deleted_at ON blog_posts(deleted_at)")
        .execute(pool)
        .await
        .expect("Failed to create blog_posts deleted_at index");
}
