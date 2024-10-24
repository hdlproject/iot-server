use actix_web::{post, get, Responder, HttpResponse};
use actix_web::web::{Json, Query};
use serde::{Deserialize, Serialize};
use crate::device::model::{DeviceRepo, Device};
use crate::config;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeviceDataRequest {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDeviceDataByIdRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DeviceData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
}

#[post("/iot-manager/device")]
pub async fn create(request: Json<CreateDeviceDataRequest>) -> impl Responder {
    let mut device_repo = DeviceRepo::new(&config::CONFIG.postgres_url).await.unwrap();
    device_repo.create(&Device {
        id: request.id.to_string(),
        lon: request.lon,
        lat: request.lat,
    }).await.unwrap();

    HttpResponse::Ok()
}

#[get("/iot-manager/device")]
pub async fn get(request: Query<GetDeviceDataByIdRequest>) -> impl Responder {
    let mut device_repo = DeviceRepo::new(&config::CONFIG.postgres_url).await.unwrap();
    let device = device_repo.get(request.id.as_str()).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(DeviceData {
            id: device.id.to_string(),
            lon: device.lon,
            lat: device.lat,
        })
}
