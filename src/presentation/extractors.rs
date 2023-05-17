use crate::application::utils::auth::decode_jwt;
use crate::application::utils::response::internal_error;
use crate::domain::models::user::Me;
use crate::infrastructure::database::db_connection::{PgConnection, PgPool};
use axum::async_trait;
use axum::extract::{FromRef, FromRequest, FromRequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::http::{request::Parts, StatusCode};
pub struct DatabaseConnection(pub PgConnection);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub struct CurrentUser(pub Me);

#[async_trait]
impl<S, B> FromRequest<S, B> for CurrentUser
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(request: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(authorization_header) = request.headers().get(AUTHORIZATION) {
            if let Some(token) = authorization_header
                .to_str()
                .unwrap()
                .strip_prefix("Bearer ")
            {
                let user = decode_jwt(token).unwrap();
                Ok(Self(user))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
