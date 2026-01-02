mod migrations;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub type DbPool = SqlitePool;

pub async fn init(database_url: &str) -> DbPool {
    let pool = connect(database_url).await;
    migrations::run(&pool).await;
    pool
}

async fn connect(database_url: &str) -> DbPool {
    let options = SqliteConnectOptions::from_str(database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    SqlitePool::connect_with(options)
        .await
        .expect("Failed to connect to database")
}
