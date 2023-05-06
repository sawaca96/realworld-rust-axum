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
use axum::routing::{get, post};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/signup", post(signup))
        .route("/signin", post(signin));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
