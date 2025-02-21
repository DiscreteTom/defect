pub mod builder;

use crate::Output;
use anyhow::{anyhow, Result};
use builder::Builder;
use bumpalo::Bump;
use jsonata_rs::JsonAta;
use openai_api_rs::v1::{
  api::OpenAIClient,
  chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole},
};

pub struct Step {
  client: OpenAIClient,
  model: String,
  pass: String,
}

impl Step {
  pub fn builder() -> Builder {
    Builder::new()
  }

  pub async fn exec(&self, text: impl Into<String>) -> Result<Output> {
    let arena = Bump::new();
    let jsonata = JsonAta::new(&self.pass, &arena)?;

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

    let content = res
      .choices
      .into_iter()
      .next()
      .ok_or(anyhow!("no choices"))?
      .message
      .content
      .ok_or(anyhow!("no content"))?;
    let pass = jsonata.evaluate(Some(&content), None)?;

    Ok(Output {
      content,
      pass: if pass.is_bool() {
        pass.as_bool()
      } else {
        return Err(anyhow!("pass is not a boolean"));
      },
    })
  }
}
