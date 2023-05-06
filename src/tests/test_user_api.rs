use axum::{http::StatusCode, routing::post};
use axum_test_helper::TestClient;

use crate::presentation::restapi::user_routers::{signin, signup};

#[tokio::test]
async fn test_signin() {
    dotenvy::dotenv().ok();
    let router = axum::Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin));
    let client = TestClient::new(router);

    let input = serde_json::json!({
        "email": "user@test.co.kr",
        "nickname": "user",
        "password": "passwordr"
    });
    let res = client.post("/signup").json(&input).send().await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let input = serde_json::json!({
        "email": "user@test.co.kr",
        "password": "passwordr"
    });
    let res = client.post("/signin").json(&input).send().await;
    assert_eq!(res.status(), StatusCode::OK);
}
