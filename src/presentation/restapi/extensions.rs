use crate::application::services::user_service::UserService;
use crate::application::utils::auth::decode_jwt;
use crate::domain::models::user::Me;
use crate::infrastructure::database::db_connection::{
    create_pg_pool, get_pg_connection, PgPooledConnection,
};

fn db_connection() -> PgPooledConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url);
    get_pg_connection(&pool)
}

fn user_service() -> UserService {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    UserService::new(db_connection())
}

fn current_user(token: &str) -> Me {
    // TODO: 필요시 유저 존재하는지 검증
    let user = decode_jwt(token).unwrap();
    user
}
