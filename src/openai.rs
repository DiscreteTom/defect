use crate::Invoker;
use openai_api_rs::v1::{
  api::OpenAIClient,
  chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole},
};
use tracing::trace;

#[derive(Debug, PartialEq, Clone)]
pub struct OpenAIConfig {
  pub model: String,
  pub endpoint: String,
  pub api_key: String,
}

impl OpenAIConfig {
  pub fn new() -> Self {
    Self {
      model: String::from("gpt-4o"),
      endpoint: String::from("https://api.openai.com/v1"),
      api_key: String::new(),
    }
  }
}

impl Default for OpenAIConfig {
  fn default() -> Self {
    Self::new()
  }
}

pub struct OpenAIInvoker {
  client: OpenAIClient,
  model: String,
}

impl OpenAIInvoker {
  pub fn new(config: OpenAIConfig) -> Self {
    Self {
      client: OpenAIClient::builder()
        .with_api_key(config.api_key)
        .with_endpoint(config.endpoint)
        .build()
        // with api_key and endpoint provided, this should never panic
        .unwrap(),
      model: config.model,
    }
  }
}

impl Invoker for OpenAIInvoker {
  async fn invoke(&self, text: impl Into<String>) {
    let req = ChatCompletionRequest::new(
      self.model.clone(),
      vec![ChatCompletionMessage {
        role: MessageRole::user,
        content: Content::Text(text.into()),
        name: None,
        tool_calls: None,
        tool_call_id: None,
      }],
    );
    trace!("{:?}", req);

    let res = self.client.chat_completion(req).await.unwrap();
    trace!("{:?}", res);

    let output = res
      .choices
      .into_iter()
      .next()
      .unwrap()
      .message
      .content
      .unwrap();

    println!("{}", output);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config() {
    assert_eq!(
      OpenAIConfig::default(),
      OpenAIConfig {
        model: String::from("gpt-4o"),
        endpoint: String::from("https://api.openai.com/v1"),
        api_key: String::new(),
      }
      .clone()
    );
  }
}
