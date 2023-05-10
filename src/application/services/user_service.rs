use crate::domain::interfaces::user_repository::IUserRepository;
use crate::domain::models::user::{Me, User};
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::{
    application::utils::auth::encode_jwt, infrastructure::database::db_connection::PgConnection,
};
use argon2::{self, Config};
use uuid::Uuid;

use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JWTError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum UserServiceError {
    Diesel(DieselError),
    JWT(JWTError),
    InvalidCredentials,
}

impl std::error::Error for UserServiceError {}

impl Display for UserServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            UserServiceError::Diesel(err) => write!(f, "Diesel error: {}", err),
            UserServiceError::JWT(err) => write!(f, "JWT error: {}", err),
            UserServiceError::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

impl From<DieselError> for UserServiceError {
    fn from(err: DieselError) -> UserServiceError {
        UserServiceError::Diesel(err)
    }
}

impl From<JWTError> for UserServiceError {
    fn from(err: JWTError) -> UserServiceError {
        UserServiceError::JWT(err)
    }
}

pub struct UserService {
    user_repository: UserRepository,
    argon2_config: Config<'static>,
}

impl UserService {
    pub async fn new(conn: PgConnection) -> Self {
        UserService {
            user_repository: UserRepository::new(conn),
            argon2_config: Config::default(),
        }
    }

    fn hash_password(&self, password: &str) -> Result<String, argon2::Error> {
        let salt = b"randomsalt"; // You should use a unique salt per user
        let hash = argon2::hash_encoded(password.as_bytes(), salt, &self.argon2_config)?;
        Ok(hash)
    }

    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, argon2::Error> {
        let valid = argon2::verify_encoded(hash, password.as_bytes())?;
        Ok(valid)
    }

    pub async fn signup(
        &mut self,
        email: String,
        nickname: String,
        password: String,
    ) -> Result<User, diesel::result::Error> {
        let hashed_password = self.hash_password(&password).unwrap();
        let new_user = User {
            id: Uuid::new_v4(),
            email,
            nickname,
            password: hashed_password,
        };

        self.user_repository.create(&new_user).await
    }

    pub async fn signin(
        &mut self,
        email: String,
        password: String,
    ) -> Result<String, UserServiceError> {
        let user = self.user_repository.get_user_by_email(&email).await?;

        if self.verify_password(&user.password, &password).unwrap() {
            let token = encode_jwt(user)?;
            Ok(token)
        } else {
            Err(UserServiceError::InvalidCredentials)
        }
    }

    pub async fn get_me(&mut self, id: &Uuid) -> Result<Me, diesel::result::Error> {
        self.user_repository.get_me(id).await
    }
}
