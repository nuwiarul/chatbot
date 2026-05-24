use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::llm::{apply_system_policy, ChatMessage};
use axum::extract::State;
use axum::response::sse::{Event, Sse};
use axum::routing::post;
use axum::Json;
use bytes::Bytes;
use futures_util::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/chat/stream", post(chat_stream))
}

#[derive(Debug, Deserialize)]
pub struct ChatStreamRequest {
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
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
struct OpenAiChatCompletionsChunk {
    choices: Vec<OpenAiChoiceDelta>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoiceDelta {
    delta: OpenAiDelta,
}

#[derive(Debug, Deserialize)]
struct OpenAiDelta {
    #[serde(default)]
    content: Option<String>,
}

async fn chat_stream(
    State(state): State<AppState>,
    Json(req): Json<ChatStreamRequest>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/v1/chat/completions",
        state.config.llama_base_url.trim_end_matches('/')
    );

    let payload = OpenAiChatCompletionsRequest {
        model: "local-model",
        messages: apply_system_policy(req.messages),
        temperature: req.temperature,
        max_tokens: req.max_tokens,
        stream: true,
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

    let byte_stream = res.bytes_stream();

    let sse_stream = async_stream::stream! {
        let mut buffer = String::new();
        futures_util::pin_mut!(byte_stream);

        while let Some(item) = byte_stream.next().await {
            let chunk: Bytes = match item {
                Ok(c) => c,
                Err(_) => break,
            };

            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(pos) = buffer.find("\n\n") {
                let frame = buffer[..pos].to_string();
                buffer.drain(..pos+2);

                for line in frame.lines() {
                    // Do NOT trim: we must preserve leading spaces in deltas.
                    let Some(data) = line.strip_prefix("data:") else { continue; };
                    // SSE allows an optional single space after `data:`
                    let data = data.strip_prefix(' ').unwrap_or(data);
                    if data == "[DONE]" {
                        yield Ok(Event::default().event("done").data("[DONE]"));
                        return;
                    }
                    let parsed: Result<OpenAiChatCompletionsChunk, _> = serde_json::from_str(data);
                    if let Ok(parsed) = parsed {
                        for choice in parsed.choices {
                            if let Some(content) = choice.delta.content {
                                if !content.is_empty() {
                                    yield Ok(Event::default().event("delta").data(content));
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(Sse::new(sse_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}
