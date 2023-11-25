use actix_web::{web, get, HttpRequest, HttpResponse, Responder, Error};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};

use crate::config;
use crate::covid19::model::{Covid19Repo, Covid19Data};

#[derive(Debug, Deserialize)]
pub struct GetCovid19Request {
    pub country: String,
    pub status: String,
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct Covid19Response(Covid19Data);

impl Responder for Covid19Response {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self.0).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[get("/covid19")]
pub async fn get(request: web::Query<GetCovid19Request>) -> impl Responder {
    let covid_19_repo = Covid19Repo::new(&config::CONFIG.covid19_service_address);
    let payload = covid_19_repo
        .get(&request.country, &request.status, &request.from, &request.to).await
        .unwrap();

    Covid19Response(payload)
}
