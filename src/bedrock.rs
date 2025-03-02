use aws_config::load_from_env;
use aws_sdk_bedrockruntime::{
  operation::converse_stream::ConverseStreamOutput,
  types::{ContentBlock, ConversationRole, ConverseStreamOutput::*, Message, SystemContentBlock},
  Client,
};
use tracing::{debug, trace};

/// Invokes the AWS Bedrock API with the given model, system instructions and user messages.
pub async fn invoke_bedrock(model: String, system: Vec<String>, message: String) {
  let config = load_from_env().await;
  let system = create_system_messages(system);
  let message = create_message(message);

  let mut output = Client::new(&config)
    .converse_stream()
    .model_id(model)
    .set_system(system)
    .messages(message)
    .send()
    .await
    .unwrap();
  debug!("{:?}", output);

  handle_stream(&mut output).await;
}

fn create_system_messages(text: Vec<String>) -> Option<Vec<SystemContentBlock>> {
  if text.is_empty() {
    None
  } else {
    let blocks = text
      .into_iter()
      .map(|text| SystemContentBlock::Text(text))
      .collect();
    debug!("{:?}", blocks);
    Some(blocks)
  }
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
  fn test_create_system_messages() {
    let text = vec!["Hello, world!".to_string()];
    let messages = create_system_messages(text.clone());

    assert_eq!(messages.unwrap()[0].as_text().unwrap(), &text[0]);
  }

  #[test]
  fn test_create_message() {
    let text = "Hello, world!".to_string();
    let message = create_message(text.clone());

    assert_eq!(message.role(), &ConversationRole::User);
    assert_eq!(message.content()[0].as_text().unwrap(), &text);
  }
}
