use actix_web::{
    client::{Client, Connector},
};
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Covid19Item {
    #[serde(rename(deserialize = "Country"))]
    pub country: String,
    #[serde(rename(deserialize = "CountryCode"))]
    pub country_code: String,
    #[serde(rename(deserialize = "Province"))]
    pub province: String,
    #[serde(rename(deserialize = "City"))]
    pub city: String,
    #[serde(rename(deserialize = "CityCode"))]
    pub city_code: String,
    #[serde(rename(deserialize = "Lat"))]
    pub lat: String,
    #[serde(rename(deserialize = "Lon"))]
    pub lon: String,
    #[serde(rename(deserialize = "Cases"))]
    pub cases: i32,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
}

pub type Covid19Data = Vec<Covid19Item>;

pub struct Covid19Repo {
    server_address: String,
}

impl Covid19Repo {
    pub fn new(server_address: &str) -> Self {
        Self {
            server_address: String::from(server_address),
        }
    }

    pub async fn get(&self, country: &str, status: &str, from: &str, to: &str) -> Result<Covid19Data> {
        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let client = Client::builder()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish();

        let payload = client
            .get(format!("{}/country/{}/status/{}?from={}&to={}", self.server_address, country, status, from, to))
            .send().await
            .unwrap().body().await
            .unwrap();
        let payload_string = std::str::from_utf8(payload.as_ref()).unwrap();

        let covid_19_data: Covid19Data = serde_json::from_str(payload_string)?;

        Ok(covid_19_data)
    }
}
