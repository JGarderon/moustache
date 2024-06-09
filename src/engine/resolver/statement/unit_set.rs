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
      None => return Err(create_internal_error!("Statement can't be empty")),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        key = source[s..e].to_string();
        break;
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in first part (must be Token::Symbol)",
          t
        )));
      }
    }
  }
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "Statement must be complete (token 'equal' not found, premature end)"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      Token::Equal => break,
      t => {
        return Err(create_internal_error!(
          "Incorrect token after the first part (must be Token::Equal)",
          format!("token found: {})", t)
        ))
      }
    }
  }
  let mut value: Vec<String> = vec![];
  let mut begining: bool = true;
  let mut operator: bool = false;
  let mut if_part: bool = false;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        if begining {
          return Err(create_internal_error!("The second part cannot be empty"));
        } else if operator == false {
          return Err(create_internal_error!(
            "Invalid ending : an operator without symbol or text after"
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
            Ok(Some(v)) => value.push(v.clone()),
            Ok(None) => {
              return Err(create_internal_error!(format!(
                "Undefined variable '{}' (position {} ~> {})",
                key, s, e
              )))
            }
            Err(err) => return Err(create_internal_error!(err)),
          }
          operator = true;
        } else {
          return Err(create_internal_error!(
            "Operator missing between symbol or text",
            format!("error on symbol : '{}' - position {} ~> {}", key, s, e)
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
            format!(
              "error on text : '{}' - position {} ~> {}",
              source[s..e].to_string(),
              s,
              e
            )
          ));
        }
      }
      Token::Plus => {
        if operator == true {
          operator = false;
        } else {
          return Err(create_internal_error!(
            "Symbol or text missing before operator"
          ));
        }
      }
      Token::Exclamation => {
        if operator == true {
          if_part = true;
          break;
        } else {
          return Err(create_internal_error!(
            "An 'if' part cannot directly follow an operator"
          ));
        }
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in second part (must be Token::Symbol)",
          t
        )))
      }
    }
  }
  let mut empty: bool = true;
  if if_part {
    loop {
      let token = match iter_tokens.next() {
        Some(t) => t,
        None => {
          return Err(create_internal_error!(
            "An 'if' part can't be empty (first token)"
          ))
        }
      };
      match token {
        Token::Space(_) => (),
        &Token::Symbol(s, e) => {
          let s: &str = &source[s..e];
          if s == "if" {
            break;
          } else {
            return Err(create_internal_error!(format!(
              "Found '{}' in 'if' part of statement (must be Token::Symbol['if'])",
              s
            )));
          }
        }
        t => {
          return Err(create_internal_error!(format!(
            "Found '{}' in 'if' part of statement (must be Token::Symbol['if'])",
            t
          )));
        }
      }
    }
    loop {
      let token = match iter_tokens.next() {
        Some(t) => t,
        None => {
          return Err(create_internal_error!(
            "An 'if' part can't be empty (second token)"
          ))
        }
      };
      match token {
        Token::Space(_) => (),
        &Token::Symbol(s, e) => match &source[s..e] {
          "unset" => break,
          "setted" => {
            empty = false;
            break;
          }
          s => {
            return Err(create_internal_error!(format!(
              "Found '{}' in 'if' part of statement (must be Token::Symbol['unset' or 'setted'])",
              s
            )))
          }
        },
        t => {
          return Err(create_internal_error!(format!(
            "Found '{}' in 'if' part of statement (must be Token::Symbol['if'])",
            t
          )));
        }
      }
    }
  }
  let setting: bool = if if_part {
    let exists = match env.get(&key) {
      Ok(Some(_)) => true,
      Ok(None) => false,
      Err(err) => return Err(create_internal_error!(err)),
    };
    if empty {
      if exists {
        false
      } else {
        true
      }
    } else {
      if exists {
        true
      } else {
        false
      }
    }
  } else {
    true
  };
  if setting {
    match env.set(key, value.into_iter().collect::<String>()) {
      Some(err) => return Err(create_internal_error!(err)),
      None => (),
    }
  }
  Ok(())
}
