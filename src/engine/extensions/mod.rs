#[cfg(feature = "engine-extensions")]
pub mod ext_default;

#[cfg(feature = "engine-extensions")]
pub mod ext_macro;

use crate::engine::Document;
use crate::engine::Environment;

#[allow(dead_code)]
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
  pub fct_name: &'a str,
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
      fct_name: &source[0..0],
      args: vec![],
    }
  }
}

#[cfg_attr(not(feature = "engine-extensions"), allow(dead_code))]
#[derive(Debug)]
pub struct Helper {
  module_name: &'static str,
  module_description: &'static str,
  functions: Vec<HelperFunction>,
}

#[cfg_attr(not(feature = "engine-extensions"), allow(dead_code))]
impl Helper {
  pub fn display(&self) {
    println!(
      "
  ♦ Extension '{}' 
    {}",
      self.module_name, self.module_description
    );
    for f in self.functions.iter() {
      f.display();
    }
    println!("");
  }
}

#[cfg_attr(not(feature = "engine-extensions"), allow(dead_code))]
#[derive(Debug)]
pub struct HelperFunction {
  function_name: &'static str,
  function_description: &'static str,
  function_can_pipe: bool,
  function_args: &'static str,
}

#[cfg_attr(not(feature = "engine-extensions"), allow(dead_code))]
impl HelperFunction {
  fn display(&self) {
    println!(
      "
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

#[cfg(feature = "engine-extensions")]
mod optional_feature {
  use crate::engine::extensions::ext_default;
  use crate::engine::extensions::ext_macro;
  use crate::engine::extensions::Context;

  pub fn execute<'a>(module: &str, context: &mut Context) -> Option<String> {
    match module {
      m if m == ext_default::MODULE_NAME => ext_default::execute(context),
      m if m == ext_macro::MODULE_NAME => ext_macro::execute(context),
      m => Some(format!(
        "Extension '{}' not found (--help-extensions argument may assist you)",
        m
      )),
    }
  }
  pub fn help() {
    ext_default::help().display();
    ext_macro::help().display();
  }
}

#[cfg(not(feature = "engine-extensions"))]
mod optional_feature {
  use crate::engine::extensions::Context;
  pub fn execute<'a>(_module: &str, _context: &mut Context) -> Option<String> {
    Some("Extensions are not available in this version of the engine".to_string())
  }
  pub fn help() {
    println!("Extensions are not available in this version of the engine")
  }
}

pub fn execute<'a>(module: &str, context: &mut Context) -> Option<String> {
  optional_feature::execute(module, context)
}

pub fn help() {
  optional_feature::help();
}
