use std::env;
use std::sync::Arc;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
pub async fn connect() -> Result<Arc<MySqlPool>, sqlx::Error> {
    // Retrieve the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool with specified options
    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Set the maximum number of connections
        .connect(&database_url) // Connect to the database
        .await?;

    // Return the connection pool wrapped in an Arc for thread safety
    Ok(Arc::new(pool))
}
