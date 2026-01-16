use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
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
}

pub async fn create_sale(
    req: web::Json<CreateSaleRequest>,
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = &data.conn;

    // Start transaction
    let txn = match conn.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to start transaction"),
    };

    // Calculate total and verify dishes
    let mut total_amount = 0.0;
    let mut sale_items_data = Vec::new();

    for item in &req.items {
        let dish = match Dishes::find_by_id(item.dish_id).one(&txn).await {
            Ok(Some(d)) => d,
            Ok(None) => return HttpResponse::BadRequest().body(format!("Dish {} not found", item.dish_id)),
            Err(_) => return HttpResponse::InternalServerError().body("Database error"),
        };

        if dish.kermesse_id != req.kermesse_id {
             return HttpResponse::BadRequest().body(format!("Dish {} does not belong to kermesse {}", item.dish_id, req.kermesse_id));
        }

        // TODO: Check quantity available

        let subtotal = dish.price * rust_decimal::Decimal::from(item.quantity);
        total_amount += subtotal.try_into().unwrap_or(0.0); // Converting Decimal to f64 just for accumulation if needed, but TotalAmount is Decimal
        // Wait, Sales::TotalAmount is Decimal.
        
        sale_items_data.push((item, dish.price));
    }

    // Insert Sale
    let sale = sales::ActiveModel {
        kermesse_id: Set(req.kermesse_id),
        seller_id: Set(user.id),
        customer_name: Set(req.customer_name.clone()),
        total_amount: Set(rust_decimal::Decimal::from_f64_retain(total_amount).unwrap_or(rust_decimal::Decimal::ZERO)), 
        // Logic detail: Converting back and forth is messy. Better to keep Decimal.
        status: Set("PENDING".to_string()),
        ..Default::default()
    };
    
    // Correction: let's recalculate total properly using Decimal
    let mut total_decimal = rust_decimal::Decimal::ZERO;
    for (item, price) in &sale_items_data {
        total_decimal += price * rust_decimal::Decimal::from(item.quantity);
    }
    
    let sale = sales::ActiveModel {
        kermesse_id: Set(req.kermesse_id),
        seller_id: Set(user.id),
        customer_name: Set(req.customer_name.clone()),
        total_amount: Set(total_decimal),
        delivery_method: Set(req.delivery_method.clone()),
        delivery_address: Set(req.delivery_address.clone()),
        contact_phone: Set(req.contact_phone.clone()),
        buyer_id: Set(Some(user.id)), // Assuming logged-in user is the buyer
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
         
         // TODO: Update dish quantity available (decrement)
    }

    match txn.commit().await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"id": sale.id, "status": "success"})),
        Err(_) => HttpResponse::InternalServerError().body("Failed to commit transaction"),
    }
}

#[derive(Serialize)]
pub struct SaleResponse {
    pub id: i32,
    pub customer_name: String,
    pub total_amount: rust_decimal::Decimal,
    pub status: String,
    // items?
}

pub async fn list_sales(
    path: web::Path<i32>,
    data: web::Data<AppState>,
    // user: AuthenticatedUser // Should verify access
) -> impl Responder {
    let kermesse_id = path.into_inner();
    let conn = &data.conn;

    let sales_list = match Sales::find()
        .filter(sales::Column::KermesseId.eq(kermesse_id))
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/sales").route(web::post().to(create_sale)),
    )
    .service(
        web::resource("/kermesses/{id}/sales").route(web::get().to(list_sales)),
    );
}
