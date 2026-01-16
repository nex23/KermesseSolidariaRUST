use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::Database;
use dotenvy::dotenv;
use std::env;
use log::info;

pub mod state;
pub mod entity;
pub mod utils; // Make sure utils is modded
pub mod api;
pub mod seed;

use state::AppState;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Kermesse Solidaria Backend is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Connecting to database...");
    let conn = Database::connect(&db_url).await.expect("Failed to connect to DB");
    info!("Database connected.");

    let state = AppState { conn };

    // Check for seed flag
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--seed".to_string()) {
        if let Err(e) = seed::seed_db(&state.conn).await {
            eprintln!("Error seeding database: {}", e);
            std::process::exit(1);
        }
        return Ok(());
    }

    info!("Starting server at http://127.0.0.1:8080");

use actix_cors::Cors;

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(health_check)
            .configure(api::auth::config)
            .configure(api::kermesse::config)
            .configure(api::sales::config)
            .configure(api::collaboration::config)
            .configure(api::dashboard::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
