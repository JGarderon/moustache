use core::slice::Iter;
use std::iter::Peekable;

use crate::create_internal_error;
use crate::engine::resolver::statement::Token;
use crate::engine::Environment;
use crate::utils::error::InternalError;

pub fn resolve_unit<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(), InternalError> {
  let mut key: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "Statement can't be empty",
          "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        key = source[s..e].to_string();
        break;
      }
      t => {
        return Err(create_internal_error!(
          format!("Found '{}' in first part (must be Token::Symbol)", t),
          "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
          format!("found statement (here with trim !) = '\x1b[3m{}\x1b[0m'", source.trim())
        ));
      }
    }
  }
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "Statement must be complete (token 'equal' not found, premature end)",
          "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
          format!("found statement (here with trim !) = '\x1b[3m{}\x1b[0m'", source.trim())
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      Token::Equal => break,
      t => {
        return Err(create_internal_error!(
          "Incorrect token after the first part (must be Token::Equal)",
          "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
          format!(
            "found statement (here with trim !) = '\x1b[3m{}\x1b[0m' (incorrect token : {})",
            source.trim(),
            t
          )
        ))
      }
    }
  }
  let mut value: Vec<String> = vec![];
  let mut begining: bool = true;
  let mut operator: bool = false;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        if begining {
          return Err(create_internal_error!(
            "The second part cannot be empty",
            "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'"
          ));
        } else if operator == false {
          return Err(create_internal_error!(
            "Invalid ending : an operator without symbol or text after",
            "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
            format!("found statement (here with trim !) = '\x1b[3m{}\x1b[0m'", source.trim())
          ));
        } else {
          break;
        }
      }
    };
    begining = false;
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        key = source[s..e].to_string();
        if operator == false {
          match env.get(&key) {
            Some(v) => value.push(v.clone()),
            None => {
              return Err(create_internal_error!(format!(
                "Undefined variable '{}' (position {} ~> {})",
                key, s, e
              )))
            }
          }
          operator = true;
        } else {
          return Err(create_internal_error!(
            "Operator missing between symbol or text",
            "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
            format!(
              "found statement (here with trim !) = '\x1b[3m{}\x1b[0m' (error on symbol : '{}' - position {} ~> {})",
              source.trim(),
              key,
              s,
              e
            )
          ));
        }
      }
      &Token::Text(s, e) => {
        if operator == false {
          value.push(source[s..e].to_string());
          operator = true;
        } else {
          return Err(create_internal_error!(
            "Operator missing between symbol or text",
            "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
            format!(
              "found statement (here with trim !) = '\x1b[3m{}\x1b[0m' (error on text : '{}' - position {} ~> {})",
              source.trim(),
              source[s..e].to_string(),
              s,
              e
            )
          ));
        }
      }
      &Token::Plus => {
        if operator == true {
          operator = false;
        } else {
          return Err(create_internal_error!(
            "Symbol or text missing before operator",
            "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'"
          ));
        }
      }
      t => {
        return Err(create_internal_error!(
          format!("Found '{}' in first part (must be Token::Symbol)", t),
          "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'",
          format!("found statement (here with trim !) = '\x1b[3m{}\x1b[0m'", source.trim())
        ))
      }
    }
  }
  env.set(key, value.into_iter().collect::<String>());
  Ok(())
}
