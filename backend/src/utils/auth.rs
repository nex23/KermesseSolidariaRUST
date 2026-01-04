use actix_web::{dev::Payload, error::ErrorUnauthorized, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::utils::jwt::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_val) = auth_header {
            if let Ok(auth_str) = auth_val.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(token_data) => {
                            return ready(Ok(AuthenticatedUser {
                                id: token_data.claims.id,
                                username: token_data.claims.sub,
                            }));
                        }
                        Err(_) => return ready(Err(ErrorUnauthorized("Invalid token"))),
                    }
                }
            }
        }

        ready(Err(ErrorUnauthorized("No valid token found")))
    }
}
