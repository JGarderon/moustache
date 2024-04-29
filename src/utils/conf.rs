use std::collections::HashMap;

#[derive(Debug)]
pub struct Configuration {
  pub output: Option<String>,
  pub variables: HashMap<String,String>,
  pub is_helping: bool
}

impl Configuration {
  pub fn new() -> Self {
    Configuration {
      output: None,
      variables: HashMap::new(),
      is_helping: false
    }
  }
  pub fn is_helping(&mut self, v: bool) {
    self.is_helping = v;
  }
}
