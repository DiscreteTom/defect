use clap::{Parser, ValueEnum};
use defect::{BedrockConfig, BedrockInvoker, Invoker, OpenAIConfig, OpenAIInvoker};
use std::{
  env,
  io::{stderr, stdin, Read},
};
use tracing::debug;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Schema {
  #[default]
  OpenAI,
  Bedrock,
}

/// Call LLMs in your pipeline.
/// To set an API key, use the "DEFECT_API_KEY" environment variable.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The model to use.
  #[arg(short, long, default_value_t = OpenAIConfig::default().model)]
  model: String,

  /// The endpoint to use.
  /// Only effective for OpenAI compatible models.
  #[arg(short, long, default_value_t = OpenAIConfig::default().endpoint)]
  endpoint: String,

  /// The API schema to use.
  #[arg(short, long, value_enum, default_value_t = Schema::OpenAI)]
  schema: Schema,

  /// The prompt to use.
  /// If not provided or equal to "-", the program will read from stdin.
  prompt: Option<String>,
}

#[tokio::main]
async fn main() {
  FmtSubscriber::builder()
    .with_env_filter(EnvFilter::from_default_env())
    .with_writer(stderr)
    .init();

  let api_key = env::var("DEFECT_API_KEY").unwrap_or_default();
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

  debug!("Using schema: {:?}", args.schema);
  match args.schema {
    Schema::Bedrock => {
      debug!("Using Bedrock model: {}", args.model);
      BedrockInvoker::new(BedrockConfig { model: args.model })
        .await
        .invoke(prompt)
        .await;
    }
    Schema::OpenAI => {
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
}
