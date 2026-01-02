use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct BlogPostSummary {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
}
