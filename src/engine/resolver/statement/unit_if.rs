use core::iter::Peekable;
use core::slice::Iter;

use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::engine::Environment;

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  _env: &mut Environment,
  _source: &'a str,
  _iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<usize, String> {
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
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim() == "endif" => break,
      _ => (),
    }
  }
  let mut if_block: Vec<Part> = vec![];
  if_block.extend_from_slice(&doc.stack[doc_position + 1..doc_position + block_ending_position]);
  println!("if_block = {:?}", if_block);

  Ok(block_ending_position)
}
