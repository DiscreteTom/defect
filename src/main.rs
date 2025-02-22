use clap::Parser;
use defect::{BedrockConfig, BedrockInvoker, Invoker, OpenAIConfig, OpenAIInvoker};
use std::{
  env,
  io::{stderr, stdin, Read},
};
use tracing::debug;
use tracing_subscriber::FmtSubscriber;

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
  FmtSubscriber::builder().with_writer(stderr).init();

  let api_key = env::var("OPENAI_API_KEY").unwrap_or_default(); // TODO: rename
  if api_key.is_empty() {
    debug!("API key not provided");
  } else {
    debug!("API key provided, length: {}", api_key.len());
  }

  let args = Args::parse();

  let prompt = match args.prompt {
    Some(prompt) if prompt != "-" => {
      debug!("Using prompt from argument, length: {}", prompt.len());
      prompt
    }
    _ => {
      let mut prompt = String::new();
      debug!("Reading prompt from stdin");
      stdin().read_to_string(&mut prompt).unwrap();
      debug!("Using prompt from stdin, length: {}", prompt.len());
      prompt
    }
  };

  let bedrock_prefix = "bedrock/";
  if args.model.starts_with(bedrock_prefix) {
    let model = args.model[bedrock_prefix.len()..].to_string();
    debug!("Using Bedrock model: {}", model);
    BedrockInvoker::new(BedrockConfig { model })
      .await
      .invoke(prompt)
      .await;
  } else {
    debug!("Using OpenAI model: {}", args.model);
    debug!("Using OpenAI endpoint: {}", args.endpoint);
    OpenAIInvoker::new(OpenAIConfig {
      model: args.model,
      endpoint: args.endpoint,
      api_key,
    })
    .invoke(prompt)
    .await;
  }
}
