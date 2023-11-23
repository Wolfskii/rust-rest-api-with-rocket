use sqlx::mysql::MySqlPool;
use sqlx::Error;
use std::env;
use urlencoding::encode;

pub async fn init_pool() -> Result<MySqlPool, Error> {
    dotenv::dotenv().ok();

    // Parse individual components from DATABASE_URL
    let host = env::var("DATABASE_HOST").expect("DATABASE_HOST not set");
    let port = env::var("DATABASE_PORT").expect("DATABASE_PORT not set");
    let user = env::var("DATABASE_USER").expect("DATABASE_USER not set");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD not set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set");

    // URL-encode the password
    let encoded_password = encode(&password);

    // Build the database URL
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, encoded_password, host, port, database_name
    );

    // Create a connection pool
    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}
