use crate::{
    application::services::user_service::UserService,
    infrastructure::database::db_connection::{create_pg_pool, get_pg_pool},
};
use axum::{http, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupInput {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SigninInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SigninOutput {
    pub token: String,
}

pub async fn signup(Json(payload): Json<SignupInput>) -> impl IntoResponse {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url);
    let mut conn = get_pg_pool(&pool);
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let mut user_service = UserService::new(&mut conn, &jwt_secret);

    match user_service.signup(payload.email, payload.nickname, payload.password) {
        Ok(user) => (
            http::StatusCode::CREATED,
            format!("User {} created", user.nickname),
        ),
        Err(err) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", err),
        ),
    }
}

pub async fn signin(Json(payload): Json<SigninInput>) -> impl IntoResponse {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url);
    let mut conn = get_pg_pool(&pool);
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let mut user_service = UserService::new(&mut conn, &jwt_secret);

    match user_service.signin(payload.email, payload.password) {
        Ok(token) => (http::StatusCode::OK, Ok(Json(SigninOutput { token }))),
        Err(err) => (http::StatusCode::UNAUTHORIZED, Err(err.to_string())),
    }
}
