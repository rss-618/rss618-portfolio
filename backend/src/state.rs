use std::sync::Arc;

use crate::config::Config;
use crate::db::DbPool;
use crate::services::FirebaseAuth;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: DbPool,
    pub firebase_auth: Arc<FirebaseAuth>,
}

impl AppState {
    pub fn new(config: Config, pool: DbPool) -> Self {
        let firebase_auth = Arc::new(FirebaseAuth::new(
            config.firebase_project_id.clone(),
            config.firebase_api_key.clone(),
        ));

        Self {
            config,
            pool,
            firebase_auth,
        }
    }
}
