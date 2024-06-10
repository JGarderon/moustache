use core::iter::Peekable;
use core::slice::Iter;

use crate::create_internal_error;
use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::engine::Environment;
use crate::utils::error::InternalError;

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(Vec<Part>, usize), InternalError> {
  let mut iter_parts = doc.stack.iter().skip(doc_position).enumerate();
  let mut block_ending_position: usize;
  let mut i = 0;
  loop {
    let part = match iter_parts.next() {
      Some((position, part)) => {
        block_ending_position = position;
        part
      }
      None => return Err(create_internal_error!("Unfinished block 'for'")),
    };
    match part {
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim().starts_with("for") => i += 1,
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim() == "endfor" => {
        i -= 1;
        if i == 0 {
          break;
        }
      }
      _ => (),
    }
  }
  let destination: &str = loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => {
        return Err(create_internal_error!(
          "Statement must be complete (start symbol not found, premature end)"
        ))
      }
    };
    match token {
      Token::Space(_) => (),
      &Token::Symbol(s, e) => break &source[s..e],
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in first part (must be Token::Symbol)",
          t
        )));
      }
    }
  };
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
  let list: String = loop {
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
        match env.get(&key) {
          Ok(Some(v)) => break v.to_string(),
          Ok(None) => {
            return Err(create_internal_error!(format!(
              "Undefined variable '{}' as pattern",
              key
            )))
          }
          Err(err) => return Err(create_internal_error!(err)),
        };
      }
      &Token::Text(s, e) => break (&source[s..e]).to_string(),
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' in third part (must be Token::Symbol['to'])",
          t
        )));
      }
    }
  };
  let mut optional_part: bool = false;
  loop {
    let token = match iter_tokens.next() {
      Some(t) => t,
      None => break,
    };
    match token {
      Token::Space(_) => (),
      Token::Exclamation => {
        optional_part = true;
        break;
      }
      t => {
        return Err(create_internal_error!(format!(
          "Found '{}' as optional part separator (must be Token::Exclamation)",
          t
        )));
      }
    }
  }
  let mut split_char: &str = "\n";
  if optional_part {
    split_char = loop {
      let token = match iter_tokens.next() {
        Some(t) => t,
        None => return Err(create_internal_error!("Optional part can't be empty")),
      };
      match token {
        Token::Space(_) => (),
        &Token::Symbol(s, e) => {
          let key = source[s..e].to_string();
          match env.get(&key) {
            Ok(Some(v)) => break v,
            Ok(None) => {
              return Err(create_internal_error!(format!(
                "Undefined variable '{}' as split char",
                key
              )))
            }
            Err(err) => return Err(create_internal_error!(err)),
          }
        }
        &Token::Text(s, e) => break &source[s..e],
        t => {
          return Err(create_internal_error!(format!(
            "Found '{}' in first part (must be Token::Symbol or Token::Text)",
            t
          )));
        }
      }
    };
  }
  let mut results: Vec<Part> = vec![];
  for item in list.split(split_char) {
    let mut result: Vec<Part> = vec![Part::GeneratedText(format!(
      "{{% set {} = \"{}\" %}}",
      destination,
      item.replace("\"", "\\\"")
    ))];
    result.extend(doc.stack[doc_position + 1..doc_position + block_ending_position].to_vec());
    results.extend(result);
  }
  Ok((results, block_ending_position))
}
