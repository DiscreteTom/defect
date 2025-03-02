use clap::{Parser, ValueEnum};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Schema {
  #[default]
  #[clap(name = "openai")]
  OpenAI,
  Bedrock,
}

/// Call LLMs in your pipeline.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// The model to use.
  #[arg(short, long, default_value_t = String::from("gpt-4o"))]
  pub model: String,

  /// The API schema to use.
  #[arg(short, long, value_enum, default_value_t = Schema::OpenAI)]
  pub schema: Schema,

  /// The prompt to use.
  /// If not provided or equal to "-", the program will read from stdin.
  pub prompt: Option<String>,
}
