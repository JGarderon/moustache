pub mod default;

use crate::engine::Document;
use crate::engine::Environment;

#[derive(Debug)]
pub enum Value {
  Text(String),
  Symbol(String),
  Number(f64),
  Vector(Vec<Value>),
  Void,
}

#[derive(Debug)]
pub struct Context<'a, 'b> {
  begining: bool,
  anterior_result: &'b Value,
  doc: &'a Document,
  doc_position: usize,
  env: &'a mut Environment,
  fct_name: &'b str,
  args: Vec<Value>,
}

impl<'a, 'b> Context<'a, 'b> {
  pub fn new(
    begining: bool,
    anterior_result: &'b Value,
    doc: &'a Document,
    doc_position: usize,
    env: &'a mut Environment,
    fct_name: &'b str,
    args: Vec<Value>,
  ) -> Self {
    Context {
      begining,
      anterior_result,
      doc,
      doc_position,
      env,
      fct_name,
      args,
    }
  }
}

#[derive(Debug)]
pub struct Helper {
  module_name: &'static str,
  module_description: &'static str,
  module_autor: &'static str,
  module_version: &'static str,
  functions: Vec<HelperFunction>,
}

#[derive(Debug)]
struct HelperFunction {
  function_name: &'static str,
  function_description: &'static str,
  function_can_pipe: bool,
  function_args: &'static str,
}
