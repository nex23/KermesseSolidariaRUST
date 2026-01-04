use bcrypt::{hash, verify, DEFAULT_COST};
use actix_web::error::ErrorInternalServerError;

pub fn hash_password(password: &str) -> Result<String, actix_web::Error> {
    hash(password, DEFAULT_COST).map_err(|e| ErrorInternalServerError(e.to_string()))
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, actix_web::Error> {
    verify(password, hashed).map_err(|e| ErrorInternalServerError(e.to_string()))
}
