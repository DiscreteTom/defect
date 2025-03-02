use openai::{
  chat::{
    ChatCompletionChoiceDelta, ChatCompletionDelta, ChatCompletionGeneric, ChatCompletionMessage,
    ChatCompletionMessageRole,
  },
  Credentials,
};
use std::env;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, trace};

/// Invokes the OpenAI API with the given model, system instructions and user messages.
pub async fn invoke_openai(model: String, system: Vec<String>, text: String) {
  let credentials = get_credentials();
  let messages = create_messages(system, text);

  let mut stream = ChatCompletionDelta::builder(&model, messages)
    .credentials(credentials)
    .create_stream()
    .await
    .unwrap();
  debug!("{:?}", stream);

  handle_stream(&mut stream).await;
}

/// Gets the OpenAI API credentials from environment variables.
fn get_credentials() -> Credentials {
  let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
  debug!("OPENAI_API_KEY length: {}", api_key.len());

  let base_url = env::var("OPENAI_API_BASE").unwrap_or("https://api.openai.com/v1/".to_string());
  debug!("OPENAI_API_BASE: {}", base_url);

  Credentials::new(api_key, base_url)
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

/// Processes the chat stream and prints the content.
async fn handle_stream(stream: &mut Receiver<ChatCompletionGeneric<ChatCompletionChoiceDelta>>) {
  while let Some(delta) = stream.recv().await {
    trace!("{:?}", delta);
    let choice = &delta.choices[0];

    if let Some(content) = &choice.delta.content {
      print!("{}", content);
    }

    if choice.finish_reason.is_some() {
      println!();
      return;
    }
  }
  unreachable!("Incomplete stream");
}

#[cfg(test)]
mod tests {
  use super::*;
  use serial_test::serial;
  use std::env;
  use tokio::sync::mpsc;

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

    let credentials = get_credentials();

    assert_eq!(credentials.api_key(), "test_key");
    assert_eq!(credentials.base_url(), "https://test.api.com/v1/");
  }

  #[test]
  #[serial]
  fn test_get_credentials_with_default_base_url() {
    // Set test env vars
    env::set_var("OPENAI_API_KEY", "test_key");
    env::remove_var("OPENAI_API_BASE");

    let credentials = get_credentials();

    assert_eq!(credentials.api_key(), "test_key");
    assert_eq!(credentials.base_url(), "https://api.openai.com/v1/");
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

  #[tokio::test]
  #[should_panic(expected = "Incomplete stream")]
  async fn test_process_stream_no_finish_reason() {
    // Create a channel with a sender and receiver
    let (tx, mut rx) = mpsc::channel(1);

    // Close the sender immediately
    drop(tx);

    // This should panic
    handle_stream(&mut rx).await;
  }
}
