pub mod expression;
pub mod statement;

use crate::add_step_internal_error;
use crate::engine::document::Part;
use crate::engine::resolver::expression::resolve_expression;
use crate::engine::resolver::statement::resolve_statement;
use crate::engine::resolver::statement::Resolved;
use crate::engine::Document;
use crate::engine::Environment;
use crate::utils::error::InternalError;

pub fn add_string_to_another(s1: &mut String, s2: &mut String) {
  s1.retain(|c| !r#"\"#.contains(c));
  s2.push_str(s1);
}

pub fn resolve<'a>(doc: &'a Document, env: &mut Environment) -> Result<Resolved, InternalError> {
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
          Err(mut err) => return Err(
            add_step_internal_error!(
              err,
              "Error in expression",
              format!("target expression (here with trim !) = '\x1b[3m{}\x1b[0m'", &doc.source[s+2..e-2].trim())
            )
          ),
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
