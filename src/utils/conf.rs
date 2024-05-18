use std::collections::HashMap;

#[derive(Debug)]
pub struct Configuration {
  pub input: Option<String>,
  pub output: Option<String>,
  pub variables: HashMap<String, String>,
  pub is_helping: bool,
  pub is_helping_extensions: bool,
  pub is_debugging: bool,
  pub is_reentrant: bool,
  pub display_version: bool,
}

impl Configuration {
  pub fn new() -> Self {
    Configuration {
      input: None,
      output: None,
      variables: HashMap::new(),
      is_helping: false,
      is_helping_extensions: false,
      is_debugging: false,
      is_reentrant: false,
      display_version: false,
    }
  }
  pub fn is_helping(&mut self, v: bool) {
    self.is_helping = v;
  }
  pub fn is_helping_extensions(&mut self, v: bool) {
    self.is_helping_extensions = v;
  }
  pub fn is_debugging(&mut self, v: bool) {
    self.is_debugging = v;
  }
  pub fn is_reentrant(&mut self, v: bool) {
    self.is_reentrant = v;
  }
  pub fn display_version(&mut self, v: bool) {
    self.display_version = v;
  }
}
