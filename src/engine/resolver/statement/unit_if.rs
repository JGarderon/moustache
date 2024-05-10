use core::iter::Peekable;
use core::slice::Iter;

use crate::engine::resolver::statement::Token;
use crate::engine::resolver::Part;
use crate::engine::Document;
use crate::engine::Environment;

#[derive(Debug)]
struct Condition {
  parts: Vec<ConditionPart>,
}

impl Condition {
  fn new() -> Self {
    Condition {
      parts: vec!(),
    }
  }
}

#[derive(Debug)]
enum ConditionPart {
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
  Error(String)
}

fn terminal(space_authorized: bool, token: Token, tokens: &Vec<&Token>, mut position: usize) -> ResultTokenPosition {
  loop {
    println!("terminal - {:?}", tokens.get(position));
    match tokens.get(position) {
      Some(Token::Space(_)) if space_authorized => (),
      Some(t) if t == &&token => return ResultTokenPosition::True(position),
      _ => return ResultTokenPosition::False,
    } 
    position += 1;
  }
}

fn exp_symbol_or_text<'a>(condition: &mut Condition, tokens: &Vec<&Token>, position: usize) -> ResultTokenPosition {
  println!("exp_symbol_or_text - {:?}", tokens.get(position));
  match terminal(true, Token::Symbol(0,0), tokens, position) {
    ResultTokenPosition::True(p) => {
      match tokens.get(p) {
        Some(Token::Symbol(s,e)) => {
          condition.parts.push(ConditionPart::Symbol(*s,*e));
          return ResultTokenPosition::True(p); 
        }
        Some(t) => return ResultTokenPosition::Error(format!("[exp_symbol_or_text] internal error for token '{}' found (must be 'Symbol')", t)),
        None => return ResultTokenPosition::Error("[exp_symbol_or_text] internal error : no token during transition".to_string()),
      }
    }
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match terminal(true, Token::Text(0,0), tokens, position) {
    ResultTokenPosition::True(p) => {
      match tokens.get(p) {
        Some(Token::Text(s,e)) => {
          condition.parts.push(ConditionPart::Text(*s,*e));
          return ResultTokenPosition::True(p); 
        }
        Some(t) => return ResultTokenPosition::Error(format!("[exp_symbol_or_text] internal error for token '{}' found (must be 'Symbol')", t)),
        None => return ResultTokenPosition::Error("[exp_symbol_or_text] internal error : no token during transition".to_string()),
      }
    }
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_operator<'a>(condition: &mut Condition, tokens: &Vec<&Token>, position: usize) -> ResultTokenPosition {
  println!("exp_operator - {:?}", tokens.get(position));
  match terminal(true, Token::Ampersand, tokens, position) {
    ResultTokenPosition::True(p) => match terminal(true, Token::Ampersand, tokens, p+1) {
      ResultTokenPosition::True(p) => {
        condition.parts.push(ConditionPart::AndOperator);
        return ResultTokenPosition::True(p);
      }
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match terminal(true, Token::Pipe, tokens, position) {
    ResultTokenPosition::True(p) => match terminal(true, Token::Pipe, tokens, p+1) {
      ResultTokenPosition::True(p) => {
        condition.parts.push(ConditionPart::OrOperator);
        return ResultTokenPosition::True(p);
      }
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_comparator<'a>(condition: &mut Condition, tokens: &Vec<&Token>, mut position: usize) -> ResultTokenPosition {
  println!("exp_comparator - {:?}", tokens.get(position));
  let mut next: bool = true; 
  match terminal(true, Token::Equal, tokens, position) {
    ResultTokenPosition::True(p) => {
      position = p+1;
      next = false;
    }
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  if next {
    match terminal(true, Token::Exclamation, tokens, position) {
      ResultTokenPosition::True(p) => position = p+1,
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
  }
  match terminal(false, Token::Equal, tokens, position) {
    ResultTokenPosition::True(p) => {
      if next == true {
        condition.parts.push(ConditionPart::NonEqualComparator);
      } else {
        condition.parts.push(ConditionPart::EqualComparator);      
      }
      return ResultTokenPosition::True(p);
    }
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_assertion<'a>(condition: &mut Condition, tokens: &Vec<&'a Token>, mut position: usize) -> ResultTokenPosition {
  println!("exp_assertion - {:?}", tokens.get(position));
  match exp_symbol_or_text(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p+1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match exp_comparator(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p+1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  match exp_symbol_or_text(condition, tokens, position) {
    ResultTokenPosition::True(p) => ResultTokenPosition::True(p),
    ResultTokenPosition::False => ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => ResultTokenPosition::Error(err),
  }
}

fn exp_assertions<'a>(condition: &mut Condition, tokens: &Vec<&'a Token>, mut position: usize) -> ResultTokenPosition {
  println!("exp_assertions - {:?}", tokens.get(position));
  match exp_assertion(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p+1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  loop {
    match exp_operator(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p+1,
      ResultTokenPosition::False => break,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
    match exp_group_or_assertions(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p+1,
      ResultTokenPosition::False => return ResultTokenPosition::False,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
  }
  ResultTokenPosition::True(position)
}

fn exp_group<'a>(condition: &mut Condition, tokens: &Vec<&'a Token>, mut position: usize) -> ResultTokenPosition {
  println!("exp_group");
  println!("exp_group - 1 {:?}", tokens.get(position));
  while let Some(Token::Space(_)) = tokens.get(position) {
    println!("exp_group - 1.1 {:?}", tokens.get(position));
    position += 1;
  }
  println!("exp_group - 2 {:?}", tokens.get(position));
  match terminal(true, Token::ParenthesisOpening, tokens, position) {
    ResultTokenPosition::True(p) => {
      position = p+1;
      condition.parts.push(ConditionPart::GroupOpening);
    }
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  println!("exp_group - 3 {:?}", tokens.get(position));
  match exp_general(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p+1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(format!("[exp_group] invalid group : assertions incorrect with error '{}'",err)),
  }
  println!("exp_group - 4");
  match terminal(true, Token::ParenthesisEnding, tokens, position-1) {
    ResultTokenPosition::True(p) => {
      position = p+1;
      condition.parts.push(ConditionPart::GroupEnding);
    }
    ResultTokenPosition::False => return ResultTokenPosition::Error("[exp_group] invalid group : no ending".to_string()),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(format!("[exp_group] invalid group : no endingwith error '{}'",err)),
  }
  println!("exp_group - 5");
  ResultTokenPosition::True(position)
}

fn exp_group_or_assertions<'a>(condition: &mut Condition, tokens: &Vec<&'a Token>, position: usize) -> ResultTokenPosition {
  println!("exp_group_or_assertions - 1 {:?}", tokens.get(position));
  match exp_group(condition, tokens, position) {
    ResultTokenPosition::True(p) => return ResultTokenPosition::True(p),
    ResultTokenPosition::False => (),
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  println!("exp_group_or_assertions - 2 {:?}", tokens.get(position));
  match exp_assertions(condition, tokens, position) {
    ResultTokenPosition::True(p) => return ResultTokenPosition::True(p),
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
}

fn exp_general<'a>(condition: &mut Condition, tokens: &Vec<&'a Token>, mut position: usize) -> ResultTokenPosition {
  match exp_group_or_assertions(condition, tokens, position) {
    ResultTokenPosition::True(p) => position = p+1,
    ResultTokenPosition::False => return ResultTokenPosition::False,
    ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
  }
  loop {
    match exp_operator(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p+1,
      ResultTokenPosition::False => break,
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
    match exp_group_or_assertions(condition, tokens, position) {
      ResultTokenPosition::True(p) => position = p+1,
      ResultTokenPosition::False => return ResultTokenPosition::Error(format!("group or assertions expected (at position {}",position)),
      ResultTokenPosition::Error(err) => return ResultTokenPosition::Error(err),
    }
  }
  ResultTokenPosition::True(position)
}

fn parse_tokens<'a>(tokens: Vec<&Token>) -> Result<Condition,String> {
  if tokens.len() == 0 {
    return Err("condition is blank".to_string());
  }
  let position = 0; 
  let mut condition = Condition::new(); 
  return match exp_general(&mut condition, &tokens, position) {
    ResultTokenPosition::True(_) => Ok(condition),
    ResultTokenPosition::False => Err("condition is invalid".to_string()),
    ResultTokenPosition::Error(err) => Err(err),
  }
}

// fn resolve_exp_suppl<'a>(state: bool, env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<Option<bool>,String> {
//   let operator = match condition.parts.get(position) {
//     Some(ConditionPart::OrOperator) => true,
//     Some(ConditionPart::AndOperator) => false,
//     Some(p) => return Err(format!("[resolve_exp] invalid part '{}' in condition (op)", p)), 
//     None => return Ok(None),
//   };
//   if state && operator {
//     return Ok(Some(true));
//   }
//   position += 1;
//   match resolve_exp(env, source, condition, position) {
//     Ok(r) => if operator {
//       Ok(Some(state || r))
//     } else {
//       Ok(Some(state && r))
//     }
//     Err(err) => Err(err), 
//   }
// } 

fn resolve_exp_assertion<'a>(env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<bool,String> {
  let a1 = match condition.parts.get(position) {
    Some(ConditionPart::Symbol(s,e)) => {
      let key = source[*s..*e].to_string(); 
      match env.get(&key) {
        Some(v) => v.to_string(),
        None => return Err(format!("[resolve_exp_assertion] env key '{}' unknow in condition (p1)", key))
      }
    }
    Some(ConditionPart::Text(s,e)) => source[*s..*e].to_string(),
    Some(c) => return Err(format!("[resolve_exp_assertion] invalid part '{:?}' in condition (p1)", c)), 
    None => return Err("[resolve_exp_assertion] condition object can't be null (p1)".to_string()),
  };
  position += 1;
  let op_equal = match condition.parts.get(position) {
    Some(ConditionPart::EqualComparator) => true,
    Some(ConditionPart::NonEqualComparator) => false,
    Some(c) => return Err(format!("[resolve_exp_assertion] invalid part '{:?}' in condition (p1)", c)), 
    None => return Err("[resolve_exp_assertion] condition object can't be null (p1)".to_string()),
  };
  position += 1;
  let a2 = match condition.parts.get(position) {
    Some(ConditionPart::Symbol(s,e)) => {
      let key = source[*s..*e].to_string(); 
      match env.get(&key) {
        Some(v) => v.to_string(),
        None => return Err(format!("[resolve_exp_assertion] env key '{}' unknow in condition (p1)", key))
      }
    }
    Some(ConditionPart::Text(s,e)) => source[*s..*e].to_string(),
    Some(c) => return Err(format!("[resolve_exp_assertion] invalid part '{:?}' in condition (p1)", c)), 
    None => return Err("[resolve_exp_assertion] condition object can't be null (p1)".to_string()),
  };
  if op_equal {
    Ok(a1 == a2)
  } else {
    Ok(a1 != a2)
  }
}

fn resolve_group_or_exp_rest<'a>(state: bool, env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<Option<(bool, usize)>,String> {
  let op_or = match condition.parts.get(position) {
    Some(ConditionPart::OrOperator) => true, 
    Some(ConditionPart::AndOperator) => false, 
    _ => return Ok(None)
  };
  position += 1;
  let r = match resolve_exp_group_or_assertions(state, env, source, condition, position) {
    Ok((r,p)) => {
      position = p;
      r
    }
    Err(err) => return Err(err)
  };
  if op_or {
    Ok(Some((state || r, position)))
  } else {
    Ok(Some((state && r, position)))    
  }
}

fn resolve_exp_assertion_and_rest<'a>(env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<(bool, usize),String> {
  let mut state = match resolve_exp_assertion(env, source, condition, position) {
    Ok(r) => r,
    Err(err) => return Err(err),
  };
  position += 3;
  loop { 
    state = match resolve_group_or_exp_rest(state, env, source, condition, position) {
      Ok(Some((r, p))) => {
        position = p;
        r
      }
      Ok(None) => break,
      Err(err) => return Err(err), 
    };
  }
  Ok((state, position))
}

fn resolve_exp_group<'a>(mut state: bool, env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<(bool, usize),String> {
  state = match resolve_exp_group_or_assertions(state, env, source, condition, position) {
    Ok((r, p)) => {
      position = p;
      r 
    }
    Err(err) => return Err(err),
  };
  if let Some(ConditionPart::GroupEnding) = condition.parts.get(position) {
    Ok((state, position))
  } else {
    Err("[resolve_exp_group] group ending not found".to_string()) 
  }
}

fn resolve_exp_group_or_assertions<'a>(mut state: bool, env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<(bool, usize),String> {
  match condition.parts.get(position) {
    Some(ConditionPart::GroupOpening) => match resolve_exp_group(state, env, source, condition, position+1) {
      Ok((r, p)) => {
        position = p;
        state = r;
      }
      Err(err) => return Err(err),
    }
    _ => match resolve_exp_assertion_and_rest(env, source, condition, position) {
      Ok((r, p)) => {
        position = p;
        state = r;
      }
      Err(err) => return Err(err),
    }
  }
  Ok((state, position))
}

fn resolve_exp<'a>(env: &Environment, source: &'a str, condition: &Condition, mut position: usize) -> Result<bool,String> {
    println!("{:?}", resolve_exp_group_or_assertions(true, env, source, condition, position));

  // let mut r1: Option<bool> = None; 
  // let mut r: bool = false; 
  // let mut comparator: bool = bool;
  // let mut operator: bool = bool;
  // let p1 = match condition.parts.get(position) {
  //   Some(ConditionPart::Symbol(s,e)) => {
  //     let key = source[*s..*e].to_string(); 
  //     match env.get(&key) {
  //       Some(v) => v.to_string(),
  //       None => return Err(format!("[resolve_exp] env key '{}' unknow in condition (p1)", key))
  //     }
  //   }
  //   Some(ConditionPart::Text(s,e)) => source[*s..*e].to_string(),
  //   Some(ConditionPart::GroupOpening) => match resolve_exp(doc, env, condition, position+1) {
  //     Ok(v) => r1 = Some(v),
  //     Err(err) => return Err(err),
  //   }
  //   Some(c) => return Err(format!("[resolve_exp] invalid part '{}' in condition (p1)", c)), 
  //   None => return Err("[resolve_exp] condition object can't be null (p1)".to_string()),
  // };
  // if r1 == None {
  //   position += 1;
  //   comparator = match condition.parts.get(position) {
  //     Some(ConditionPart::EqualComparator) => true,
  //     Some(ConditionPart::NonEqualComparator) => false,
  //     Some(p) => return Err(format!("[resolve_exp] invalid part '{}' in condition (op)", p)), 
  //     None => return Err("[resolve_exp] condition object can't be null (op)".to_string()),
  //   };
  //   position += 1;
  //   let p2 = match condition.parts.get(position) {
  //     Some(ConditionPart::Symbol(s,e)) => {
  //       let key = source[*s..*e].to_string(); 
  //       match env.get(&key) {
  //         Some(v) => v.to_string(),
  //         None => return Err(format!("[resolve_exp] env key '{}' unknow in condition (p2)", key))
  //       }
  //     }
  //     Some(ConditionPart::Text(s,e)) => source[*s..*e].to_string(),
  //     Some(ConditionPart::GroupOpening) => match resolve_exp(doc, env, condition, position+1) {
  //       Ok(v) => r1 = Some(v),
  //       Err(err) => return Err(err),
  //     }
  //     None => return Err("[resolve_exp] condition object can't be null (p2)".to_string()),
  //     Some(c) => return Err(format!("[resolve_exp] invalid part '{}' in condition (p2)", c)), 
  //   };
  // } else {
  //   position += 1;
  //   operator = match condition.parts.get(position) {
  //     Some(ConditionPart::OrOperator) => true,
  //     Some(ConditionPart::AndOperator) => false,
  //     Some(p) => return Err(format!("[resolve_exp] invalid part '{}' in condition (op)", p)), 
  //     None => return Err("[resolve_exp] condition object can't be null (op)".to_string()),
  //   };
  //   if let Some(true) == r1 && operator {
  //     return Ok(true)
  //   }
  //   r = r1.unwrap();
  //   position += 1;
  //   match resolve_exp_suppl(r, doc, env, condition, position) {
  //     Ok(v) => r = v,
  //     Err(err) => return Err(err),
  //   }
  // }
  

  // if comparator {
  //   Ok(p1 == p2)
  // } else {
  //   Ok(p1 != p2)
  // }
  Ok(false)
}

fn resolve_condition<'a>(env: &Environment, source: &'a str, condition: Condition) -> Result<bool,String> {
  resolve_exp(env, source, &condition, 0)
}

pub fn resolve_unit<'a>(
  doc: &'a Document,
  doc_position: usize,
  env: &mut Environment,
  source: &'a str,
  iter_tokens: &mut Peekable<Iter<'_, Token>>,
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
  let tokens = iter_tokens.collect::<Vec<&Token>>();
  let condition = match parse_tokens(tokens) {
    Ok(c) => c, 
    Err(err) => return Err(format!("error during conditional tokens parsing : {}", err))
  };
  println!("if_block condition = {:?}", condition);
  let result = match resolve_condition(env, source, condition) {
    Ok(r) => r, 
    Err(err) => return Err(format!("error during conditional tokens resolving : {}", err))
  };
  println!("if_block result = {:?}", result);
  if result {
    Ok(block_ending_position)
  } else {
    Ok(block_ending_position)
  }
}
