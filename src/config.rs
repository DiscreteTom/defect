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
