pub mod unit_block;
pub mod unit_call;
pub mod unit_if;
pub mod unit_include;

use crate::engine::document::Document;
use crate::engine::document::Part;
use crate::engine::environment::Environment;
use crate::engine::parser;
use crate::engine::parser::Token;

use crate::engine::resolver::statement::unit_block::resolve_unit as resolve_statement_block;
use crate::engine::resolver::statement::unit_call::resolve_unit as resolve_statement_call;
use crate::engine::resolver::statement::unit_if::resolve_unit as resolve_statement_if;
use crate::engine::resolver::statement::unit_include::resolve_unit as resolve_statement_include;

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
        "include" => match resolve_statement_include(env, source, &mut iter) {
          Ok(v) => {
            output.push(v);
            break;
          }
          Err(err) => return Err(format!("error in include statement : {}", err)),
        },
        "if" => match resolve_statement_if(doc, doc_position, env, source, &mut iter) {
          Ok(p) => {
            position_skip = p;
            break;
          }
          Err(err) => return Err(format!("error in if statement : {}", err)),
        },
        s => return Err(format!("invalid action '{}' in statement", s)),
      },
      parser::Token::Space(_) => (),
      t => return Err(format!("token {} not authorized in statement", t)),
    }
  }
  Ok((output, position_skip))
}
