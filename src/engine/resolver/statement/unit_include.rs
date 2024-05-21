use core::iter::Peekable;
use core::slice::Iter;
use std::fs;
use std::path::Path;

use crate::create_internal_error;
use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Environment;
use crate::utils::error::InternalError;

pub fn resolve_unit<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<Part, InternalError> {
  let include_path: String;
  loop {
    match iter_tokens.next() {
      Some(token) => match token {
        Token::Space(_) => (),
        &Token::Text(s, e) => {
          include_path = source[s..e].to_string();
          break;
        }
        &Token::Symbol(s, e) => {
          let value = source[s..e].to_string();
          include_path = match env.get(&value) {
            Some(v) => v.clone(),
            None => {
              return Err(create_internal_error!(format!(
                "Undefined variable '{}'",
                value
              )))
            }
          };
          break;
        }
        t => {
          return Err(create_internal_error!(format!(
            "Token '{}' is not allowed in a block declaration.",
            t
          )))
        }
      },
      None => return Err(create_internal_error!("Unterminated block declaration.")),
    }
  }
  let path = Path::new(&include_path);
  if !path.exists() {
    return Err(create_internal_error!(
      "The path for file inclusion does not exist on your local system.",
      format!("Found '{}' path", include_path)
    ));
  }
  if !path.is_file() {
    return Err(create_internal_error!(
      "The path for inclusion is not a file on your local system.",
      format!("Found '{}' path", include_path)
    ));
  }
  let file_content = match fs::read_to_string(path) {
    Ok(s) => s,
    Err(err) => {
      return Err(create_internal_error!(format!(
        "An error occurred during file inclusion : '{}' ",
        err
      )))
    }
  };
  Ok(Part::GeneratedText(file_content))
}
