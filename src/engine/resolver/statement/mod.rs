pub mod unit_block;
pub mod unit_call;
pub mod unit_execute;
pub mod unit_if;
pub mod unit_include;
pub mod unit_set;
pub mod unit_find;
pub mod unit_raw;

use crate::add_step_internal_error;
use crate::create_internal_error;
use crate::engine::document::Document;
use crate::engine::document::Part;
use crate::engine::environment::Environment;
use crate::engine::parser;
use crate::engine::parser::Token;
use crate::utils::error::InternalError;

use crate::engine::resolver::statement::unit_block::resolve_unit as resolve_statement_block;
use crate::engine::resolver::statement::unit_call::resolve_unit as resolve_statement_call;
use crate::engine::resolver::statement::unit_execute::resolve_unit as resolve_statement_execute;
use crate::engine::resolver::statement::unit_if::resolve_unit as resolve_statement_if;
use crate::engine::resolver::statement::unit_include::resolve_unit as resolve_statement_include;
use crate::engine::resolver::statement::unit_set::resolve_unit as resolve_statement_set;
use crate::engine::resolver::statement::unit_find::resolve_unit as resolve_statement_find;
use crate::engine::resolver::statement::unit_raw::resolve_unit as resolve_statement_raw;

#[derive(Debug)]
pub struct Resolved {
  pub changed: bool,
  pub stack: Vec<Part>,
}

pub fn resolve_statement<'a>(
  doc: &'a Document,
  doc_position: usize,
  expr: &'a str,
  env: &mut Environment,
) -> Result<(Vec<Part>, usize), InternalError> {
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
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'define block' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
            ))
          }
        },
        "call" => match resolve_statement_call(env, source, &mut iter) {
          Ok(v) => {
            if v.len() > 0 {
              output.extend(v);
            }
            break;
          }
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'call block' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
            ))
          }
        },
        "include" => match resolve_statement_include(env, source, &mut iter) {
          Ok(v) => {
            output.push(v);
            break;
          }
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'include' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
            ))
          }
        },
        "if" => match resolve_statement_if(doc, doc_position, env, source, &mut iter) {
          Ok((v, p)) => {
            output.extend(v);
            position_skip = p;
            break;
          }
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'if' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
            ))
          }
        },
        "raw" => match resolve_statement_raw(doc, doc_position, env, source, &mut iter) {
          Ok((v, p)) => {
            output.extend(v);
            position_skip = p;
            break;
          }
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'raw' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
            ))
          }
        },
        "set" => match resolve_statement_set(env, source, &mut iter) {
          Ok(_) => break,
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'set' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim()),
              "must be = '\x1b[3mset [symbol] = [text or symbol (+ text or symbol (+ ...))]\x1b[0m'"
            ))
          }
        },
        "find" => match resolve_statement_find(env, source, &mut iter) {
          Ok(_) => break,
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'find' statement",
              format!("source = '\x1b[3m{}\x1b[0m'", source.trim()),
              "must be = '\x1b[3mfind ['files' or 'directories' or 'all'] in [text or symbol] to [text or symbol]\x1b[0m'"
            ))
          }
        },
        "execute" => match resolve_statement_execute(doc, doc_position, env, source, &mut iter) {
          Ok(_) => break,
          Err(mut err) => {
            return Err(add_step_internal_error!(
              err,
              "error in 'execute' statement"
            ))
          }
        },
        s => {
          return Err(create_internal_error!(
            format!("invalid action '{}' in statement", s),
            format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
          ))
        }
      },
      parser::Token::Space(_) => (),
      t => {
        return Err(create_internal_error!(
          format!("token {} not authorized in statement", t),
          format!("source = '\x1b[3m{}\x1b[0m'", source.trim())
        ))
      }
    }
  }
  Ok((output, position_skip))
}
