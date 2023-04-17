// src/test_utils.rs

use crate::infrastructure::database::db_connection::{create_pg_pool, PgPooledConnection};
use dotenvy::dotenv;
use std::env;

pub fn get_test_db_connection() -> PgPooledConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url);
    pool.get()
        .expect("Failed to get a test database connection")
}
