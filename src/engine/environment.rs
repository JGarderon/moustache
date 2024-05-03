use std::collections::HashMap;

use crate::utils::conf::Configuration;

#[derive(Debug)]
pub struct Environment {
  stack: HashMap<String, String>,
}

#[allow(dead_code)]
impl Environment {
  pub fn new() -> Self {
    Environment {
      stack: HashMap::new(),
    }
  }
  pub fn from_args(conf: &Configuration) -> Self {
    Environment {
      stack: conf.variables.clone(),
    }
  }
  pub fn set(&mut self, key: String, value: String) {
    self.stack.insert(key, value);
  }
  pub fn get(&mut self, key: &String) -> Option<&String> {
    self.stack.get(key)
  }
}

// #[derive(Debug)]
// pub enum Value {
//   Bool(bool),
//   Number(f64),
//   Text(String),
//   Array(Vec<Value>),
//   Dict(HashMap<String,Value>),
//   Null,
//   None
// }
