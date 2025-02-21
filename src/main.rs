use clap::Parser;
use defect::{Config, Step};
use std::{
  env,
  io::{stdin, Read},
};

/// Call LLM in your pipeline.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The model to use.
  #[arg(short, long, default_value_t = Config::default().model)]
  model: String,

  /// The endpoint to use.
  #[arg(short, long, default_value_t = Config::default().endpoint)]
  endpoint: String,

  /// A JSONata expression to determine if the program should exit with 0.
  #[arg(short, long, default_value_t = Config::default().pass)]
  pass: String,

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

  let output = Step::builder()
    .api_key(api_key)
    .model(args.model)
    .endpoint(args.endpoint)
    .build()
    .exec(prompt)
    .await
    .unwrap();

  println!("{}", output.content);

  if !output.pass {
    std::process::exit(1);
  }
}
