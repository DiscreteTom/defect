mod args;
mod bedrock;
mod openai;

use args::{Args, Schema};
use bedrock::invoke_bedrock;
use clap::Parser;
use openai::invoke_openai;
use std::io::{stderr, stdin, Read};
use tracing::debug;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

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
