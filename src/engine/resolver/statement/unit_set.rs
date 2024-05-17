use core::slice::Iter;
use std::iter::Peekable;

use crate::engine::resolver::statement::Token;
use crate::engine::Environment;

pub fn resolve_unit<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(), String> {
  let mut key: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("set var : can't be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        key = source[s..e].to_string();
        break;
      }
      t => {
        return Err(format!(
          "set var : found '{}' in first part (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("set var : can't be empty (token 'equal' not found)".to_string()),
    };
    match token {
      Token::Space(_) => (),
      Token::Equal => break,
      t => {
        return Err(format!(
          "set var : found '{}' in first part (must be Token::Equal)",
          t
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
          return Err("set var : can't be empty".to_string());
        } else if operator == false {
          return Err(
            "set var : invalid ending (an operator without symbol or text after)".to_string(),
          );
        } else {
          break;
        }
      }
    };
    begining = false;
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        if operator == false {
          key = source[s..e].to_string();
          match env.get(&key) {
            Some(v) => value.push(v.clone()),
            None => return Err(format!("set var : var '{}' in value is undefined", key)),
          }
          operator = true;
        } else {
          return Err("set var : operator missing".to_string());
        }
      }
      &Token::Text(s, e) => {
        if operator == false {
          value.push(source[s..e].to_string());
          operator = true;
        } else {
          return Err("set var : operator missing".to_string());
        }
      }
      &Token::Plus => {
        if operator == true {
          operator = false;
        } else {
          return Err("set var : symbol or text missing before operator".to_string());
        }
      }
      t => {
        return Err(format!(
          "set var : found '{}' in first part (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  env.set(key, value.into_iter().collect::<String>());
  Ok(())
}
