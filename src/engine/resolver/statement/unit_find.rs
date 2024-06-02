use core::slice::Iter;
use std::fs;
use std::iter::Peekable;
use std::path::Path;

use crate::create_internal_error;
use crate::engine::resolver::statement::Token;
use crate::engine::Environment;
use crate::utils::error::InternalError;

#[derive(Debug)]
enum SearchType {
  Files,
  Directories,
  All,
}

pub fn resolve_unit<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(), InternalError> {
  let search_type: SearchType;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => return Err(create_internal_error!("Statement can't be empty")),
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        search_type = match &source[s..e] {
          // .to_lowercase()[..]
          "files" => SearchType::Files,
          "directories" => SearchType::Directories,
          "all" => SearchType::All,
          o => {
            return Err(create_internal_error!(
              "Invalid search type found",
              format!("found '{}' search type", o)
            ))
          }
        };
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
          "Statement must be complete (symbol 'in' not found, premature end)"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) if &source[s..e] == "in" => break,
      &Token::Symbol(s, e) if &source[s..e] != "in" => {
        return Err(create_internal_error!(format!(
          "Found '{}' in second part (must be Token::Symbol['in'])",
          &source[s..e]
        )));
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in second part (must be Token::Symbol['in'])",
          t
        )));
      }
    }
  }
  let pattern: &str;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "Statement must be complete (text or symbol not found after 'in' symbol, premature end)"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        let key: String = source[s..e].to_string();
        pattern = match env.get(&key) {
          Some(v) => v,
          None => {
            return Err(create_internal_error!(format!(
              "Undefined variable '{}' as pattern",
              key
            )))
          }
        };
        break;
      }
      &Token::Text(s, e) => {
        pattern = &source[s..e];
        break;
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in third part (must be Token::Symbol['to'])",
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
          "Statement must be complete (symbol 'to' not found, premature end)"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) if &source[s..e] == "to" => break,
      &Token::Symbol(s, e) if &source[s..e] != "to" => {
        return Err(create_internal_error!(format!(
          "Found '{}' in fourth part (must be Token::Symbol['to'])",
          &source[s..e]
        )));
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in fourth part (must be Token::Symbol['to'])",
          t
        )));
      }
    }
  }
  let destination: String;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "Statement must be complete (text or symbol not found after 'to' symbol, premature end)"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => {
        destination = source[s..e].to_string();
        break;
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in final part (must be Token::Symbol)",
          t
        )));
      }
    }
  }
  if pattern == "" {
    return Err(create_internal_error!(
      "Invalid empty path: for the local directory, please set value as '.'"
    ));
  }
  let path = if pattern.contains("*") {
    let (mut left, _) = pattern.split_once('*').unwrap();
    if left == "" {
      left = "./";
    }
    Path::new(left)
  } else {
    Path::new(pattern)
  };
  if path.is_file() {
    env.set(destination, pattern.to_string());
  } else if path.is_dir() {
    let items = fs::read_dir(path).unwrap();
    let mut results: Vec<String> = vec![];
    for item in items {
      match item {
        Ok(directory_item) => match search_type {
          SearchType::Files if directory_item.path().is_file() => {
            results.push(directory_item.path().display().to_string())
          }
          SearchType::Directories if directory_item.path().is_dir() => {
            results.push(directory_item.path().display().to_string())
          }
          SearchType::All => results.push(directory_item.path().display().to_string()),
          _ => (),
        },
        Err(err) => {
          return Err(create_internal_error!(
            format!("Error during reading directory '{}'", pattern),
            format!("Result of read_dir = '{}'", err.to_string())
          ))
        }
      }
    }
    match pattern.split_once('*') {
      Some((pattern_left, pattern_right)) => {
        results.retain(|item| item.starts_with(pattern_left));
        results.retain(|item| item.ends_with(pattern_right));
      }
      None => (),
    }
    env.set(destination, results.join("\n"));
  } else {
    return Err(create_internal_error!(format!(
      "Invalid path '{}' (not a directory or a regular file)",
      pattern
    )));
  }
  Ok(())
}
