use core::slice::Iter;
use std::iter::Peekable;

use crate::engine::document::Document;
use crate::engine::document::Part;
use crate::engine::environment::Environment;
use crate::engine::parser;
use crate::engine::parser::Token;

#[derive(Debug)]
pub struct Resolved {
  pub changed: bool,
  pub stack: Vec<Part>,
}

fn add_string_to_another(s1: &mut String, s2: &mut String) {
  s1.retain(|c| !r#"\"#.contains(c));
  s2.push_str(s1);
}

fn resolve_statement_block<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<usize, String> {
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
          return Err(format!(
            "token {} not authorized in declarative block statement",
            t
          ))
        }
      },
      None => return Err("unfinished declaration block".to_string()),
    };
  }
  let mut iter_parts = doc.stack.iter().skip(doc_position).enumerate();
  let mut block_ending_position: usize;
  loop {
    let part = match iter_parts.next() {
      Some((position, part)) => {
        block_ending_position = position;
        part
      }
      None => return Err("unfinished block".to_string()),
    };
    match part {
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim() == "endblock" => break,
      _ => (),
    }
  }
  let mut block: Vec<Part> = vec![];
  block.extend_from_slice(&doc.stack[doc_position + 1..doc_position + block_ending_position]);
  env.set_block(block_name, block);
  return Ok(block_ending_position);
}

fn resolve_statement_call<'a>(
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<Vec<Part>, String> {
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
          return Err(format!(
            "token {} not authorized in declarative block statement",
            t
          ))
        }
      },
      None => return Err("unfinished declaration block".to_string()),
    };
  }
  match env.get_block(&block_name) {
    Some(v) => Ok(v.clone()),
    None => Err("undefined block".to_string()),
  }
}

fn resolve_statement<'a>(
  doc: &'a Document,
  doc_position: usize,
  expr: &'a str,
  env: &mut Environment,
) -> Result<(Vec<Part>, usize), String> {
  let source: &str = &expr[2..expr.len() - 2];
  let tokens: Vec<parser::Token> = match parser::parse(source) {
    Ok(t) => t,
    Err(err) => return Err(err),
  };
  let mut output: Vec<Part> = vec![];
  let mut iter = tokens.iter().peekable();
  let mut position_skip: usize = 0;
  loop {
    let token = match iter.next() {
      Some(t) => t,
      None => break,
    };
    match token {
      &parser::Token::Symbol(s, e) => match &source[s..e] {
        "block" => match resolve_statement_block(doc, doc_position, env, source, &mut iter) {
          Ok(p) => {
            position_skip = p;
            break;
          }
          Err(err) => return Err(format!("error in declaring block statement : {}", err)),
        },
        "call" => match resolve_statement_call(env, source, &mut iter) {
          Ok(v) => {
            if v.len() > 0 {
              output.extend(v);
            }
            break;
          }
          Err(err) => return Err(format!("error in call block statement : {}", err)),
        },
        s => return Err(format!("invalid action {} in statement", s)),
      },
      parser::Token::Space(_) => (),
      t => return Err(format!("token {} not authorized in statement", t)),
    }
  }
  Ok((output, position_skip))
}

fn resolve_expression<'a>(
  _doc: &'a Document,
  expr: &'a str,
  env: &mut Environment,
) -> Result<Part, String> {
  let source: &str = &expr[2..expr.len() - 2];
  let tokens: Vec<parser::Token> = match parser::parse(source) {
    Ok(t) => t,
    Err(err) => return Err(err),
  };
  let mut output = "".to_string();
  let mut iter = tokens.iter().peekable();
  let mut is_begining: bool = true;
  loop {
    let token = match iter.next() {
      Some(t) => t,
      None => break,
    };
    match token {
      parser::Token::Text(_, _) if is_begining == false => {
        return Err(format!("invalid position's text in expression"))
      }
      parser::Token::Symbol(s, e) if is_begining == false => {
        return Err(format!(
          "invalid position's symbol in expression {}:{}",
          s, e
        ))
      }
      parser::Token::Plus if is_begining => {
        return Err(format!(
          "token {} not authorized in expression begining",
          token
        ))
      }

      parser::Token::Text(s, e) if is_begining => {
        add_string_to_another(&mut source[*s..*e].to_string(), &mut output);
        is_begining = false;
      }
      parser::Token::Symbol(s, e) if is_begining => {
        let symbol = source[*s..*e].to_string();
        match env.get(&symbol) {
          Some(v) => output.push_str(v),
          None => return Err(format!("key '{}' not found in env (expr)", symbol)),
        }
        is_begining = false;
      }
      parser::Token::Plus if is_begining == false => {
        loop {
          match iter.next() {
            Some(parser::Token::Text(s, e)) => {
              add_string_to_another(&mut source[*s..*e].to_string(), &mut output);
              is_begining = false;
              break;
            }
            Some(parser::Token::Symbol(s, e)) => {
              let symbol = &source[*s..*e].to_string();
              match env.get(&symbol) {
                Some(v) => add_string_to_another(
                  &mut v[..].to_string(), // pas terrible...
                  &mut output,
                ),
                None => return Err(format!("key '{}' not found in env ('plus' expr)", symbol)),
              }
              is_begining = false;
              break;
            }
            Some(parser::Token::Space(_)) => continue,
            Some(t) => {
              return Err(format!(
                "token {} not authorized in second part of 'plus' expression",
                t
              ))
            }
            None => {
              return Err(format!(
                "no token found for second part in 'plus' expression"
              ))
            }
          };
        }
      }
      parser::Token::Space(_) => (),
      t => return Err(format!("token {} not authorized in expression", t)),
    }
  }
  Ok(Part::GeneratedText(output))
}

pub fn resolve<'a>(doc: &'a Document, env: &mut Environment) -> Result<Resolved, String> {
  let mut position: usize = 0;
  let max: usize = doc.stack_len();
  let mut result: Vec<Part> = vec![];
  let mut changed: bool = false;
  loop {
    if position >= max {
      break;
    }
    match doc.stack_get(position) {
      Some(&Part::StaticText(s, e)) => result.push(Part::StaticText(s, e)),
      Some(&Part::Expression(s, e)) => {
        match resolve_expression(doc, &doc.source[s..e], env) {
          Ok(p) => result.push(p),
          Err(err) => return Err(err),
        }
        changed = true;
      }
      Some(&Part::Statement(s, e)) => {
        match resolve_statement(doc, position, &doc.source[s..e], env) {
          Ok((v, p)) => {
            if v.len() > 0 {
              result.extend(v.into_iter());
            }
            position += p;
          }
          Err(err) => return Err(err),
        }
        changed = true;
      }
      Some(Part::GeneratedText(_)) | Some(Part::Comment(_, _)) => (),
      None => break,
    }
    position += 1;
  }
  Ok(Resolved {
    changed: changed,
    stack: result,
  })
}
