use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

pub fn establish_connection() -> MysqlConnection {
    // Get the database url from environment variables.
    let database_url = env::var(DATABASE_URL_KEY).expect("Database URL not set.");

    // Create a new MySqlConnection (maria db is compatible). If this fails, print an error message.
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
