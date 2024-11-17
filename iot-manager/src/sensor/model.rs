use std::fmt::Debug;
use std::str;
use awc::{Client as HttpClient, Connector};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls, Error};
use crate::device::model::Device;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Sensor {
    #[serde(skip)]
    pub id: Option<i32>,
    #[serde(skip)]
    pub device: Device,
    pub temperature: f64,
    pub humidity: f64,
}

pub struct Insight {
    pub message: String,
}

pub struct SensorRepo {
    client: Client,
    general_service_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatDataRequest {
    pub system_message: String,
    pub user_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatDataResponse {
    pub assistant_message: String,
}

impl SensorRepo {
    pub async fn new(dsn: &str, general_service_url: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Self {
            client,
            general_service_url: String::from(general_service_url),
        })
    }

    pub async fn create(&mut self, sensor: &Sensor) -> Result<(), Error> {
        self.client.execute(
            "INSERT INTO sensors (device_id, temperature, humidity) VALUES ($1, $2, $3);",
            &[&sensor.device.id, &sensor.temperature, &sensor.humidity],
        ).await?;

        Ok(())
    }

    pub async fn get_all(&mut self) -> Result<Vec<Sensor>, Error> {
        let rows = self.client.query(
            "SELECT id, device_id, temperature, humidity FROM sensors;",
            &[],
        ).await?;

        let mut sensors: Vec<Sensor>= Vec::new();
        for row in rows {
            let sensor = Sensor {
                id: Some(row.get(0)),
                device: Device {
                    id: row.get(1),
                    lon: 0.0,
                    lat: 0.0,
                },
                temperature: row.get(2),
                humidity: row.get(3),
            };
            sensors.push(sensor);
        }
        

        Ok(sensors)
    }

    pub async fn get_insight(&mut self, sensor: Sensor) -> Result<Insight, Error> {
        let client = HttpClient::builder()
            .connector(Connector::new())
            .finish();

        let sensor_json = serde_json::to_string(&sensor).unwrap();

        let chat_request = ChatDataRequest {
            system_message: "Give user a concise and consistent answer format.
Use the following steps to respond the user query:
Step 1. The user will give several sensor data in JSON.
Step 2. Parse the data accordingly and give them the insight related to the condition and what should they do.".to_string(),
            user_message: sensor_json,
        };

        let payload = client
            .post(format!("{}/openai/chat", self.general_service_url))
            .send_json(&chat_request).await
            .unwrap().body().await
            .unwrap();

        let payload_string = str::from_utf8(payload.as_ref()).unwrap();
        let chat_response: ChatDataResponse = serde_json::from_str(payload_string).unwrap();

        Ok(Insight {
            message: chat_response.assistant_message,
        })
    }
}
