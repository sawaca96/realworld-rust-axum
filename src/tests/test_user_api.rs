use std::error::Error;

use crate::create_app;
use crate::schema::users::dsl;
use axum::http::StatusCode;
use axum_test_helper::TestClient;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::Deserialize;

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

fn run_migrations(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct SigninResponse {
    token: String,
}
#[tokio::test]
async fn test_signin() {
    dotenvy::dotenv().ok();
    let app = create_app().await;
    let client = TestClient::new(app);
    let mut connection = establish_connection();
    run_migrations(&mut connection).unwrap();

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

    let signin_response = res.json::<SigninResponse>().await;
    let res = client
        .get("/me")
        .header(
            "Authorization",
            "Bearer ".to_string() + &signin_response.token,
        )
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    diesel::delete(dsl::users).execute(&mut connection).unwrap();
}
