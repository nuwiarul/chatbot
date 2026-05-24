use crate::error::{AppError, AppResult};
use crate::http::AppState;
use axum::extract::State;
use axum::routing::post;
use axum::Json;
use serde::{Deserialize, Serialize};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/chat", post(chat))
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
}

#[derive(Debug, Serialize)]
struct OpenAiChatCompletionsRequest {
    model: &'static str,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatCompletionsResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: ChatMessage,
}

pub async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> AppResult<Json<ChatResponse>> {
    let client = reqwest::Client::new();
    let url = format!("{}/v1/chat/completions", state.config.llama_base_url.trim_end_matches('/'));

    let payload = OpenAiChatCompletionsRequest {
        model: "local-model",
        messages: req.messages,
        temperature: req.temperature,
        max_tokens: req.max_tokens,
        stream: false,
    };

    let res = client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|_| AppError::Config("failed to call llama server"))?;

    if !res.status().is_success() {
        return Err(AppError::Config("llama server returned error"));
    }

    let body: OpenAiChatCompletionsResponse = res
        .json()
        .await
        .map_err(|_| AppError::Config("invalid response from llama server"))?;

    let Some(choice) = body.choices.into_iter().next() else {
        return Err(AppError::Config("llama server returned no choices"));
    };

    Ok(Json(ChatResponse { message: choice.message }))
}
