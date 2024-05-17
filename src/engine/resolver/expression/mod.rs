use crate::engine::document::Part;
use crate::engine::parser;
use crate::engine::resolver::add_string_to_another;
use crate::engine::Document;
use crate::engine::Environment;

pub fn resolve_expression<'a>(
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
