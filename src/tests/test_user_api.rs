use crate::create_app;
use axum::http::StatusCode;
use axum_test_helper::TestClient;

#[tokio::test]
async fn test_signin() {
    dotenvy::dotenv().ok();
    let app = create_app().await;
    let client = TestClient::new(app);

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
