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

    // Build the connection URL (without specifying the database name initially)
    let connection_url = format!(
        "mysql://{}:{}@{}:{}/",
        user, encoded_password, host, port
    );

    // Create a connection pool without specifying a database
    let pool_without_db = MySqlPool::connect(&connection_url).await?;

    // Check if the specified database exists
    let database_exists: Option<i32> = sqlx::query_scalar(&format!("SELECT 1 FROM information_schema.schemata WHERE schema_name = '{}'", database_name))
        .fetch_optional(&pool_without_db)
        .await
        .unwrap_or_default();

    if database_exists.unwrap_or_default() == 0 {
        // If the database does not exist, create it
        sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS {}", database_name))
            .execute(&pool_without_db)
            .await?;
    }

    // Build the full database URL now that the database exists
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, encoded_password, host, port, database_name
    );

    // Create a connection pool with the specified database
    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}
