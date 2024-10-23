use awc::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::str;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
    pub system_fingerprint: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatChoice {
    pub index: i64,
    pub message: ChatMessage,
    pub logprobs: Value,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub refusal: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
    pub prompt_tokens_details: ChatPromptTokensDetails,
    pub completion_tokens_details: ChatCompletionTokensDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatPromptTokensDetails {
    pub cached_tokens: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatCompletionTokensDetails {
    pub reasoning_tokens: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

pub struct OpenAIRepo {
    api_url: String,
    api_key: String,
}

impl OpenAIRepo {
    pub fn new(api_url: &str, api_key: &str) -> Self {
        Self {
            api_url: String::from(api_url),
            api_key: String::from(api_key),
        }
    }

    pub async fn chat(&self, system_message: &str, user_message: &str) -> ChatResponse {
        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let client = Client::builder()
            .connector(Connector::new().openssl(builder.build()))
            .finish();

        let chat_request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_message.to_string(),
                    refusal: Default::default(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_message.to_string(),
                    refusal: Default::default(),
                }
            ],
        };

        let payload = client
            .post(format!("{}/v1/chat/completions", self.api_url))
            .insert_header(("Authorization", format!("Bearer {}", self.api_key)))
            .insert_header(("Content-Type", "application/json"))
            .send_json(&chat_request).await
            .unwrap().body().await
            .unwrap();

        let payload_string = str::from_utf8(payload.as_ref()).unwrap();
        let chat_response: ChatResponse = serde_json::from_str(payload_string).unwrap();

        chat_response
    }
}
