pub mod default;

use crate::engine::Document;
use crate::engine::Environment;

#[derive(Debug)]
pub enum Value {
  Text(String),
  Symbol(String),
  Number(f64),
  Vector(Vec<Value>),
  True,
  False,
  Void,
}

#[derive(Debug)]
pub struct Context<'a> {
  pub begining: bool,
  pub result: Option<Value>,
  pub doc: &'a Document<'a>,
  pub doc_position: usize,
  pub env: &'a mut Environment,
  pub source: &'a str,
  pub fct_name: String,
  pub args: Vec<Value>,
}

impl<'a> Context<'a> {
  pub fn new(
    doc: &'a Document,
    doc_position: usize,
    env: &'a mut Environment,
    source: &'a str,
  ) -> Self {
    Context {
      begining: true,
      result: None,
      doc: doc,
      doc_position: doc_position,
      env: env,
      source: source,
      fct_name: "".to_string(),
      args: vec![],
    }
  }
}

#[derive(Debug)]
pub struct Helper {
  module_name: &'static str,
  module_description: &'static str,
  module_autor: &'static str,
  functions: Vec<HelperFunction>,
}

impl Helper {
  pub fn display(&self) {
    println!("
  ♦ Extension '{}' - by {} 
    {}", 
      self.module_name,
      self.module_autor,
      self.module_description
    );
    for f in self.functions.iter() {
      f.display();
    }
    println!("");
  }
}

#[derive(Debug)]
struct HelperFunction {
  function_name: &'static str,
  function_description: &'static str,
  function_can_pipe: bool,
  function_args: &'static str,
}

impl HelperFunction {
  fn display(&self) {
    println!("
    ↪ function '{}' (pipe : {}) 
      description : {}
      args : {}", 
      self.function_name,
      if self.function_can_pipe { "yes" } else { "no" },
      self.function_description,
      self.function_args,
    );
  }
}
