mod config;
mod handler;
mod jwt_auth;
mod model;
mod response;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use config::Config;
use sqlx::{Pool, Postgres};

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-Web, Postgres and SQLx";
    HttpResponse::Ok().json(serde_json::json!({"status":"success", "message":MESSAGE}))
}

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();
    println!("🔥 Server Started Successfully");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_checker_handler)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
