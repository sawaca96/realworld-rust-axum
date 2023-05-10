use crate::application::utils::auth::decode_jwt;
use crate::application::utils::response::internal_error;
use crate::domain::models::user::Me;
use crate::infrastructure::database::db_connection::{PgConnection, PgPool};
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
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

fn current_user(token: &str) -> Me {
    // TODO: 필요시 유저 존재하는지 검증
    let user = decode_jwt(token).unwrap();
    user
}
