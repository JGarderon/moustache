use core::slice::Iter;
use std::iter::Peekable;

use crate::engine::extensions::default;
use crate::engine::extensions::Context;
use crate::engine::extensions::Value;
use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::engine::Environment;

fn resolve_fct<'a>(
  begining: bool,
  anterior_result: &Value,
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<Value, String> {
  let mut fct: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("execute var : can't be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        fct = source[s..e].to_string();
        break;
      }
      t => {
        return Err(format!(
          "execute var : found '{}' in first part (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  println!("fct = {:?}", fct);
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("execute var : can't be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::ParenthesisOpening => break,
      t => {
        return Err(format!(
          "execute var : found '{}' in opening separator (must be Token::ParenthesisOpening)",
          t
        ))
      }
    }
  }
  let mut args: Vec<Value> = vec![];
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("execute var : must be ended".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        args.push(Value::Symbol(source[s..e].to_string()));
      }
      &Token::Text(s, e) => {
        args.push(Value::Text(source[s..e].to_string()));
      }
      &Token::ParenthesisEnding => break,
      t => {
        return Err(format!(
          "execute var : found '{}' in first part (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  println!("fct = {:?} ; args = {:?}", fct, args);
  let f: Vec<_> = fct.splitn(2, '.').collect();
  if f.len() < 2 {
    return Err(format!("invalid name function '{}'", fct));
  }
  match *f.get(0).unwrap() {
    "default" => default::execute(Context::new(
      begining,
      anterior_result,
      doc,
      doc_position,
      env,
      *f.get(1).unwrap(),
      args,
    )),
    n => return Err(format!("extension '{}' not found", n)),
  }
}

fn cast(results: Value) -> Result<Option<Vec<Part>>, String> {
  println!("cast calling = {:?}", results);
  Ok(None)
}

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(), String> {
  // let mut key: String;
  // loop {
  //   let token = match iter_tokens.next() {
  //     Some(t) => t,
  //     None => return Err("execute var : can't be empty".to_string()),
  //   };
  //   match token {
  //     Token::Space(_) => (),
  //     &Token::Symbol(s, e) => {
  //       key = source[s..e].to_string();
  //       break;
  //     }
  //     t => {
  //       return Err(format!(
  //         "execute var : found '{}' in first part (must be Token::Symbol)",
  //         t
  //       ))
  //     }
  //   }
  // }
  // println!("key = {:?}", key);
  // loop {
  //   let token = match iter_tokens.next() {
  //     Some(t) => t,
  //     None => return Err("execute var : can't be empty (token 'equal' not found)".to_string()),
  //   };
  //   match token {
  //     Token::Space(_) => (),
  //     Token::Equal => break,
  //     t => {
  //       return Err(format!(
  //         "execute var : found '{}' in first part (must be Token::Equal)",
  //         t
  //       ))
  //     }
  //   }
  // }
  // println!("key = {:?} ; fct = {:?}", key, fct);

  let mut begining: bool = true;
  let mut result: Value = Value::Void;
  loop {
    match resolve_fct(
      begining,
      &result,
      doc,
      doc_position,
      env,
      source,
      iter_tokens,
    ) {
      Ok(r) => result = r,
      Err(err) => {
        return Err(format!(
          "execute statement : error during execution : '{}'",
          err
        ))
      }
    }
    begining = false;
    break;
  }

  println!("cast = {:?}", cast(result));

  // let mut begining: bool = true;
  // let mut operator: bool = false;
  // loop {
  //   let token = match iter_tokens.next() {
  //     Some(t) => t,
  //     None => {
  //       if begining {
  //         return Err("set var : can't be empty".to_string());
  //       } else if operator == false {
  //         return Err(
  //           "set var : invalid ending (an operator without symbol or text after)".to_string(),
  //         );
  //       } else {
  //         break;
  //       }
  //     }
  //   };
  //   begining = false;
  //   match token {
  //     Token::Space(_) => (),
  //     &Token::Symbol(s, e) => {
  //       if operator == false {
  //         key = source[s..e].to_string();
  //         match env.get(&key) {
  //           Some(v) => value.push(v.clone()),
  //           None => return Err(format!("set var : var '{}' in value is undefined", key)),
  //         }
  //         operator = true;
  //       } else {
  //         return Err("set var : operator missing".to_string());
  //       }
  //     }
  //     &Token::Text(s, e) => {
  //       if operator == false {
  //         value.push(source[s..e].to_string());
  //         operator = true;
  //       } else {
  //         return Err("set var : operator missing".to_string());
  //       }
  //     }
  //     &Token::Plus => {
  //       if operator == true {
  //         operator = false;
  //       } else {
  //         return Err("set var : symbol or text missing before operator".to_string());
  //       }
  //     }
  //     t => {
  //       return Err(format!(
  //         "set var : found '{}' in first part (must be Token::Symbol)",
  //         t
  //       ))
  //     }
  //   }
  // }
  // env.set(key, value.into_iter().collect::<String>());
  Ok(())
}
