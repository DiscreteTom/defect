mod bedrock;
mod openai;

use std::future::Future;

pub use bedrock::*;
pub use openai::*;

pub trait Invoker {
  fn invoke(&self, text: impl Into<String>) -> impl Future<Output = ()>;
}
