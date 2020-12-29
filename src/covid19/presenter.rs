use actix_web::{get, HttpResponse, Responder};
use crate::config;

#[get("/covid19")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", &config::CONFIG.covid19_server_address))
}
