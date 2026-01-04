use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::error::ErrorInternalServerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub id: i32,     // user_id
    pub exp: usize,
}

pub fn sign_token(id: i32, username: &str) -> Result<String, actix_web::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 60 * 60 * 24; // 24 hours

    let claims = Claims {
        sub: username.to_string(),
        id,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ErrorInternalServerError(e.to_string()))
}
