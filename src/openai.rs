use openai::{
  chat::{ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole},
  Credentials,
};
use std::env;
use tracing::{debug, trace};

pub async fn invoke_openai(model: String, text: String) {
  let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
  debug!("OPENAI_API_KEY length: {}", api_key.len());
  let base_url = env::var("OPENAI_API_BASE").unwrap_or("https://api.openai.com/v1/".to_string());
  debug!("OPENAI_API_BASE: {}", base_url);
  let credentials = Credentials::new(api_key, base_url);

  let messages = vec![ChatCompletionMessage {
    role: ChatCompletionMessageRole::User,
    content: Some(text),
    ..Default::default()
  }];
  debug!("{:?}", messages);

  let mut chat_stream = ChatCompletionDelta::builder(&model, messages)
    .credentials(credentials)
    .create_stream()
    .await
    .unwrap();

  while let Some(delta) = chat_stream.recv().await {
    trace!("{:?}", delta);
    let choice = &delta.choices[0];
    if let Some(content) = &choice.delta.content {
      print!("{}", content);
    }
    if let Some(_) = &choice.finish_reason {
      println!();
      return;
    }
  }
  unreachable!("Incomplete stream");
}
