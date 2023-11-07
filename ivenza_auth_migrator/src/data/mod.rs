use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

pub async fn establish_connection() -> MySqlPool {
    // Get the database url from environment variables.
    let database_url = env::var(DATABASE_URL_KEY).expect("Database URL not set.");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.expect(&format!("Error connecting to {}", database_url));
    pool
}
