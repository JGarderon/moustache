
use crate::engine::document::Document;
use crate::engine::document::Part;
use crate::engine::environment::Environment;

fn resolve_expression<'a>(_doc: &'a Document, expr: &'a str, env: &mut Environment) -> Result<Part,String> {
  let k: String = expr[2..expr.len()-2].trim().to_string(); 
  match env.get(&k) {
    Some(v) => Ok(Part::GeneratedText(v.to_string())),
    None => return Err(format!("key '{}' not found in env (expr)", k))
  }
}

pub fn resolve<'a>(doc: &'a Document, env: &mut Environment) -> Result<Vec<Part>,String> {
  let mut position: usize = 0; 
  let max: usize = doc.stack_len(); 
  let mut result: Vec<Part> = vec!();
  loop {
    if position >= max {
      break
    }
    match doc.stack_get(position) {
      Some(&Part::StaticText(s,e)) => result.push(Part::StaticText(s,e)),
      Some(&Part::Expression(s,e)) => match resolve_expression(doc, &doc.source[s..e], env) {
        Ok(p) => result.push(p),
        Err(err) => return Err(err)
      },
      Some(Part::GeneratedText(_)) | Some(Part::Statement(_, _)) | Some(Part::Comment(_, _)) => (),
      None => break
    }
    position += 1;
  }
  Ok(result)
}

