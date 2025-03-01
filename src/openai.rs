use crate::Invoker;
use openai::{
  chat::{ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole},
  Credentials,
};
use tracing::trace;

#[derive(Debug, PartialEq, Clone)]
pub struct OpenAIConfig {
  pub model: String,
}

impl OpenAIConfig {
  pub fn new() -> Self {
    Self {
      model: String::from("gpt-4o"),
    }
  }
}

impl Default for OpenAIConfig {
  fn default() -> Self {
    Self::new()
  }
}

pub struct OpenAIInvoker {
  model: String,
  credentials: Credentials,
}

impl OpenAIInvoker {
  pub fn new(config: OpenAIConfig) -> Self {
    Self {
      credentials: Credentials::from_env(),
      model: config.model,
    }
  }
}

impl Invoker for OpenAIInvoker {
  async fn invoke(&self, text: impl Into<String>) {
    let messages = vec![ChatCompletionMessage {
      role: ChatCompletionMessageRole::User,
      content: Some(text.into()),
      ..Default::default()
    }];
    trace!("messages: {:?}", messages);
    let mut chat_stream = ChatCompletionDelta::builder(&self.model, messages)
      .credentials(self.credentials.clone()) // TODO: prevent clone
      .create_stream()
      .await
      .unwrap();

    while let Some(delta) = chat_stream.recv().await {
      let choice = &delta.choices[0];
      if let Some(content) = &choice.delta.content {
        print!("{}", content);
      }
      if let Some(_) = &choice.finish_reason {
        println!();
      }
    }
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
      }
      .clone()
    );
  }
}
