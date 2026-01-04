use std::sync::Arc;

use crate::config::Config;
use crate::db::DbPool;
use crate::repositories::BlogRepository;
use crate::services::{BlogService, FirebaseAuthService};

#[derive(Clone)]
pub struct AppState {
    pub firebase_auth: Arc<FirebaseAuthService>,
    pub blog_service: Arc<BlogService>,
}

impl AppState {
    pub fn new(config: Config, pool: DbPool) -> Self {
        let firebase_auth = Arc::new(FirebaseAuthService::new(
            config.firebase_project_id,
            config.firebase_api_key,
        ));
        let blog_service = Arc::new(BlogService::new(BlogRepository::new(pool)));
        Self {
            firebase_auth,
            blog_service,
        }
    }
}
