use actix_web::{post, Responder, HttpResponse};
use actix_web::web::Json;
use serde::{Deserialize};
use crate::device::model::{DeviceRepo, Device};
use crate::config;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeviceDataRequest {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
}

#[post("/iot-manager/device")]
pub async fn create(request: Json<DeviceDataRequest>) -> impl Responder {
    let mut device_repo = DeviceRepo::new(&config::CONFIG.postgres_url).await.unwrap();
    device_repo.create(Device {
        id: request.id.as_str(),
        lon: request.lon,
        lat: request.lat,
    }).await.unwrap();

    HttpResponse::Ok()
}
