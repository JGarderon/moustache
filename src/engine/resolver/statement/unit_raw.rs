use crate::create_internal_error;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::InternalError;

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
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
      None => return Err(create_internal_error!("Unfinished 'raw' statement")),
    };
    match part {
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim() == "raw" => i += 1,
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim() == "endraw" => {
        i -= 1;
        if i == 0 {
          break;
        }
      }
      _ => (),
    }
  }
  Ok((
    doc.stack[doc_position + 1..doc_position + block_ending_position].to_vec(),
    block_ending_position,
  ))
}
