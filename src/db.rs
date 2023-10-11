use sqlx::mysql::MySqlPool;
use sqlx::Error;
use std::env;

pub async fn init_pool() -> Result<MySqlPool, Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Create a connection pool
    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}
