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
    pub financial_goal: Option<rust_decimal::Decimal>,
    pub qr_code_url: Option<String>,
    pub department: Option<String>,
    pub city: Option<String>,
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
    pub financial_goal: Option<rust_decimal::Decimal>,
    pub qr_code_url: Option<String>,
    pub status: String,
    pub organizer_id: i32,
    pub department: Option<String>,
    pub city: Option<String>,
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
            financial_goal: model.financial_goal,
            qr_code_url: model.qr_code_url,
            status: model.status,
            organizer_id: model.organizer_id,
            department: model.department,
            city: model.city,
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
        financial_goal: Set(req.financial_goal),
        qr_code_url: Set(req.qr_code_url.clone()),
        department: Set(req.department.clone()),
        city: Set(req.city.clone()),
        status: Set("ACTIVE".to_string()), // Default to ACTIVE for now for testing
        ..Default::default()
    };

    match kermesse.insert(conn).await {
        Ok(model) => HttpResponse::Created().json(KermesseResponse::from(model)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create kermesse: {}", e)),
    }
}

#[derive(Deserialize)]
pub struct KermesseFilter {
    pub department: Option<String>,
}

pub async fn list_kermesses(
    data: web::Data<AppState>,
    filter: web::Query<KermesseFilter>,
) -> impl Responder {
    let conn = &data.conn;

    let mut query = Kermesses::find().filter(kermesses::Column::Status.eq("ACTIVE"));

    if let Some(dept) = &filter.department {
        if !dept.is_empty() && dept != "Todos" {
             query = query.filter(kermesses::Column::Department.eq(dept.clone()));
        }
    }

    let kermesses = query.all(conn).await;

    match kermesses {
        Ok(list) => {
            let response: Vec<KermesseResponse> = list.into_iter().map(KermesseResponse::from).collect();
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[derive(Serialize)]
pub struct MyKermesseResponse {
    #[serde(flatten)]
    pub kermesse: KermesseResponse,
    pub total_raised: rust_decimal::Decimal,
    pub total_orders: i64,
}

pub async fn get_my_kermesses(
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    let kermesses = match Kermesses::find()
        .filter(kermesses::Column::OrganizerId.eq(user.id))
        .all(conn)
        .await
    {
        Ok(k) => k,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let mut response = Vec::new();
    for k in kermesses {
        let k_id = k.id;
        
        let sales_list = match crate::entity::sales::Entity::find()
            .filter(crate::entity::sales::Column::KermesseId.eq(k_id))
            .all(conn)
            .await
        {
            Ok(s) => s,
            Err(_) => vec![],
        };

        let total_raised = sales_list
            .iter()
            .filter(|s| s.status == "PAID" || s.status == "DELIVERED")
            .fold(rust_decimal::Decimal::ZERO, |acc, s| acc + s.total_amount);

        let total_orders = sales_list.len() as i64;

        response.push(MyKermesseResponse {
            kermesse: KermesseResponse::from(k),
            total_raised,
            total_orders,
        });
    }

    HttpResponse::Ok().json(response)
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
        web::resource("/my-kermesses")
            .route(web::get().to(get_my_kermesses)),
    )
    .service(
        web::resource("/kermesses")
            .route(web::get().to(list_kermesses))
            .route(web::post().to(create_kermesse)),
    )
    .service(
        web::resource("/kermesses/{id}")
            .route(web::get().to(get_kermesse))
            .route(web::put().to(update_kermesse)),
    )
    .service(
        web::resource("/kermesses/{id}/dishes").route(web::post().to(create_dish)),
    );
}

#[derive(Deserialize)]
pub struct UpdateKermesseRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub event_date: Option<NaiveDate>,
    pub beneficiary_name: Option<String>,
    pub beneficiary_reason: Option<String>,
    pub beneficiary_image_url: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub financial_goal: Option<rust_decimal::Decimal>,
    pub qr_code_url: Option<String>,
    pub department: Option<String>,
    pub city: Option<String>,
}

pub async fn update_kermesse(
    path: web::Path<i32>,
    req: web::Json<UpdateKermesseRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    let kermesse = match Kermesses::find_by_id(kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    if kermesse.organizer_id != user.id {
        return HttpResponse::Forbidden().body("Only organizer can edit kermesse");
    }

    let mut kermesse: kermesses::ActiveModel = kermesse.into();

    if let Some(name) = &req.name { kermesse.name = Set(name.clone()); }
    if let Some(desc) = &req.description { kermesse.description = Set(desc.clone()); }
    if let Some(date) = req.event_date { kermesse.event_date = Set(date); }
    if let Some(b_name) = &req.beneficiary_name { kermesse.beneficiary_name = Set(b_name.clone()); }
    if let Some(b_reason) = &req.beneficiary_reason { kermesse.beneficiary_reason = Set(b_reason.clone()); }
    if let Some(b_img) = &req.beneficiary_image_url { kermesse.beneficiary_image_url = Set(Some(b_img.clone())); }
    if let Some(start) = &req.start_time { kermesse.start_time = Set(Some(start.clone())); }
    if let Some(end) = &req.end_time { kermesse.end_time = Set(Some(end.clone())); }
    if let Some(goal) = req.financial_goal { kermesse.financial_goal = Set(Some(goal)); }
    if let Some(qr) = &req.qr_code_url { kermesse.qr_code_url = Set(Some(qr.clone())); }
    if let Some(dept) = &req.department { kermesse.department = Set(Some(dept.clone())); }
    if let Some(city) = &req.city { kermesse.city = Set(Some(city.clone())); }

    match kermesse.update(conn).await {
        Ok(model) => HttpResponse::Ok().json(KermesseResponse::from(model)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update: {}", e)),
    }
}
