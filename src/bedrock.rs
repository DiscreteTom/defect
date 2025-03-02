use aws_config::load_from_env;
use aws_sdk_bedrockruntime::{
  operation::converse_stream::ConverseStreamOutput,
  types::{ContentBlock, ConversationRole, ConverseStreamOutput::*, Message},
  Client,
};
use tracing::{debug, trace};

/// Invokes the AWS Bedrock API with the given model and text.
pub async fn invoke_bedrock(model: String, text: String) {
  let config = load_from_env().await;
  let message = create_message(text);

  let mut output = Client::new(&config)
    .converse_stream()
    .model_id(model)
    .messages(message)
    .send()
    .await
    .unwrap();
  debug!("{:?}", output);

  handle_stream(&mut output).await;
}

/// Creates a message from the input text.
fn create_message(text: String) -> Message {
  let message = Message::builder()
    .role(ConversationRole::User)
    .content(ContentBlock::Text(text))
    .build()
    .unwrap();
  debug!("{:?}", message);
  message
}

/// Processes the stream and prints the output.
async fn handle_stream(output: &mut ConverseStreamOutput) {
  while let Some(output) = output.stream.recv().await.unwrap() {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_message() {
    let text = "Hello, world!".to_string();
    let message = create_message(text.clone());

    assert_eq!(message.role(), &ConversationRole::User);
    assert_eq!(message.content()[0].as_text().unwrap(), &text);
  }
}
