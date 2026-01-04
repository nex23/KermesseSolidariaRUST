use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::entity::users;
use crate::state::AppState;
use crate::utils::{hash, jwt};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
    pub id: i32,
}

pub async fn register(
    req: web::Json<RegisterRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    // Check if user exists
    let existing_user = users::Entity::find()
        .filter(
            users::Column::Username.eq(&req.username)
                .or(users::Column::Email.eq(&req.email))
        )
        .one(conn)
        .await;

    match existing_user {
        Ok(Some(_)) => return HttpResponse::BadRequest().body("Username or Email already exists"),
        Ok(None) => {}
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    }

    // Hash password
    let password_hash = match hash::hash_password(&req.password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    // Create User
    let new_user = users::ActiveModel {
        username: Set(req.username.clone()),
        email: Set(req.email.clone()),
        password_hash: Set(password_hash),
        full_name: Set(req.full_name.clone()),
        phone: Set(req.phone.clone()),
        ..Default::default()
    };

    match new_user.insert(conn).await {
        Ok(user) => {
            // Generate Token
            match jwt::sign_token(user.id, &user.username) {
                Ok(token) => HttpResponse::Ok().json(AuthResponse {
                    token,
                    username: user.username,
                    id: user.id,
                }),
                Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

pub async fn login(
    req: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    // Find user
    // Find user by username OR email
    let user = match users::Entity::find()
        .filter(
            users::Column::Username.eq(&req.username)
                .or(users::Column::Email.eq(&req.username))
        )
        .one(conn)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    // Verify password
    match hash::verify_password(&req.password, &user.password_hash) {
        Ok(valid) => {
            if !valid {
                return HttpResponse::Unauthorized().body("Invalid credentials");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("Failed to verify password"),
    }

    // Generate Token
    match jwt::sign_token(user.id, &user.username) {
        Ok(token) => HttpResponse::Ok().json(AuthResponse {
            token,
            username: user.username,
            id: user.id,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth/register").route(web::post().to(register))
    )
    .service(
         web::resource("/auth/login").route(web::post().to(login))
    );
}
