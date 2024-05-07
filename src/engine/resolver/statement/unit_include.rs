use core::iter::Peekable;
use core::slice::Iter;
use std::fs;
use std::path::Path;

use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Environment;

pub fn resolve_unit<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<Part, String> {
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
            None => return Err(format!("undefined var '{}'", value)),
          };
          break;
        }
        t => {
          return Err(format!(
            "token {} not authorized in declarative block statement",
            t
          ))
        }
      },
      None => return Err("unfinished declaration block".to_string()),
    }
  }
  let path = Path::new(&include_path);
  if !path.exists() {
    return Err(format!(
      "include path '{}' don't exist on local system",
      include_path
    ));
  }
  if !path.is_file() {
    return Err(format!(
      "include path '{}' isn't a file on local system",
      include_path
    ));
  }
  let file_content = match fs::read_to_string(path) {
    Ok(s) => s,
    Err(err) => return Err(format!("error during include path : '{}' ", err)),
  };
  Ok(Part::GeneratedText(file_content))
}
