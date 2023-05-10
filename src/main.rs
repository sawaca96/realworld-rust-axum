mod application;
mod domain;
mod infrastructure;
mod presentation;
mod schema;
#[cfg(test)]
mod tests {
    mod test_user_api;
}

use std::net::SocketAddr;

use crate::presentation::restapi::user_routers::{signin, signup};
use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use infrastructure::database::db_connection::create_pg_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = create_app().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_app() -> Router {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url).await;
    axum::Router::new()
        .route("/ping", get(ping))
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .with_state(pool)
}

async fn ping() -> &'static str {
    "pong"
}
