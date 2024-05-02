use std::collections::HashMap;

#[derive(Debug)]
pub struct Configuration {
  pub input: Option<String>,
  pub output: Option<String>,
  pub variables: HashMap<String,String>,
  pub is_helping: bool,
  pub is_debugging: bool,
  pub is_reentrant: bool
}

impl Configuration {
  pub fn new() -> Self {
    Configuration {
      input: None,
      output: None,
      variables: HashMap::new(),
      is_helping: false,
      is_debugging: false,
      is_reentrant: false
    }
  }
  pub fn is_helping(&mut self, v: bool) {
    self.is_helping = v;
  }
  pub fn is_debugging(&mut self, v: bool) {
    self.is_debugging = v;
  }
  pub fn is_reentrant(&mut self, v: bool) {
    self.is_reentrant = v;
  }
}
