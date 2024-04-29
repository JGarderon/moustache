
use crate::engine::document::Document;
use crate::engine::document::Part;

pub fn resolve<'a>(doc: &Document) -> Result<Vec<Part>,String> {
  let mut position: usize = 0; 
  let mut max: usize = doc.stack_len(); 
  let mut result: Vec<Part> = vec!();
  loop {
    if position >= max {
      break
    }
    let p = match doc.stack_get(position) {
      Some(p) => p,
      None => break,
    };
    println!("{:?}", p);
    position += 1;
  }
  Ok(result)
}

