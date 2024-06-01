use core::slice::Iter;
use std::iter::Peekable;

use crate::add_step_internal_error;
use crate::create_internal_error;
use crate::engine::extensions;
use crate::engine::extensions::Context;
use crate::engine::extensions::Value;
use crate::engine::resolver::statement::Token;
use crate::engine::Document;
use crate::engine::Environment;
use crate::utils::error::InternalError;

fn resolve_fct<'a>(
  context: &mut Context,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Option<String> {
  let fct: &str;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Some("The call expression cannot be empty".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        fct = &context.source[s..e];
        break;
      }
      t => {
        return Some(format!(
          "Found '{}' for name function (must be Token::Symbol)",
          t
        ))
      }
    }
  }
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Some("The call expression must have parenthesized arguments".to_string()),
    };
    match token {
      Token::Space(_) => (),
      &Token::ParenthesisOpening => break,
      t => {
        return Some(format!(
          "Found '{}' in opening separator (must be Token::ParenthesisOpening)",
          t
        ))
      }
    }
  }
  let mut args: Vec<Value> = vec![];
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Some("The call expression must be terminated by a closing parenthesis".to_string())
      }
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
          "Found '{}' in first part (must be Token::Symbol for the function name)",
          t
        ))
      }
    }
  }
  let f: Vec<&str> = fct.splitn(2, '.').collect();
  if f.len() < 2 {
    return Some(format!(
      "Invalid function name found '{}' (must be in the form '[extension name].[function name]')",
      fct
    ));
  }
  context.fct_name = f.get(1).unwrap();
  context.args = args;
  extensions::execute(
    f.get(0).unwrap(),
    context,
  )
}

fn cast(env: &mut Environment, results: Option<Value>) -> Result<String, String> {
  match results {
    Some(Value::Text(v)) => return Ok(v.to_string()),
    Some(Value::Symbol(v)) => match env.get(&v) {
      Some(r) => return Ok(r.to_string()),
      None => {
        return Err(format!(
          "Error during casting with unfound environment key '{}'",
          v
        ))
      }
    },
    Some(Value::Vector(vector)) => {
      let mut iter = vector.into_iter().rev();
      let mut tmp: Vec<String> = vec![];
      while let Some(v) = iter.next() {
        match cast(env, Some(v)) {
          Ok(r) => tmp.push(r),
          Err(err) => return Err(err),
        }
      }
      return Ok(tmp.join("\n"));
    }
    Some(Value::True) => return Ok("true".to_string()),
    Some(Value::Number(n)) => return Ok(n.to_string()),
    Some(Value::False) => return Ok("".to_string()),
    Some(Value::Void) => return Ok("".to_string()),
    None => return Ok("".to_string()),
  }
}

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(), InternalError> {
  if doc.conf.no_extensions {
    return Err(create_internal_error!(
      "Execute statement found: not authorized by configuration",
      "the --no-extensions argument was specified"
    ));
  }
  let key: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "The statement can't be empty".to_string()
        ));
      }
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
          "At least one function must be called and preceded by an equal sign"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      Token::Equal => break,
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' instead of the '=' sign (must be Token::Equal)",
          t
        )));
      }
    }
  }
  let mut context = Context::new(doc, doc_position, env, source);
  'outer: loop {
    match resolve_fct(&mut context, iter_tokens) {
      Some(err) => {
        let mut err = create_internal_error!(err);
        return Err(add_step_internal_error!(
          err,
          "An error occurred during execution"
        ));
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
          return Err(
            create_internal_error!(format!(
            "The pipe symbol '|' is required between two function calls, found '{}' (must be Token::Pipe)",
            t
          )));
        }
      }
    }
  }
  let Context {
    begining: _,
    result,
    doc: _,
    doc_position: _,
    env,
    source: _,
    fct_name: _,
    args: _,
  } = context;
  let value = match cast(env, result) {
    Ok(value) => value,
    Err(err) => {
      let mut err = create_internal_error!(err);
      return Err(add_step_internal_error!(
        err,
        "Error during casting of the final function return"
      ));
    }
  };
  env.set(key, value);
  Ok(())
}
