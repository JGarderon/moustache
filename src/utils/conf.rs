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
  pub no_extensions: bool,
  pub error_formatting: bool,
  pub skip_first_line: bool,
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
      no_extensions: false,
      error_formatting: false,
      skip_first_line: false,
    }
  }
  pub fn display(&self) -> String {
    format!(
      r#"Configuration:
input:                 {:?}
output:                {:?}
variables:             {}

is_helping:            '{}'
is_helping_extensions: '{}'
is_debugging:          '{}'
is_reentrant:          '{}'
display_version:       '{}'
no_extensions:         '{}'
error_formatting:      '{}'
skip_first_line:       '{}'"#,
      self.input,
      self.output,
      {
        let mut s: Vec<String> = vec!["".to_string()];
        for (key, val) in self.variables.iter() {
          s.push(format!("      - {} -> {}", key, val))
        }
        s.join("\n")
      },
      self.is_helping,
      self.is_helping_extensions,
      self.is_debugging,
      self.is_reentrant,
      self.display_version,
      self.no_extensions,
      self.error_formatting,
      self.skip_first_line,
    )
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
  pub fn no_extensions(&mut self, v: bool) {
    self.no_extensions = v;
  }
  pub fn error_formatting(&mut self, v: bool) {
    self.error_formatting = v;
  }
  pub fn skip_first_line(&mut self, v: bool) {
    self.skip_first_line = v;
  }
}
