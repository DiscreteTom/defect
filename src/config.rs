#[derive(Debug, PartialEq, Clone)]
pub struct Config {
  pub model: String,
  pub endpoint: String,
  pub pass: String,
  pub api_key: String,
}

impl Config {
  pub fn new() -> Self {
    Self {
      model: String::from("gpt-4o"),
      endpoint: String::from("https://api.openai.com/v1"),
      pass: String::from("pass"),
      api_key: String::new(),
    }
  }
}

impl Default for Config {
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
      Config::default(),
      Config {
        model: String::from("gpt-4o"),
        endpoint: String::from("https://api.openai.com/v1"),
        pass: String::from("pass"),
        api_key: String::new(),
      }
      .clone()
    );
  }
}
