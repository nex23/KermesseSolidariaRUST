use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter,
};
use serde::Serialize;

use crate::entity::{ingredient_donations, ingredients, kermesses, sales, prelude::*};
use crate::state::AppState;
use crate::utils::auth::AuthenticatedUser;

#[derive(Serialize)]
pub struct DashboardStats {
    pub financial_goal: Option<rust_decimal::Decimal>,
    pub total_raised: rust_decimal::Decimal,
    pub progress_percentage: f64,
    pub total_orders: i64,
    pub pending_orders: i64,
    pub paid_orders: i64,
    pub delivered_orders: i64,
    pub ingredient_coverage_percentage: f64,
}

pub async fn get_dashboard_stats(
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
        return HttpResponse::Forbidden().body("Only organizer can view dashboard");
    }

    // Get all sales for this kermesse
    let sales_list = match Sales::find()
        .filter(sales::Column::KermesseId.eq(kermesse_id))
        .all(conn)
        .await
    {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    // Calculate financial stats
    let total_raised = sales_list
        .iter()
        .filter(|s| s.status == "PAID" || s.status == "DELIVERED")
        .fold(rust_decimal::Decimal::ZERO, |acc, s| acc + s.total_amount);

    let total_orders = sales_list.len() as i64;
    let pending_orders = sales_list.iter().filter(|s| s.status == "PENDING").count() as i64;
    let paid_orders = sales_list.iter().filter(|s| s.status == "PAID").count() as i64;
    let delivered_orders = sales_list.iter().filter(|s| s.status == "DELIVERED").count() as i64;

    let progress_percentage = if let Some(goal) = kermesse.financial_goal {
        if goal > rust_decimal::Decimal::ZERO {
            let ratio: f64 = (total_raised / goal).try_into().unwrap_or(0.0);
            (ratio * 100.0).min(100.0)
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Calculate ingredient coverage
    let ingredients_list = match Ingredients::find()
        .filter(ingredients::Column::KermesseId.eq(kermesse_id))
        .all(conn)
        .await
    {
        Ok(i) => i,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let mut total_needed = rust_decimal::Decimal::ZERO;
    let mut total_donated = rust_decimal::Decimal::ZERO;

    for ingredient in ingredients_list {
        total_needed += ingredient.quantity_needed;

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

        total_donated += donations_sum;
    }

    let ingredient_coverage_percentage = if total_needed > rust_decimal::Decimal::ZERO {
        let ratio: f64 = (total_donated / total_needed).try_into().unwrap_or(0.0);
        (ratio * 100.0).min(100.0)
    } else {
        100.0 // If no ingredients needed, consider it 100% covered
    };

    HttpResponse::Ok().json(DashboardStats {
        financial_goal: kermesse.financial_goal,
        total_raised,
        progress_percentage,
        total_orders,
        pending_orders,
        paid_orders,
        delivered_orders,
        ingredient_coverage_percentage,
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/kermesses/{id}/dashboard/stats")
            .route(web::get().to(get_dashboard_stats)),
    );
}
