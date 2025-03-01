use openai::{
  chat::{ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole},
  Credentials,
};
use tracing::trace;

pub async fn invoke_openai(model: String, text: String) {
  let messages = vec![ChatCompletionMessage {
    role: ChatCompletionMessageRole::User,
    content: Some(text),
    ..Default::default()
  }];
  trace!("messages: {:?}", messages);
  let mut chat_stream = ChatCompletionDelta::builder(&model, messages)
    .credentials(Credentials::from_env())
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
      return;
    }
  }
  panic!("Incomplete stream");
}
