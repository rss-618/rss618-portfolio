use std::sync::Arc;

use crate::config::Config;
use crate::db::DbPool;
use crate::repositories::BlogRepository;
use crate::services::{BlogService, FirebaseAuth};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub firebase_auth: Arc<FirebaseAuth>,
    pub blog_service: Arc<BlogService>,
}

impl AppState {
    pub fn new(config: Config, pool: DbPool) -> Self {
        let firebase_auth = Arc::new(FirebaseAuth::new(
            config.firebase_project_id.clone(),
            config.firebase_api_key.clone(),
        ));

        let blog_service = Arc::new(BlogService::new(BlogRepository::new(pool)));

        Self {
            config,
            firebase_auth,
            blog_service,
        }
    }
}
