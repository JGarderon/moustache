use core::iter::Peekable;
use core::slice::Iter;

use crate::add_step_internal_error;
use crate::create_internal_error;
use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::engine::Environment;
use crate::utils::error::InternalError;

#[derive(Debug)]
struct Condition {
  tmp: Vec<ConditionPart>,
  parts: Vec<ConditionPart>,
}

impl Condition {
  fn new() -> Self {
    Condition {
      tmp: vec![],
      parts: vec![],
    }
  }
}

#[derive(Debug)]
enum ConditionPart {
  // Assertion :
  //  - is equal (bool)
  //  - is symbol (bool), start (usize), end (usize) -> left
  //  - is symbol (bool), start (usize), end (usize) -> right
  Assertion(bool, bool, usize, usize, bool, usize, usize),
  Text(usize, usize),
  Symbol(usize, usize),
  EqualComparator,
  NonEqualComparator,
  OrOperator,
  AndOperator,
  GroupOpening,
  GroupEnding,
}

#[derive(Debug)]
enum ResultTokenPosition {
  True(usize),
  False,
  Error(String),
}

fn terminal(
  space_authorized: bool,
  token: Token,
  tokens: &Vec<&Token>,
  mut position: usize,
) -> ResultTokenPosition {
  loop {
    match tokens.get(position) {
      Some(Token::Space(_)) if space_authorized => (),
      Some(t) if t == &&token => return ResultTokenPosition::True(position),
      _ => return ResultTokenPosition::False,
    }
    position += 1;
  }
}

fn exp_symbol_or_text<'a>(
  condition: &mut Condition,
  tokens: &Vec<&Token>,
  mut position: usize,
) -> ResultTokenPosition {
  match terminal(true, Token::Symbol(0, 0), tokens, position) {
    ResultTokenPosition::True(p) => match tokens.get(p) {
      Some(Token::Symbol(s, e)) => {
        condition.tmp.push(ConditionPart::Symbol(*s, *e));
        return ResultTokenPosition::True(p);
      }
      Some(t) => {
        return ResultTokenPosition::Error(format!(
          "[exp_symbol_or_text] internal error for token '{}' found (must be 'Symbol')",
          t
        ))
      }
      None => {
        return ResultTokenPosition::Error(
          "[exp_symbol_or_text] internal error : no token during transition".to_string(),
        )
      }
    },
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match terminal(true, Token::Text(0, 0), tokens, position) {
    ResultTokenPosition::True(p) => match tokens.get(p) {
      Some(Token::Text(s, e)) => {
        condition.tmp.push(ConditionPart::Text(*s, *e));
        return ResultTokenPosition::True(p);
      }
      Some(t) => {
        return ResultTokenPosition::Error(format!(
          "[exp_symbol_or_text] internal error for token '{}' found (must be 'Symbol')",
          t
        ))
      }
      None => {
        return ResultTokenPosition::Error(
          "[exp_symbol_or_text] internal error : no token during transition".to_string(),
        )
      }
    },
    ResultTokenPosition::False => loop {
      match tokens.get(position) {
        Some(Token::Space(_)) => (),
        Some(t) => {
          return ResultTokenPosition::Error(format!(
            "Invalid token found : {} (must be a symbol or text)",
            t
          ))
        }
        None => return ResultTokenPosition::False,
      }
      position += 1;
    },
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_operator<'a>(
  condition: &mut Condition,
  tokens: &Vec<&Token>,
  position: usize,
) -> ResultTokenPosition {
  match terminal(true, Token::Ampersand, tokens, position) {
    ResultTokenPosition::True(p) => match terminal(true, Token::Ampersand, tokens, p + 1) {
      ResultTokenPosition::True(p) => {
        condition.parts.push(ConditionPart::AndOperator);
        return ResultTokenPosition::True(p);
      }
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    },
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match terminal(true, Token::Pipe, tokens, position) {
    ResultTokenPosition::True(p) => match terminal(true, Token::Pipe, tokens, p + 1) {
      ResultTokenPosition::True(p) => {
        condition.parts.push(ConditionPart::OrOperator);
        return ResultTokenPosition::True(p);
      }
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    },
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_comparator<'a>(
  condition: &mut Condition,
  tokens: &Vec<&Token>,
  mut position: usize,
) -> ResultTokenPosition {
  let mut next: bool = true;
  match terminal(true, Token::Equal, tokens, position) {
    ResultTokenPosition::True(p) => {
      position = p + 1;
      next = false;
    }
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  if next {
    match terminal(true, Token::Exclamation, tokens, position) {
      ResultTokenPosition::True(p) => position = p + 1,
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
  }
  match terminal(false, Token::Equal, tokens, position) {
    ResultTokenPosition::True(p) => {
      if next == true {
        condition.tmp.push(ConditionPart::NonEqualComparator);
      } else {
        condition.tmp.push(ConditionPart::EqualComparator);
      }
      return ResultTokenPosition::True(p);
    }
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_assertion<'a>(
  condition: &mut Condition,
  tokens: &Vec<&'a Token>,
  mut position: usize,
) -> ResultTokenPosition {
  match exp_symbol_or_text(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p + 1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match exp_comparator(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p + 1,
    ResultTokenPosition::False => {
      return ResultTokenPosition::Error(
        "comparator not found after first symbol or text in assertion".to_string(),
      )
    }
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  let p = match exp_symbol_or_text(condition, tokens, position) {
    ResultTokenPosition::True(p) => p,
    ResultTokenPosition::False => {
      return ResultTokenPosition::Error(
        "symbol or text not found after comparator in assertion".to_string(),
      )
    }
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  };
  let (second_is_symbol, second_start, second_end) = match condition.tmp.pop() {
    Some(ConditionPart::Text(s,e)) => (false, s, e),
    Some(ConditionPart::Symbol(s,e)) => (true, s, e),
    o => return ResultTokenPosition::Error(format!("internal logic error in tmp condition (found '{:?}', must be ConditionPart::[Text or Symbol])", o)),
  };
  let comparator = match condition.tmp.pop() {
    Some(ConditionPart::EqualComparator) => true,
    Some(ConditionPart::NonEqualComparator) => false,
    o => return ResultTokenPosition::Error(format!("internal logic error in tmp condition (found '{:?}', must be ConditionPart::[EqualComparator or NonEqualComparator])", o)),
  };
  let (first_is_symbol, first_start, first_end) = match condition.tmp.pop() {
    Some(ConditionPart::Text(s,e)) => (false, s, e),
    Some(ConditionPart::Symbol(s,e)) => (true, s, e),
    o => return ResultTokenPosition::Error(format!("internal logic error in tmp condition (found '{:?}', must be ConditionPart::[Text or Symbol])", o)),
  };
  condition.parts.push(ConditionPart::Assertion(
    comparator,
    first_is_symbol,
    first_start,
    first_end,
    second_is_symbol,
    second_start,
    second_end,
  ));
  ResultTokenPosition::True(p)
}

fn exp_assertions<'a>(
  condition: &mut Condition,
  tokens: &Vec<&'a Token>,
  mut position: usize,
) -> ResultTokenPosition {
  match exp_assertion(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p + 1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  loop {
    match exp_operator(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p + 1,
      ResultTokenPosition::False => break,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
    match exp_group_or_assertions(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p + 1,
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
  }
  ResultTokenPosition::True(position - 1)
}

fn exp_group<'a>(
  condition: &mut Condition,
  tokens: &Vec<&'a Token>,
  mut position: usize,
) -> ResultTokenPosition {
  while let Some(Token::Space(_)) = tokens.get(position) {
    position += 1;
  }
  match terminal(true, Token::ParenthesisOpening, tokens, position) {
    ResultTokenPosition::True(p) => {
      position = p + 1;
      condition.parts.push(ConditionPart::GroupOpening);
    }
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match exp_general(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p + 1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => {
      return ResultTokenPosition::Error(format!(
        "[exp_group] invalid group : assertions incorrect with error '{}'",
        err
      ))
    }
  }
  match terminal(true, Token::ParenthesisEnding, tokens, position - 1) {
    ResultTokenPosition::True(p) => {
      position = p + 1;
      condition.parts.push(ConditionPart::GroupEnding);
    }
    ResultTokenPosition::False => {
      return ResultTokenPosition::Error("[exp_group] invalid group : no ending".to_string())
    }
    ResultTokenPosition::Error(err) => {
      return ResultTokenPosition::Error(format!(
        "[exp_group] invalid group : no endingwith error '{}'",
        err
      ))
    }
  }
  ResultTokenPosition::True(position)
}

fn exp_group_or_assertions<'a>(
  condition: &mut Condition,
  tokens: &Vec<&'a Token>,
  position: usize,
) -> ResultTokenPosition {
  match exp_group(condition, tokens, position) {
    ResultTokenPosition::True(p) => return ResultTokenPosition::True(p),
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match exp_assertions(condition, tokens, position) {
    ResultTokenPosition::True(p) => return ResultTokenPosition::True(p),
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_general<'a>(
  condition: &mut Condition,
  tokens: &Vec<&'a Token>,
  mut position: usize,
) -> ResultTokenPosition {
  match exp_group_or_assertions(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p + 1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  loop {
    match exp_operator(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p + 1,
      ResultTokenPosition::False => break,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
    match exp_group_or_assertions(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p + 1,
      ResultTokenPosition::False => {
        return ResultTokenPosition::Error(format!(
          "group or assertions expected (at position {}",
          position
        ))
      }
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
  }
  ResultTokenPosition::True(position)
}

fn verify_tokens<'a>(tokens: Vec<&Token>) -> Result<Condition, InternalError> {
  if tokens.len() == 0 {
    return Err(create_internal_error!(
      "The condition is empty (no assertion provided)"
    ));
  }
  let position = 0;
  let mut condition = Condition::new();
  return match exp_general(&mut condition, &tokens, position) {
    ResultTokenPosition::True(_) => Ok(condition),
    ResultTokenPosition::False => Err(create_internal_error!("The condition is invalid")),
    ResultTokenPosition::Error(err) => Err(create_internal_error!(err)),
  };
}

fn resolve_exp<'a>(
  env: &Environment,
  source: &'a str,
  condition: &Condition,
  mut position: usize,
) -> Result<(bool, usize), InternalError> {
  let mut result = false;
  let mut operator_and: Option<bool> = None;
  let mut beginning = true;
  loop {
    match condition.parts.get(position) {
      Some(ConditionPart::Text(_, _)) | Some(ConditionPart::Symbol(_, _)) => {
        return Err(create_internal_error!(
          "The expression resolver encountered invalid logic in the condition",
          "Expecting : ConditionPart::[Text or Symbol]"
        ))
      }
      Some(ConditionPart::EqualComparator) | Some(ConditionPart::NonEqualComparator) => {
        return Err(create_internal_error!(
          "Invalid logic in expression resolver in condition",
          "Found ConditionPart::[EqualComparator or NonEqualComparator])"
        ))
      }
      Some(ConditionPart::AndOperator) => {
        if operator_and == None {
          operator_and = Some(true);
        } else {
          return Err(create_internal_error!(
            "Invalid logic in expression resolver in condition",
            "Two consecutive operators found (second : 'and')"
          ));
        }
      }
      Some(ConditionPart::OrOperator) => {
        if operator_and == None {
          operator_and = Some(false);
        } else {
          return Err(create_internal_error!(
            "Invalid logic in expression resolver in condition",
            "Two consecutive operators found (second : 'or')"
          ));
        }
      }
      Some(ConditionPart::Assertion(_, _, _, _, _, _, _))
        if operator_and == None && beginning == false =>
      {
        return Err(create_internal_error!(
          "Invalid logic in expression resolver in condition",
          "No operator found between two assertions"
        ))
      }
      Some(ConditionPart::Assertion(
        is_equal,
        first_is_symbol,
        first_start,
        first_end,
        second_is_symbol,
        second_start,
        second_end,
      )) => {
        let first: String;
        if *first_is_symbol {
          let key: String = source[*first_start..*first_end].to_string();
          match env.get(&key) {
            Some(v) => first = v.to_string(),
            None => {
              return Err(create_internal_error!(format!(
                "Undefined variable '{}' in condition",
                key
              )))
            }
          }
        } else {
          first = source[*first_start..*first_end].to_string();
        }
        let second: String;
        if *second_is_symbol {
          let key: String = source[*second_start..*second_end].to_string();
          match env.get(&key) {
            Some(v) => second = v.to_string(),
            None => {
              return Err(create_internal_error!(format!(
                "Undefined variable '{}' in condition",
                key
              )))
            }
          }
        } else {
          second = source[*second_start..*second_end].to_string();
        }
        let r: bool;
        if *is_equal {
          r = first == second;
        } else {
          r = first != second;
        }
        if operator_and == Some(true) {
          result &= r;
          operator_and = None;
        } else if operator_and == Some(false) {
          result |= r;
          operator_and = None;
        } else {
          result = r;
        }
      }
      Some(ConditionPart::GroupOpening) => {
        match resolve_exp(env, source, condition, position + 1) {
          Ok((r, p)) => {
            position = p;
            if operator_and == Some(true) {
              result &= r;
            } else {
              return Ok((result, position + 1));
            }
          }
          Err(err) => return Err(err),
        }
      }
      Some(ConditionPart::GroupEnding) | None => return Ok((result, position + 1)),
    }
    position += 1;
    beginning = false;
  }
}

fn resolve_condition<'a>(
  env: &Environment,
  source: &'a str,
  condition: Condition,
) -> Result<bool, InternalError> {
  match resolve_exp(env, source, &condition, 0) {
    Ok((r, _)) => Ok(r),
    Err(err) => Err(err),
  }
}

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<(Vec<Part>, usize), InternalError> {
  let mut iter_parts = doc.stack.iter().skip(doc_position).enumerate();
  let mut block_ending_position: usize;
  loop {
    let part = match iter_parts.next() {
      Some((position, part)) => {
        block_ending_position = position;
        part
      }
      None => return Err(
        create_internal_error!(
          "Unfinished block",
          "must be = '\x1b[3mif [symbol or text] [ '==' | '!=' ] [symbol or text] ( [ '&' | '|' ] ... )\x1b[0m'",
          format!("found statement = '\x1b[3m{}\x1b[0m'", source.trim())
        )
      ),
    };
    match part {
      &Part::Statement(s, e) if doc.source[s + 2..e - 2].trim() == "endif" => break,
      _ => (),
    }
  }
  let tokens = iter_tokens.collect::<Vec<&Token>>();
  let condition = match verify_tokens(tokens) {
    Ok(c) => c,
    Err(mut err) => {
      return Err(add_step_internal_error!(
        err,
        "Error during conditional tokens parsing ('verify tokens' step)",
        "must be = '\x1b[3mif [symbol or text] ['==' or '!='] [symbol or text] ( ['&&' or '||'] ... )\x1b[0m'",
        format!("found statement = '\x1b[3m{}\x1b[0m'", source.trim())
      ))
    }
  };
  let result = match resolve_condition(env, source, condition) {
    Ok(r) => r,
    Err(mut err) => {
      return Err(add_step_internal_error!(
        err,
        "Error during conditional tokens resolving ('resolve condition' step)",
        "must be = '\x1b[3mif [symbol or text] [ '==' | '!=' ] [symbol or text] ( [ '&' | '|' ] ... )\x1b[0m'",
        format!("found statement = '\x1b[3m{}\x1b[0m'", source.trim())
      ))
    }
  };
  if result {
    Ok((
      doc.stack[doc_position + 1..doc_position + block_ending_position - 1].to_vec(),
      block_ending_position,
    ))
  } else {
    Ok((vec![], block_ending_position))
  }
}
