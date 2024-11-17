use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{Json};
use serde::{Deserialize, Serialize};
use crate::config;
use crate::device::model::{Device};
use crate::device::presenter::{DeviceData};
use crate::sensor::model::{Sensor, SensorRepo};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSensorDataRequest {
    pub device: String,
    pub temperature: f64,
    pub humidity: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SensorData {
    pub id: i32,
    pub device: DeviceData,
    pub temperature: f64,
    pub humidity: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SensorInsightData {
    pub message: String,
}

#[post("/iot-manager/sensor")]
pub async fn create(request: Json<CreateSensorDataRequest>) -> impl Responder {
    let mut sensor_repo = SensorRepo::new(&config::CONFIG.postgres_url, &config::CONFIG.general_service_url).await.unwrap();
    sensor_repo.create(&Sensor {
        id: None,
        device: Device {
            id: request.device.to_string(),
            lon: 0.0,
            lat: 0.0,
        },
        temperature: request.temperature,
        humidity: request.humidity,
    }).await.unwrap();

    HttpResponse::Ok()
}

#[get("/iot-manager/sensor")]
pub async fn get() -> impl Responder {
    let mut sensor_repo = SensorRepo::new(&config::CONFIG.postgres_url, &config::CONFIG.general_service_url).await.unwrap();
    let sensors = sensor_repo.get_all().await.unwrap();
    
    let mut sensors_data: Vec<SensorData> = Vec::new();
    for sensor in sensors {
        let sensor_data = SensorData {
            id: sensor.id.unwrap(),
            device: DeviceData {
                id: sensor.device.id,
                lon: 0.0,
                lat: 0.0,
            },
            temperature: sensor.temperature,
            humidity: sensor.humidity,
        };
        
        sensors_data.push(sensor_data);
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(sensors_data)
}

#[get("/iot-manager/sensor/insight")]
pub async fn get_insight() -> impl Responder {
    let mut sensor_repo = SensorRepo::new(&config::CONFIG.postgres_url, &config::CONFIG.general_service_url).await.unwrap();
    let insight = sensor_repo.get_insight(Sensor {
        id: Some(1),
        device: Device {
            id: String::from("test"),
            lon: 0.0,
            lat: 0.0,
        },
        temperature: 37.0,
        humidity: 20.0,
    }).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(SensorInsightData {
            message: insight.message,
        })
}
