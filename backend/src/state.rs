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
    pub fn new(config: Config, pool: DbPool, firebase_auth: Arc<FirebaseAuth>) -> Self {
        Self {
            config,
            pool,
            firebase_auth,
        }
    }
}
