use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::entity::{collaborators, ingredient_donations, ingredients, kermesses, users, prelude::*};
use crate::state::AppState;
use crate::utils::auth::AuthenticatedUser;

// ===== Collaborator Requests =====

#[derive(Serialize, Deserialize)]
pub struct RequestCollaborationRequest {
    pub proposed_role: String, // "KITCHEN", "SELLER", "DELIVERY", "INGREDIENT_GETTER"
}

pub async fn request_collaboration(
    path: web::Path<i32>,
    req: web::Json<RequestCollaborationRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    // Check if kermesse exists
    if Kermesses::find_by_id(kermesse_id).one(conn).await.unwrap_or(None).is_none() {
        return HttpResponse::NotFound().body("Kermesse not found");
    }

    // Check if user already requested/is collaborator
    let existing = Collaborators::find()
        .filter(collaborators::Column::KermesseId.eq(kermesse_id))
        .filter(collaborators::Column::UserId.eq(user.id))
        .one(conn)
        .await;

    if let Ok(Some(_)) = existing {
        return HttpResponse::BadRequest().body("Already a collaborator or pending request");
    }

    let collab = collaborators::ActiveModel {
        kermesse_id: Set(kermesse_id),
        user_id: Set(user.id),
        role: Set("".to_string()), // Will be set on approval
        status: Set("PENDING".to_string()),
        proposed_role: Set(Some(req.proposed_role.clone())),
        ..Default::default()
    };

    match collab.insert(conn).await {
        Ok(model) => HttpResponse::Created().json(serde_json::json!({"id": model.id, "status": "PENDING"})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create request: {}", e)),
    }
}

#[derive(Serialize)]
pub struct CollaboratorRequestResponse {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub full_name: String,
    pub proposed_role: Option<String>,
    pub status: String,
}

pub async fn list_collaboration_requests(
    path: web::Path<i32>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    // Verify user is organizer
    let kermesse = match Kermesses::find_by_id(kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    if kermesse.organizer_id != user.id {
        return HttpResponse::Forbidden().body("Only organizer can view requests");
    }

    let requests = match Collaborators::find()
        .filter(collaborators::Column::KermesseId.eq(kermesse_id))
        .filter(collaborators::Column::Status.eq("PENDING"))
        .find_also_related(Users)
        .all(conn)
        .await
    {
        Ok(r) => r,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let response: Vec<CollaboratorRequestResponse> = requests
        .into_iter()
        .filter_map(|(collab, user_opt)| {
            user_opt.map(|u| CollaboratorRequestResponse {
                id: collab.id,
                user_id: collab.user_id,
                username: u.username,
                full_name: u.full_name,
                proposed_role: collab.proposed_role,
                status: collab.status,
            })
        })
        .collect();

    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub struct ApproveCollaboratorRequest {
    pub approve: bool,
    pub assigned_role: Option<String>, // If approved, which role to assign
}

pub async fn manage_collaboration_request(
    path: web::Path<(i32, i32)>, // (kermesse_id, collaborator_id)
    req: web::Json<ApproveCollaboratorRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let (kermesse_id, collaborator_id) = path.into_inner();
    let conn = &data.conn;

    // Verify user is organizer
    let kermesse = match Kermesses::find_by_id(kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    if kermesse.organizer_id != user.id {
        return HttpResponse::Forbidden().body("Only organizer can manage requests");
    }

    // Find collaborator request
    let mut collab: collaborators::ActiveModel = match Collaborators::find_by_id(collaborator_id)
        .one(conn)
        .await
    {
        Ok(Some(c)) => c.into(),
        Ok(None) => return HttpResponse::NotFound().body("Request not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    if req.approve {
        let role = req.assigned_role.clone().unwrap_or_else(|| "COLLABORATOR".to_string());
        collab.status = Set("ACCEPTED".to_string());
        collab.role = Set(role);
    } else {
        collab.status = Set("REJECTED".to_string());
    }

    match collab.update(conn).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update: {}", e)),
    }
}

// ===== Ingredient Donations =====

#[derive(Serialize)]
pub struct IngredientWithProgress {
    pub id: i32,
    pub name: String,
    pub quantity_needed: rust_decimal::Decimal,
    pub unit: String,
    pub quantity_donated: rust_decimal::Decimal,
}

pub async fn get_ingredients_with_progress(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    let ingredients_list = match Ingredients::find()
        .filter(ingredients::Column::KermesseId.eq(kermesse_id))
        .all(conn)
        .await
    {
        Ok(i) => i,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let mut result = Vec::new();

    for ingredient in ingredients_list {
        // Get sum of donations for this ingredient
        let donations_sum = match IngredientDonations::find()
            .filter(ingredient_donations::Column::IngredientId.eq(ingredient.id))
            .all(conn)
            .await
        {
            Ok(donations) => donations
                .iter()
                .fold(rust_decimal::Decimal::ZERO, |acc, d| acc + d.quantity_donated),
            Err(_) => rust_decimal::Decimal::ZERO,
        };

        result.push(IngredientWithProgress {
            id: ingredient.id,
            name: ingredient.name,
            quantity_needed: ingredient.quantity_needed,
            unit: ingredient.unit,
            quantity_donated: donations_sum,
        });
    }

    HttpResponse::Ok().json(result)
}

#[derive(Deserialize)]
pub struct DonateIngredientRequest {
    pub quantity: rust_decimal::Decimal,
}

pub async fn donate_ingredient(
    path: web::Path<i32>, // ingredient_id
    req: web::Json<DonateIngredientRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let ingredient_id = path.into_inner();
    let conn = &data.conn;

    // Verify ingredient exists
    if Ingredients::find_by_id(ingredient_id).one(conn).await.unwrap_or(None).is_none() {
        return HttpResponse::NotFound().body("Ingredient not found");
    }

    let donation = ingredient_donations::ActiveModel {
        ingredient_id: Set(ingredient_id),
        user_id: Set(user.id),
        quantity_donated: Set(req.quantity),
        ..Default::default()
    };

    match donation.insert(conn).await {
        Ok(model) => HttpResponse::Created().json(serde_json::json!({"id": model.id})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to record donation: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/kermesses/{id}/join")
            .route(web::post().to(request_collaboration)),
    )
    .service(
        web::resource("/kermesses/{id}/collaborators/requests")
            .route(web::get().to(list_collaboration_requests)),
    )
    .service(
        web::resource("/kermesses/{kermesse_id}/collaborators/{collab_id}/manage")
            .route(web::post().to(manage_collaboration_request)),
    )
    .service(
        web::resource("/kermesses/{id}/ingredients/progress")
            .route(web::get().to(get_ingredients_with_progress)),
    )
    .service(
        web::resource("/ingredients/{id}/donate")
            .route(web::post().to(donate_ingredient)),
    );
}
