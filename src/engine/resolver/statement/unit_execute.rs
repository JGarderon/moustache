use core::slice::Iter;
use std::iter::Peekable;

use crate::engine::extensions::default;
use crate::engine::extensions::Context;
use crate::engine::extensions::Value;
use crate::engine::resolver::statement::Token;
use crate::engine::Document;
use crate::engine::Environment;

fn resolve_fct<'a>(
  context: &mut Context,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Option<String> {
  let fct: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Some("execute var : can't be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        fct = context.source[s..e].to_string();
        break;
      }
      t => {
        return Some(format!(
          "execute var : found '{}' in first part (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Some("execute var : can't be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::ParenthesisOpening => break,
      t => {
        return Some(format!(
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
      None => return Some("execute var : must be ended".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        args.push(Value::Symbol(context.source[s..e].to_string()));
      }
      &Token::Text(s, e) => {
        args.push(Value::Text(context.source[s..e].to_string()));
      }
      &Token::ParenthesisEnding => break,
      t => {
        return Some(format!(
          "execute var : found '{}' in first part (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  let f: Vec<&str> = fct.splitn(2, '.').collect();
  if f.len() < 2 {
    return Some(format!("invalid name function '{}'", fct));
  }
  context.fct_name = f.get(1).unwrap().to_string();
  context.args = args;
  match *f.get(0).unwrap() {
    "default" => default::execute(context),
    n => Some(format!("extension '{}' not found", n)),
  }
}

fn cast(env: &mut Environment, results: Option<Value>) -> Result<String, String> {
  let mut value: String = "".to_string();
  match results {
    Some(Value::Text(v)) => value = v.to_string(),
    Some(Value::Symbol(v)) => {
      value = match env.get(&v) {
        Some(r) => r.to_string(),
        None => {
          return Err(format!(
            "execute var : error during casting with unfound env key '{}'",
            v
          ))
        }
      };
    }
    Some(Value::Vector(vector)) => {
      let mut iter = vector.into_iter().rev();
      let mut tmp: Vec<String> = vec![];
      while let Some(v) = iter.next() {
        match cast(env, Some(v)) {
          Ok(r) => tmp.push(r),
          Err(err) => return Err(err),
        }
      }
      value = tmp.join("\n");
    }
    Some(Value::True) => value = "true".to_string(),
    Some(Value::False) => (),
    Some(Value::Void) => (),
    None => (),
  }
  Ok(value)
}

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(), String> {
  let key: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("execute var : can't be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        key = source[s..e].to_string();
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
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err("execute var : can't be empty (token 'equal' not found)".to_string()),
    };
    match token {
      Token::Space(_) => (),
      Token::Equal => break,
      t => {
        return Err(format!(
          "execute var : found '{}' in first part (must be Token::Equal)",
          t
        ))
      }
    }
  }
  let mut context = Context::new(doc, doc_position, env, source);
  'outer: loop {
    match resolve_fct(&mut context, iter_tokens) {
      Some(err) => {
        return Err(format!(
          "execute statement : error during execution : '{}'",
          err
        ))
      }
      None => (),
    }
    context.begining = false;
    'inner: loop {
      let token = match iter_tokens.next() {
        Some(t) => t,
        None => break 'outer,
      };
      match token {
        Token::Space(_) => (),
        Token::Pipe => break 'inner,
        t => {
          return Err(format!(
            "execute var : found '{}' in first part (must be Token::Pipe)",
            t
          ))
        }
      }
    }
  }
  let Context {
    begining: _,
    result: result,
    doc: _,
    doc_position: _,
    env: env,
    source: _,
    fct_name: _,
    args: _,
  } = context;
  let value = match cast(env, result) {
    Ok(value) => value,
    Err(err) => return Err(format!("execute var : error during casting : '{}'", err)),
  };
  env.set(key, value);
  Ok(())
}
