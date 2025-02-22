use clap::Parser;
use defect::{BedrockConfig, BedrockInvoker, Invoker, OpenAIConfig, OpenAIInvoker};
use std::{
  env,
  io::{stdin, Read},
};

/// Call LLM in your pipeline.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The model to use.
  /// For AWS Bedrock models, use the format "bedrock/<model-id>".
  #[arg(short, long, default_value_t = OpenAIConfig::default().model)]
  model: String,

  /// The endpoint to use.
  /// Only used for OpenAI models.
  #[arg(short, long, default_value_t = OpenAIConfig::default().endpoint)]
  endpoint: String,

  /// The prompt to use.
  /// If not provided or equal to "-", the program will read from stdin.
  prompt: Option<String>,
}

#[tokio::main]
async fn main() {
  let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
  let args = Args::parse();

  let prompt = match args.prompt {
    Some(prompt) if prompt != "-" => prompt,
    _ => {
      let mut prompt = String::new();
      stdin().read_to_string(&mut prompt).unwrap();
      prompt
    }
  };

  if args.model.starts_with("bedrock/") {
    BedrockInvoker::new(BedrockConfig { model: args.model })
      .await
      .invoke(prompt)
      .await;
  } else {
    OpenAIInvoker::new(OpenAIConfig {
      model: args.model,
      endpoint: args.endpoint,
      api_key,
    })
    .invoke(prompt)
    .await;
  }
}
