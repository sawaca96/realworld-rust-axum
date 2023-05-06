use crate::domain::models::user::{Me, User};
use jsonwebtoken::errors::Result as JWTResult;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    exp: usize,
    user_id: uuid::Uuid,
    nickname: String,
    email: String,
}

pub fn encode_jwt(user: User) -> JWTResult<String> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let payload = Payload {
        exp: now + 86400,
        user_id: user.id,
        nickname: user.nickname,
        email: user.email,
    };

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<Me, String> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Payload>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );
    match token_data {
        Ok(token_data) => {
            let mut user = Me {
                id: token_data.claims.user_id,
                nickname: token_data.claims.nickname,
                email: token_data.claims.email,
            };
            Ok(user)
        }
        Err(err) => Err(err.to_string()),
    }
}
