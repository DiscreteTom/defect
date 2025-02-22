pub mod builder;

use anyhow::{anyhow, Result};
use builder::Builder;
use openai_api_rs::v1::{
  api::OpenAIClient,
  chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole},
};

pub struct Step {
  client: OpenAIClient,
  model: String,
}

impl Step {
  pub fn builder() -> Builder {
    Builder::new()
  }

  pub async fn exec(&self, text: impl Into<String>) -> Result<String> {
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
    let res = self.client.chat_completion(req).await?;

    res
      .choices
      .into_iter()
      .next()
      .ok_or(anyhow!("no choices"))?
      .message
      .content
      .ok_or(anyhow!("no content"))
  }
}
