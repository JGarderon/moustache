use core::iter::Peekable;
use core::slice::Iter;

use crate::create_internal_error;
use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Environment;
use crate::utils::error::InternalError;

pub fn resolve_unit<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<Vec<Part>, InternalError> {
  let block_name: String;
  loop {
    match iter_tokens.next() {
      Some(token) => match token {
        Token::Space(_) => (),
        &Token::Text(s, e) => {
          block_name = source[s..e].to_string();
          break;
        }
        t => {
          return Err(create_internal_error!(format!(
            "token {} not authorized in declarative block statement",
            t
          )));
        }
      },
      None => return Err(create_internal_error!("unfinished declaration block")),
    };
  }
  match env.get_block(&block_name) {
    Some(v) => Ok(v.clone()),
    None => Err(create_internal_error!("undefined block")),
  }
}
