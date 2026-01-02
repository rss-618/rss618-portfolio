mod blog;

use super::DbPool;

pub async fn run(pool: &DbPool) {
    blog::migrate(pool).await;
}
