use crate::application::services::user_service::UserService;
use crate::presentation::extractors::{CurrentUser, DatabaseConnection};
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

pub async fn signup(
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<SignupInput>,
) -> impl IntoResponse {
    let mut user_service = UserService::new(conn).await;

    match user_service
        .signup(payload.email, payload.nickname, payload.password)
        .await
    {
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

pub async fn signin(
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<SigninInput>,
) -> impl IntoResponse {
    let mut user_service = UserService::new(conn).await;

    match user_service.signin(payload.email, payload.password).await {
        Ok(token) => (http::StatusCode::OK, Ok(Json(SigninOutput { token }))),
        Err(err) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Err(err.to_string()),
        ),
    }
}

pub async fn me(
    DatabaseConnection(conn): DatabaseConnection,
    CurrentUser(user): CurrentUser,
) -> impl IntoResponse {
    let mut user_service = UserService::new(conn).await;

    match user_service.get_me(&user.id).await {
        Ok(me) => (http::StatusCode::OK, Ok(Json(me))),
        Err(err) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Err(err.to_string()),
        ),
    }
}
