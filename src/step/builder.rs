use super::Step;
use crate::Config;
use openai_api_rs::v1::api::OpenAIClient;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Builder {
  config: Config,
}

impl Builder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn model(mut self, model: impl Into<String>) -> Self {
    self.config.model = model.into();
    self
  }

  pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
    self.config.endpoint = endpoint.into();
    self
  }

  pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
    self.config.api_key = api_key.into();
    self
  }

  pub fn build(self) -> Step {
    let client = OpenAIClient::builder()
      .with_api_key(self.config.api_key)
      .with_endpoint(self.config.endpoint)
      .build()
      // with api_key and endpoint provided, this should never panic
      .unwrap();

    Step {
      client,
      model: self.config.model,
    }
  }
}
