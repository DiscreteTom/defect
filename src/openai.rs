use futures_util::StreamExt;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{
  env,
  io::{stdout, Write},
  time::Duration,
};
use tracing::{debug, trace};

/// Message role for OpenAI API
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionMessageRole {
  System,
  User,
  Assistant,
  Tool,
  Function,
}

/// Message structure for OpenAI API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionMessage {
  pub role: ChatCompletionMessageRole,
  pub content: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_calls: Option<Vec<serde_json::Value>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_call_id: Option<String>,
}

impl Default for ChatCompletionMessage {
  fn default() -> Self {
    Self {
      role: ChatCompletionMessageRole::User,
      content: None,
      name: None,
      tool_calls: None,
      tool_call_id: None,
    }
  }
}

/// Request structure for OpenAI API
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
  model: String,
  messages: Vec<ChatCompletionMessage>,
  stream: bool,
}

/// Response structure for OpenAI API streaming
#[derive(Debug, Deserialize)]
struct ChatCompletionStreamResponse {
  choices: Vec<ChatCompletionStreamChoice>,
}

/// Choice structure for OpenAI API streaming
#[derive(Debug, Deserialize)]
struct ChatCompletionStreamChoice {
  delta: ChatCompletionDelta,
  finish_reason: Option<String>,
}

/// Delta structure for OpenAI API streaming
#[derive(Debug, Deserialize)]
struct ChatCompletionDelta {
  content: Option<String>,
}

/// Invokes the OpenAI API with the given model, system instructions and user messages.
pub async fn invoke_openai(model: String, system: Vec<String>, text: String) {
  let (api_key, base_url) = get_credentials();
  let messages = create_messages(system, text);

  let client = create_client();
  let request = ChatCompletionRequest {
    model,
    messages,
    stream: true,
  };

  debug!("Request: {:?}", request);

  let url = format!("{}chat/completions", base_url);
  let response = client
    .post(&url)
    .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
    .header(header::CONTENT_TYPE, "application/json")
    .json(&request)
    .send()
    .await
    .expect("Failed to send request to OpenAI API");

  let status = response.status();
  if !status.is_success() {
    let error_text = response.text().await.unwrap_or_default();
    panic!("OpenAI API error: {} - {}", status, error_text);
  }

  let mut stream = response.bytes_stream();
  let stdout = stdout();
  let mut stdout_lock = stdout.lock();

  while let Some(chunk_result) = stream.next().await {
    match chunk_result {
      Ok(chunk) => {
        let chunk_str = String::from_utf8_lossy(&chunk);
        for line in chunk_str.lines() {
          if line.is_empty() || line == "data: [DONE]" {
            continue;
          }

          if let Some(data) = line.strip_prefix("data: ") {
            match serde_json::from_str::<ChatCompletionStreamResponse>(data) {
              Ok(response) => {
                trace!("Response chunk: {:?}", response);
                if let Some(choice) = response.choices.first() {
                  if let Some(content) = &choice.delta.content {
                    write!(stdout_lock, "{}", content).expect("Failed to write to stdout");
                    stdout_lock.flush().expect("Failed to flush stdout");
                  }

                  if choice.finish_reason.is_some() {
                    writeln!(stdout_lock).expect("Failed to write newline to stdout");
                    return;
                  }
                }
              }
              Err(e) => {
                debug!("Failed to parse chunk: {} - {}", data, e);
              }
            }
          }
        }
      }
      Err(e) => {
        debug!("Error receiving chunk: {}", e);
      }
    }
  }

  writeln!(stdout_lock).expect("Failed to write newline to stdout");
}

/// Gets the OpenAI API credentials from environment variables.
fn get_credentials() -> (String, String) {
  let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
  debug!("OPENAI_API_KEY length: {}", api_key.len());

  let base_url = env::var("OPENAI_API_BASE").unwrap_or("https://api.openai.com/v1/".to_string());
  debug!("OPENAI_API_BASE: {}", base_url);

  (api_key, base_url)
}

/// Creates a reqwest client with appropriate settings
fn create_client() -> Client {
  Client::builder()
    .timeout(Duration::from_secs(300))
    .build()
    .expect("Failed to create HTTP client")
}

/// Creates a message vector with the provided text.
fn create_messages(system: Vec<String>, text: String) -> Vec<ChatCompletionMessage> {
  let mut messages = system
    .into_iter()
    .map(|s| ChatCompletionMessage {
      role: ChatCompletionMessageRole::System,
      content: Some(s),
      ..Default::default()
    })
    .collect::<Vec<_>>();

  messages.push(ChatCompletionMessage {
    role: ChatCompletionMessageRole::User,
    content: Some(text),
    ..Default::default()
  });

  debug!("{:?}", messages);
  messages
}

#[cfg(test)]
mod tests {
  use super::*;
  use serial_test::serial;

  #[test]
  fn test_create_messages() {
    let text = "Hello, world!".to_string();
    let messages = create_messages(vec![], text.clone());

    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].role, ChatCompletionMessageRole::User);
    assert_eq!(messages[0].content, Some(text.clone()));

    let system = vec!["System message".to_string()];
    let messages = create_messages(system, text.clone());
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].role, ChatCompletionMessageRole::System);
    assert_eq!(messages[0].content, Some("System message".to_string()));
    assert_eq!(messages[1].role, ChatCompletionMessageRole::User);
    assert_eq!(messages[1].content, Some(text));
  }

  #[test]
  #[serial]
  fn test_get_credentials_with_custom_base_url() {
    // Set test env vars
    env::set_var("OPENAI_API_KEY", "test_key");
    env::set_var("OPENAI_API_BASE", "https://test.api.com/v1/");

    let (api_key, base_url) = get_credentials();

    assert_eq!(api_key, "test_key");
    assert_eq!(base_url, "https://test.api.com/v1/");
  }

  #[test]
  #[serial]
  fn test_get_credentials_with_default_base_url() {
    // Set test env vars
    env::set_var("OPENAI_API_KEY", "test_key");
    env::remove_var("OPENAI_API_BASE");

    let (api_key, base_url) = get_credentials();

    assert_eq!(api_key, "test_key");
    assert_eq!(base_url, "https://api.openai.com/v1/");
  }

  #[test]
  #[serial]
  #[should_panic(expected = "OPENAI_API_KEY must be set")]
  fn test_get_credentials_no_api_key() {
    // Remove API key to trigger panic
    env::remove_var("OPENAI_API_KEY");

    // This should panic
    let _ = get_credentials();
  }
}
