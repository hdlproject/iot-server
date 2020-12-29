mod covid19;
mod config;

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
            .service(covid19::presenter::get)
    })
        .bind(format!("127.0.0.1:{}", &config::CONFIG.port))?
        .run()
        .await
}
