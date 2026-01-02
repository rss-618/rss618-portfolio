use proto::blog::get_blog_posts_request::Sort;

#[derive(Debug, Clone, Copy, Default)]
pub enum BlogPostSort {
    #[default]
    Relevance,
    CreatedAsc,
    CreatedDesc,
    UpdatedAsc,
    UpdatedDesc,
}

impl From<i32> for BlogPostSort {
    fn from(value: i32) -> Self {
        match Sort::try_from(value).unwrap_or(Sort::Relevance) {
            Sort::Relevance => Self::Relevance,
            Sort::CreatedAsc => Self::CreatedAsc,
            Sort::CreatedDesc => Self::CreatedDesc,
            Sort::UpdatedAsc => Self::UpdatedAsc,
            Sort::UpdatedDesc => Self::UpdatedDesc,
        }
    }
}
