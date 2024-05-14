// use core::option::Iter;
use core::iter::Peekable;
use core::slice::Iter;

use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::engine::Environment;

pub fn resolve_unit<'a>(
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