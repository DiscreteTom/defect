mod bedrock;
mod openai;

use self::{bedrock::invoke_bedrock, openai::invoke_openai};
use clap::{Parser, ValueEnum};
use std::io::{stderr, stdin, Read};
use tracing::debug;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Schema {
  #[default]
  #[clap(name = "openai")]
  OpenAI,
  Bedrock,
}

/// Call LLMs in your pipeline.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The model to use.
  #[arg(short, long, default_value_t = String::from("gpt-4o"))]
  model: String,

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
      invoke_bedrock(args.model, prompt).await;
    }
    Schema::OpenAI => {
      debug!("Using OpenAI model: {}", args.model);
      invoke_openai(args.model, prompt).await;
    }
  }
}
