use crate::Invoker;
use aws_config::load_from_env;
use aws_sdk_bedrockruntime::{
  types::{ContentBlock, ConversationRole, Message},
  Client,
};
use tracing::trace;

#[derive(Debug, PartialEq, Clone)]
pub struct BedrockConfig {
  pub model: String,
}

impl BedrockConfig {
  pub fn new() -> Self {
    Self {
      model: String::from("anthropic.claude-3-5-sonnet-20240620-v1:0"),
    }
  }
}

impl Default for BedrockConfig {
  fn default() -> Self {
    Self::new()
  }
}

pub struct BedrockInvoker {
  client: Client,
  model: String,
}

impl BedrockInvoker {
  pub async fn new(config: BedrockConfig) -> Self {
    Self {
      client: Client::new(&load_from_env().await),
      model: config.model,
    }
  }
}

impl Invoker for BedrockInvoker {
  async fn invoke(&self, text: impl Into<String>) -> () {
    let message = Message::builder()
      .role(ConversationRole::User)
      .content(ContentBlock::Text(text.into()))
      .build()
      .unwrap();
    trace!("{:?}", message);

    let mut res = self
      .client
      .converse_stream()
      .model_id(&self.model)
      .messages(message)
      .send()
      .await
      .unwrap();
    trace!("{:?}", res);

    while let Some(output) = res.stream.recv().await.unwrap() {
      trace!("{:?}", output);
      output
        .as_content_block_delta()
        .ok()
        .and_then(|e| e.delta.as_ref())
        .and_then(|delta| delta.as_text().ok())
        .map(|text| print!("{}", text));
    }

    println!("");
  }
}
