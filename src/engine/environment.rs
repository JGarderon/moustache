use std::collections::HashMap;

use crate::engine::document::Part;
use crate::utils::conf::Configuration;
use crate::engine::Document;

#[derive(Debug)]
pub struct Environment {
  stack: HashMap<String, String>,
  blocks: HashMap<String, Vec<Part>>,
}

#[allow(dead_code)]
impl Environment {
  pub fn new() -> Self {
    Environment {
      stack: HashMap::new(),
      blocks: HashMap::new(),
    }
  }
  pub fn from_args(conf: &Configuration) -> Self {
    Environment {
      stack: conf.variables.clone(),
      blocks: HashMap::new(),
    }
  }
  pub fn set(&mut self, key: String, value: String) {
    self.stack.insert(key, value);
  }
  pub fn get(&self, key: &String) -> Option<&String> {
    self.stack.get(key)
  }
  pub fn set_block(&mut self, key: String, value: Vec<Part>) {
    self.blocks.insert(key, value);
  }
  pub fn get_block(&self, key: &String) -> Option<&Vec<Part>> {
    self.blocks.get(key)
  }
  pub fn transform(&mut self, doc: &Document) {
    for block in self.blocks.values_mut() {
      let mut destination: String = "".to_string();
      for value in &mut *block {
        match value {
          &mut Part::StaticText(s, e) => destination.push_str(&doc.source[s..e]),
          Part::GeneratedText(s) => destination.push_str(&s[..]),
          &mut Part::Statement(s, e) => destination.push_str(&doc.source[s..e]),
          &mut Part::Expression(s, e) => destination.push_str(&doc.source[s..e]),
          Part::Comment(_, _) => (),
        }
      }
      *block = vec!(Part::GeneratedText(destination));
    }
  }
}
