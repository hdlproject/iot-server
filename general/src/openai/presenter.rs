use actix_web::{post, HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::openai::model::{OpenAIRepo};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatData {
    pub assistant_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatDataRequest {
    pub system_message: String,
    pub user_message: String,
}

impl Responder for ChatData {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[post("/openai/chat")]
pub async fn chat(request: Json<ChatDataRequest>) -> impl Responder {
    let openai_repo = OpenAIRepo::new(&config::CONFIG.openai_api_url, &config::CONFIG.openai_api_key);
    let mut payload = openai_repo.chat(&request.system_message, &request.user_message).await;

    ChatData {
        assistant_message: payload.choices.remove(0).message.content,
    }
}
