use std::sync::Arc;

use crate::config::Config;
use crate::db::DbPool;
use crate::repositories::BlogRepository;
use crate::services::BlogService;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    // TODO: Add auth back when migrated to gRPC
    pub blog_service: Arc<BlogService>,
}

impl AppState {
    pub fn new(config: Config, pool: DbPool) -> Self {
        let blog_service = Arc::new(BlogService::new(BlogRepository::new(pool)));
        Self {
            config,
            blog_service,
        }
    }
}
