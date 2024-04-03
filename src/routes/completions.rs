use std::fmt;
use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::Client;
use serde::Deserialize;

use crate::AppState;

enum OpenAIMessageRoles {
    User,
    System,
    Assistant,
}

impl fmt::Display for OpenAIMessageRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIMessageRoles::User => write!(f, "user"),
            OpenAIMessageRoles::System => write!(f, "system"),
            OpenAIMessageRoles::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(Deserialize)]
struct OpenAIMessage {
    content: String,
    role: OpenAIMessageRoles,
}

enum GeminiMessageRoles {
    User,
    Model,
}

impl fmt::Display for GeminiMessageRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeminiMessageRoles::Model => write!(f, "model"),
            GeminiMessageRoles::User => write!(f, "user"),
        }
    }
}

#[derive(Deserialize)]
struct GeminiMessage {
    content: String,
    role: GeminiMessageRoles,
}

impl GeminiMessage {
    pub fn from_openai_message(message: &OpenAIMessage) -> Self {
        GeminiMessage {
            content: message.content.clone(),
            role: match &message.role {
                OpenAIMessageRoles::User => GeminiMessageRoles::User,
                OpenAIMessageRoles::System => GeminiMessageRoles::User,
                OpenAIMessageRoles::Assistant => GeminiMessageRoles::Model,
            },
        }
    }
}


#[derive(Deserialize)]
struct CompletionsResponse {}

#[derive(Deserialize)]
struct CompletionsRequestBody {
    messages: Vec<OpenAIMessage>,
}

#[derive(Deserialize)]
struct CompletionsResponseError {
    message: &'static str,
}

fn extract_gemini_api_key(authorization_key: &HeaderValue) -> Option<&str> {
    /// Extracts the Gemini API key from the request headers.
    let gemini_api_key = authorization_key.to_str().ok()
        .and_then(|key| key.strip_prefix("Bearer sk-"))?;

    Some(gemini_api_key)
}

fn convert_messages(messages: Vec<OpenAIMessage>) -> Vec<GeminiMessage> {
    // System prompt is not supported in Gemini API, so we need to convert it to a normal user message.
    let mut gemini_messages: Vec<GeminiMessage> = Vec::new();

    for message in messages {
        gemini_messages.push(GeminiMessage::from_openai_message(&message));

        if let OpenAIMessageRoles::System = &message.role {
            gemini_messages.push(GeminiMessage {
                content: "".to_string(),
                role: GeminiMessageRoles::Model,
            });
        }
    }

    gemini_messages
}

async fn call_gemini_api(client: &Client, messages: &Vec<GeminiMessage>, gemini_api_key: &str) {
    /// Calls the Gemini API and returns the response.
    todo!();
}

pub async fn handler<T: IntoResponse>(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CompletionsRequestBody>,
) -> (StatusCode, Json<T>) {
    /// Handles the request and returns the response from Gemini API.
    let gemini_api_key: &str = match headers.get("Authorization").and_then(extract_gemini_api_key) {
        Some(authorization_key) => authorization_key,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(CompletionsResponseError { message: "Unauthorized" })
            );
        }
    };

    let messages: Vec<GeminiMessage> = convert_messages(payload.messages);

    call_gemini_api(&state.client, &messages, gemini_api_key).await;

    (StatusCode::OK, Json(CompletionsResponse {}))
}