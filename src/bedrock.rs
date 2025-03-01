use aws_config::load_from_env;
use aws_sdk_bedrockruntime::{
  types::{ContentBlock, ConversationRole, ConverseStreamOutput::*, Message},
  Client,
};
use tracing::{debug, trace};

pub async fn invoke_bedrock(model: String, text: String) {
  let message = Message::builder()
    .role(ConversationRole::User)
    .content(ContentBlock::Text(text))
    .build()
    .unwrap();
  debug!("{:?}", message);

  let mut res = Client::new(&load_from_env().await)
    .converse_stream()
    .model_id(model)
    .messages(message)
    .send()
    .await
    .unwrap();
  debug!("{:?}", res);

  while let Some(output) = res.stream.recv().await.unwrap() {
    trace!("{:?}", output);

    match output {
      ContentBlockDelta(e) => {
        print!("{}", e.delta.unwrap().as_text().unwrap());
      }
      ContentBlockStop(_) => {
        println!();
        return;
      }
      _ => {}
    }
  }
  unreachable!("Incomplete stream")
}
