use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
