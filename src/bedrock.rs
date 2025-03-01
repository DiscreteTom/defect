use aws_config::load_from_env;
use aws_sdk_bedrockruntime::{
  types::{ContentBlock, ConversationRole, Message},
  Client,
};
use tracing::trace;

pub async fn invoke_bedrock(model: String, text: String) {
  let message = Message::builder()
    .role(ConversationRole::User)
    .content(ContentBlock::Text(text))
    .build()
    .unwrap();
  trace!("{:?}", message);

  let mut res = Client::new(&load_from_env().await)
    .converse_stream()
    .model_id(model)
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
      .inspect(|text| print!("{}", text));
  }

  println!();
}
