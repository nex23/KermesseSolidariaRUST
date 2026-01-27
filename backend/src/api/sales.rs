use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait, QueryOrder,
};
use serde::{Deserialize, Serialize};

use crate::entity::{dishes, kermesses, sale_items, sales, prelude::*};
use crate::state::AppState;
use crate::utils::auth::AuthenticatedUser;

#[derive(Serialize, Deserialize)]
pub struct SaleItemRequest {
    pub dish_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSaleRequest {
    pub kermesse_id: i32,
    pub customer_name: String,
    pub items: Vec<SaleItemRequest>,
    pub delivery_method: String, // "PICKUP" or "DELIVERY"
    pub delivery_address: Option<String>,
    pub contact_phone: Option<String>,
    pub payment_method: String, // "QR" or "CASH"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleItemReceipt {
    pub dish_name: String,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
    pub subtotal: rust_decimal::Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleReceipt {
    pub id: i32,
    pub kermesse_name: String,
    pub event_date: String,
    pub customer_name: String,
    pub total_amount: rust_decimal::Decimal,
    pub status: String,
    pub payment_method: String,
    pub delivery_method: String,
    pub created_at: String,
    pub items: Vec<SaleItemReceipt>,
}

pub async fn create_sale(
    req: web::Json<CreateSaleRequest>,
    user: Option<AuthenticatedUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    // Start transaction
    let txn = match conn.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to start transaction"),
    };

    // calculate total and verify dishes
    let mut total_decimal = rust_decimal::Decimal::ZERO;
    let mut sale_items_data = Vec::new();
    let mut receipt_items = Vec::new();

    // Verify Kermesse exists to get Organizer ID (default seller)
    let kermesse = match Kermesses::find_by_id(req.kermesse_id).one(&txn).await {
         Ok(Some(k)) => k,
         Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
         Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    for item in &req.items {
        let dish = match Dishes::find_by_id(item.dish_id).one(&txn).await {
            Ok(Some(d)) => d,
            Ok(None) => return HttpResponse::BadRequest().body(format!("Dish {} not found", item.dish_id)),
            Err(_) => return HttpResponse::InternalServerError().body("Database error"),
        };

        if dish.kermesse_id != req.kermesse_id {
             return HttpResponse::BadRequest().body(format!("Dish {} does not belong to kermesse {}", item.dish_id, req.kermesse_id));
        }

        if dish.quantity_available < item.quantity {
             return HttpResponse::BadRequest().body(format!("Insufficient stock for dish '{}'. Available: {}, Requested: {}", dish.name, dish.quantity_available, item.quantity));
        }

        // Decrement quantity available
        let mut dish_active: dishes::ActiveModel = dish.clone().into();
        dish_active.quantity_available = Set(dish.quantity_available - item.quantity);
        if let Err(_) = dish_active.update(&txn).await {
             return HttpResponse::InternalServerError().body("Failed to update dish stock");
        }

        let subtotal = dish.price * rust_decimal::Decimal::from(item.quantity);
        total_decimal += subtotal;
        
        receipt_items.push(SaleItemReceipt {
            dish_name: dish.name.clone(),
            quantity: item.quantity,
            unit_price: dish.price,
            subtotal,
        });
        
        sale_items_data.push((item, dish.price));
    }

    // Determine Seller and Buyer
    let seller_id = kermesse.organizer_id;
    let buyer_id = user.map(|u| u.id);

    let sale = sales::ActiveModel {
        kermesse_id: Set(req.kermesse_id),
        seller_id: Set(seller_id),
        customer_name: Set(req.customer_name.clone()),
        total_amount: Set(total_decimal),
        delivery_method: Set(req.delivery_method.clone()),
        delivery_address: Set(req.delivery_address.clone()),
        contact_phone: Set(req.contact_phone.clone()),
        buyer_id: Set(buyer_id),
        payment_method: Set(req.payment_method.clone()), // "QR", "CASH"
        status: Set("PENDING".to_string()),
        ..Default::default()
    };

    let sale = match sale.insert(&txn).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to create sale"),
    };

    // Insert Sale Items
    for (item, price) in sale_items_data {
         let subtotal = price * rust_decimal::Decimal::from(item.quantity);
         let sale_item = sale_items::ActiveModel {
            sale_id: Set(sale.id),
            dish_id: Set(item.dish_id),
            quantity: Set(item.quantity),
            subtotal: Set(subtotal),
            ..Default::default()
         };
         if let Err(_) = sale_item.insert(&txn).await {
             return HttpResponse::InternalServerError().body("Failed to insert sale item");
         }
    }

    match txn.commit().await {
        Ok(_) => {
            let receipt = SaleReceipt {
                id: sale.id,
                kermesse_name: kermesse.name,
                event_date: kermesse.event_date.to_string(),
                customer_name: sale.customer_name,
                total_amount: sale.total_amount,
                status: sale.status,
                payment_method: sale.payment_method,
                delivery_method: sale.delivery_method,
                created_at: sale.created_at.to_string(),
                items: receipt_items,
            };
            HttpResponse::Created().json(receipt)
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to commit transaction"),
    }
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_sale_status(
    path: web::Path<i32>,
    req: web::Json<UpdateStatusRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let sale_id = path.into_inner();
    let conn = &data.conn;

    let sale = match Sales::find_by_id(sale_id).one(conn).await {
        Ok(Some(s)) => s,
        Ok(None) => return HttpResponse::NotFound().body("Sale not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    // Verify User is Organizer OR Collaborator
    let kermesse = match Kermesses::find_by_id(sale.kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        _ => return HttpResponse::InternalServerError().body("Kermesse lookup error"),
    };

    let is_organizer = kermesse.organizer_id == user.id;
    let is_collaborator = if !is_organizer {
        let collab = crate::entity::collaborators::Entity::find()
            .filter(crate::entity::collaborators::Column::KermesseId.eq(sale.kermesse_id))
            .filter(crate::entity::collaborators::Column::UserId.eq(user.id))
            .filter(crate::entity::collaborators::Column::Status.eq("ACCEPTED"))
            .one(conn)
            .await;
        matches!(collab, Ok(Some(_)))
    } else {
        false
    };

    if !is_organizer && !is_collaborator {
        return HttpResponse::Forbidden().body("Only organizer or collaborator can update status");
    }

    let mut sale: sales::ActiveModel = sale.into();
    sale.status = Set(req.status.clone());

    match sale.update(conn).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update status"),
    }
}

#[derive(Serialize)]
pub struct SaleResponse {
    pub id: i32,
    pub customer_name: String,
    pub total_amount: rust_decimal::Decimal,
    pub status: String,
}

pub async fn list_sales(
    path: web::Path<i32>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    // Verify User is Organizer OR Collaborator
    let kermesse = match Kermesses::find_by_id(kermesse_id).one(conn).await {
        Ok(Some(k)) => k,
        Ok(None) => return HttpResponse::NotFound().body("Kermesse not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let is_organizer = kermesse.organizer_id == user.id;
    let is_collaborator = if !is_organizer {
        let collab = crate::entity::collaborators::Entity::find()
            .filter(crate::entity::collaborators::Column::KermesseId.eq(kermesse_id))
            .filter(crate::entity::collaborators::Column::UserId.eq(user.id))
            .filter(crate::entity::collaborators::Column::Status.eq("ACCEPTED"))
            .one(conn)
            .await;
        matches!(collab, Ok(Some(_)))
    } else {
        false
    };

    if !is_organizer && !is_collaborator {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let sales_list = match Sales::find()
        .filter(sales::Column::KermesseId.eq(kermesse_id))
        .order_by_desc(sales::Column::CreatedAt)
        .all(conn)
        .await
    {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let response: Vec<SaleResponse> = sales_list.into_iter().map(|s| SaleResponse {
        id: s.id,
        customer_name: s.customer_name,
        total_amount: s.total_amount,
        status: s.status,
    }).collect();

    HttpResponse::Ok().json(response)
}

#[derive(Serialize)]
pub struct MySaleResponse {
    pub id: i32,
    pub kermesse_name: String,
    pub event_date: String,
    pub total_amount: rust_decimal::Decimal,
    pub status: String,
    pub payment_method: String,
    pub created_at: String,
}

pub async fn list_my_orders(
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    // Find sales where buyer_id = user.id
    // Also join with Kermesses to get name and date
    // SeaORM join:
    let start = std::time::Instant::now();
    let sales_list = match Sales::find()
        .filter(sales::Column::BuyerId.eq(user.id))
        .find_also_related(kermesses::Entity)
        .order_by_desc(sales::Column::CreatedAt)
        .all(conn)
        .await
    {
         Ok(s) => s,
         Err(e) => return HttpResponse::InternalServerError().body(format!("DB Error: {}", e)),
    };
    
    // transform
    let mut response = Vec::new();
    for (sale, kermesse_opt) in sales_list {
        if let Some(k) = kermesse_opt {
            response.push(MySaleResponse {
                id: sale.id,
                kermesse_name: k.name,
                event_date: k.event_date.to_string(),
                total_amount: sale.total_amount,
                status: sale.status,
                payment_method: sale.payment_method,
                created_at: sale.created_at.to_string(),
            });
        }
    }

    HttpResponse::Ok().json(response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/sales").route(web::post().to(create_sale)),
    )
    .service(
        web::resource("/kermesses/{id}/sales").route(web::get().to(list_sales)),
    )
    .service(
        web::resource("/sales/{id}/status").route(web::put().to(update_sale_status)),
    )
    .service(
        web::resource("/my-orders").route(web::get().to(list_my_orders)),
    );
}
