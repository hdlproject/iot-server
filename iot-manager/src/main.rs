mod config;
mod device;
mod sensor;

use std::process;
use actix_web::{HttpServer, App, get, Responder, HttpResponse};

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(device::presenter::create)
            .service(device::presenter::get)
            .service(sensor::presenter::create)
            .service(sensor::presenter::get)
    })
        .bind(format!("0.0.0.0:{}", &config::CONFIG.port)).
        unwrap_or_else(|e| {
            println!("can not bind to port {}: {}", &config::CONFIG.port, e);
            process::exit(1)
        })
        .run()
        .await
}
