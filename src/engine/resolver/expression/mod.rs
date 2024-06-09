use crate::create_internal_error;
use crate::engine::document::Part;
use crate::engine::parser;
use crate::engine::resolver::add_string_to_another;
use crate::engine::Document;
use crate::engine::Environment;
use crate::utils::error::InternalError;

pub fn resolve_expression<'a>(
  _doc: &'a Document,
  expr: &'a str,
  env: &mut Environment,
) -> Result<Part, InternalError> {
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
      &parser::Token::Text(s, e) if is_begining == false => {
        return Err(
          create_internal_error!(
            format!(
              "Invalid position's text in expression: the operator '+' is likely missing (found '{}' at {} ~> {})",
              &source[s..e],
              s,
              e
            )
          )
        )
      }
      &parser::Token::Symbol(s, e) if is_begining == false => {
        return Err(
          create_internal_error!(
            format!(
              "Invalid position's symbol in expression: the operator '+' is likely missing (found '{}' at {} ~> {})",
              &source[s..e],
              s,
              e
            )
          )
        )
      }
      parser::Token::Plus if is_begining => {
        return Err(
          create_internal_error!(
            "The operator '+' is not allowed at the beginning of the expression or more than once in a row"
          )
        )
      }
      parser::Token::Text(s, e) if is_begining => {
        add_string_to_another(&mut source[*s..*e].to_string(), &mut output);
        is_begining = false;
      }
      parser::Token::Symbol(s, e) if is_begining => {
        let symbol = source[*s..*e].to_string();
        match env.get(&symbol) {
          Ok(Some(v)) => output.push_str(v),
          Ok(None) => match env.get_real_key(&symbol) {
            Some((true, real_key)) => return Err(create_internal_error!(format!(
              "Undefined indirection variable '{}' in environment (original : '{}')",
              real_key,
              symbol
            ))),
            Some((false, real_key)) => return Err(create_internal_error!(format!(
              "Undefined variable '{}' in environment (no indirection)",
              real_key
            ))),
            None => return Err(create_internal_error!(format!(
              "Undefined indirection variable '{}' in environment (original value)",
              symbol
            )))
          }
          Err(err) => return Err(create_internal_error!(
            "Error during getting variable",
            err
          )),
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
                Ok(Some(v)) => add_string_to_another(
                  &mut v[..].to_string(), // pas terrible...
                  &mut output,
                ),
                Ok(None) => {
                  return Err(create_internal_error!(format!(
                    "Undefined variable '{}' in environment",
                    symbol
                  )))
                },
                Err(err) => return Err(create_internal_error!(err)),
              }
              is_begining = false;
              break;
            }
            Some(parser::Token::Space(_)) => continue,
            Some(t) => {
              return Err(create_internal_error!(format!(
                "Token {} not authorized in second part of expression (after '+'; must be text or symbol)",
                t
              )))
            }
            None => {
              return Err(create_internal_error!(
                "No token found for second part of expression (after '+')"
              ))
            }
          };
        }
      }
      parser::Token::Space(_) => (),
      t => {
        return Err(create_internal_error!(format!(
          "Token {} not authorized in second part of expression (at beginning; must be text or symbol)",
          t
        )))
      }
    }
  }
  Ok(Part::GeneratedText(output))
}
