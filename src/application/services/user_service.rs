use crate::domain::interfaces::user_repository::IUserRepository;
use crate::domain::models::user::User;
use crate::infrastructure::repositories::user_repository::UserRepository;
use argon2::{self, Config};
use diesel::prelude::*;
use jsonwebtoken::errors::Result as JWTResult;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
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
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct UserService<'a> {
    user_repository: UserRepository<'a>,
    jwt_secret: &'a str,
    argon2_config: Config<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a mut PgConnection, jwt_secret: &'a str) -> Self {
        UserService {
            user_repository: UserRepository::new(conn),
            jwt_secret,
            argon2_config: Config::default(),
        }
    }

    fn generate_jwt(&self, user_id: Uuid) -> JWTResult<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + 86400, // 1 day
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
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
    pub fn signup(
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

        self.user_repository.create(&new_user)
    }

    pub fn signin(&mut self, email: String, password: String) -> Result<String, UserServiceError> {
        let user = self.user_repository.find_by_email(&email)?;

        if self.verify_password(&user.password, &password).unwrap() {
            let token = self.generate_jwt(user.id)?;
            Ok(token)
        } else {
            Err(UserServiceError::InvalidCredentials)
        }
    }
}
