use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, LoaderTrait, ModelTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::entity::{dishes, ingredients, kermesses, collaborators, users, prelude::*};
use crate::state::AppState;
use crate::utils::auth::AuthenticatedUser;

#[derive(Serialize, Deserialize)]
pub struct CreateKermesseRequest {
    pub name: String,
    pub description: String,
    pub event_date: NaiveDate,
    pub beneficiary_name: String,
    pub beneficiary_reason: String,
    pub beneficiary_image_url: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Serialize)]
pub struct KermesseResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub event_date: NaiveDate,
    pub beneficiary_name: String,
    pub beneficiary_reason: String,
    pub beneficiary_image_url: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: String,
    pub organizer_id: i32,
}

#[derive(Serialize)]
pub struct CollaboratorResponse {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub phone: String,
}

#[derive(Serialize)]
pub struct KermesseDetailResponse {
    #[serde(flatten)]
    pub kermesse: KermesseResponse,
    pub dishes: Vec<dishes::Model>,
    pub ingredients: Vec<ingredients::Model>,
    pub collaborators: Vec<CollaboratorResponse>,
}

impl From<kermesses::Model> for KermesseResponse {
    fn from(model: kermesses::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            event_date: model.event_date,
            beneficiary_name: model.beneficiary_name,
            beneficiary_reason: model.beneficiary_reason,
            beneficiary_image_url: model.beneficiary_image_url,
            start_time: model.start_time,
            end_time: model.end_time,
            status: model.status,
            organizer_id: model.organizer_id,
        }
    }
}

pub async fn create_kermesse(
    req: web::Json<CreateKermesseRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    let slug = req.name.to_lowercase().replace(" ", "-"); // Simple slug generation

    let kermesse = kermesses::ActiveModel {
        name: Set(req.name.clone()),
        slug: Set(slug), // Need to ensure uniqueness or handle error
        description: Set(req.description.clone()),
        event_date: Set(req.event_date),
        organizer_id: Set(user.id),
        beneficiary_name: Set(req.beneficiary_name.clone()),
        beneficiary_reason: Set(req.beneficiary_reason.clone()),
        beneficiary_image_url: Set(req.beneficiary_image_url.clone()),
        start_time: Set(req.start_time.clone()),
        end_time: Set(req.end_time.clone()),
        status: Set("ACTIVE".to_string()), // Default to ACTIVE for now for testing
        ..Default::default()
    };

    match kermesse.insert(conn).await {
        Ok(model) => HttpResponse::Created().json(KermesseResponse::from(model)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create kermesse: {}", e)),
    }
}

pub async fn list_kermesses(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let kermesses = Kermesses::find()
        .filter(kermesses::Column::Status.eq("ACTIVE"))
        .all(conn)
        .await;

    match kermesses {
        Ok(list) => {
            let response: Vec<KermesseResponse> = list.into_iter().map(KermesseResponse::from).collect();
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

pub async fn get_kermesse(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    let kermesse = match Kermesses::find_by_id(kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    // Load related dishes
    let dishes = match Dishes::find()
        .filter(dishes::Column::KermesseId.eq(kermesse_id))
        .all(conn)
        .await
    {
        Ok(d) => d,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to load dishes"),
    };

    // Load related ingredients
    let ingredients = match Ingredients::find()
        .filter(ingredients::Column::KermesseId.eq(kermesse_id))
        .all(conn)
        .await
    {
        Ok(i) => i,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to load ingredients"),
    };

    // Load collaborators with user info
    let collaborators_list = match Collaborators::find()
        .filter(collaborators::Column::KermesseId.eq(kermesse_id))
        .find_also_related(Users)
        .all(conn)
        .await
    {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to load collaborators"),
    };

    let collaborators: Vec<CollaboratorResponse> = collaborators_list.into_iter().filter_map(|(collab, user)| {
        user.map(|u| CollaboratorResponse {
            id: collab.id,
            username: u.username,
            full_name: u.full_name,
            role: collab.role,
            phone: u.phone,
        })
    }).collect();

    HttpResponse::Ok().json(KermesseDetailResponse {
        kermesse: KermesseResponse::from(kermesse),
        dishes,
        ingredients,
        collaborators,
    })
}

#[derive(Serialize, Deserialize)]
pub struct CreateDishRequest {
    pub name: String,
    pub description: String,
    pub price: rust_decimal::Decimal,
    pub quantity_available: i32,
    pub image_url: Option<String>,
}

pub async fn create_dish(
    path: web::Path<i32>,
    req: web::Json<CreateDishRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    // Verify user is organizer (optional but good)
    let kermesse = match Kermesses::find_by_id(kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    if kermesse.organizer_id != user.id {
        return HttpResponse::Forbidden().body("Only organizer can add dishes");
    }

    let dish = dishes::ActiveModel {
        kermesse_id: Set(kermesse_id),
        name: Set(req.name.clone()),
        description: Set(req.description.clone()),
        price: Set(req.price),
        quantity_available: Set(req.quantity_available),
        image_url: Set(req.image_url.clone()),
        ..Default::default()
    };

    match dish.insert(conn).await {
        Ok(model) => HttpResponse::Created().json(model),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create dish: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/kermesses")
            .route(web::get().to(list_kermesses))
            .route(web::post().to(create_kermesse)),
    )
    .service(
        web::resource("/kermesses/{id}").route(web::get().to(get_kermesse)),
    )
    .service(
        web::resource("/kermesses/{id}/dishes").route(web::post().to(create_dish)),
    );
}
