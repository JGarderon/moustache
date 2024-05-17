use std::cmp::PartialEq;
use std::fmt;

// #[derive(Debug)]
// #[derive(PartialEq)]
pub enum Token {
  Symbol(usize, usize),
  Text(usize, usize),
  Space(TokenSpace),
  ParenthesisOpening,
  ParenthesisEnding,
  Plus,
  Minus,
  Equal,
  Divide,
  Multiply,
  Escape,
  Pipe,
  Ampersand,
  Exclamation,
}

impl PartialEq<Token> for Token {
  fn eq(&self, other: &Token) -> bool {
    match (self, other) {
      (Token::Symbol(_, _), Token::Symbol(_, _)) => true,
      (Token::Text(_, _), Token::Text(_, _)) => true,
      (Token::Space(_), Token::Space(_)) => true,
      (Token::ParenthesisOpening, Token::ParenthesisOpening) => true,
      (Token::ParenthesisEnding, Token::ParenthesisEnding) => true,
      (Token::Plus, Token::Plus) => true,
      (Token::Minus, Token::Minus) => true,
      (Token::Equal, Token::Equal) => true,
      (Token::Divide, Token::Divide) => true,
      (Token::Multiply, Token::Multiply) => true,
      (Token::Escape, Token::Escape) => true,
      (Token::Pipe, Token::Pipe) => true,
      (Token::Ampersand, Token::Ampersand) => true,
      (Token::Exclamation, Token::Exclamation) => true,
      _ => false,
    }
  }
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::Symbol(s, e) => write!(f, "Token::Symbol({},{})", s, e),
      Token::Text(s, e) => write!(f, "Token::Text({},{})", s, e),
      Token::Space(s) => write!(f, "Token::Space({:?})", s),
      Token::ParenthesisOpening => write!(f, "Token::ParenthesisOpening"),
      Token::ParenthesisEnding => write!(f, "Token::ParenthesisEnding"),
      Token::Plus => write!(f, "Token::Plus"),
      Token::Minus => write!(f, "Token::Minus"),
      Token::Equal => write!(f, "Token::Equal"),
      Token::Divide => write!(f, "Token::Divide"),
      Token::Multiply => write!(f, "Token::Multiply"),
      Token::Escape => write!(f, "Token::Escape"),
      Token::Pipe => write!(f, "Token::Pipe"),
      Token::Ampersand => write!(f, "Token::Ampersand"),
      Token::Exclamation => write!(f, "Token::Exclamation"),
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::Symbol(s, e) => write!(f, "Token::Symbol({},{})", s, e),
      Token::Text(s, e) => write!(f, "Token::Text({},{})", s, e),
      Token::Space(s) => write!(f, "Token::Space({:?})", s),
      Token::ParenthesisOpening => write!(f, "Token::ParenthesisOpening"),
      Token::ParenthesisEnding => write!(f, "Token::ParenthesisEnding"),
      Token::Plus => write!(f, "Token::Plus"),
      Token::Minus => write!(f, "Token::Minus"),
      Token::Equal => write!(f, "Token::Equal"),
      Token::Divide => write!(f, "Token::Divide"),
      Token::Multiply => write!(f, "Token::Multiply"),
      Token::Escape => write!(f, "Token::Escape"),
      Token::Pipe => write!(f, "Token::Pipe"),
      Token::Ampersand => write!(f, "Token::Ampersand"),
      Token::Exclamation => write!(f, "Token::Exclamation"),
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum TokenSpace {
  Space,
  Tabulation,
  LineFeed,
  CarriageReturn,
}

pub fn parse<'a>(source: &'a str) -> Result<Vec<Token>, String> {
  let mut stack: Vec<Token> = vec![];
  let mut is_text: bool = false;
  let mut portion_start: usize = 0;
  let mut is_escaping: bool = false;
  for (i, c) in source.char_indices() {
    // println!(
    //   "(boucle) i = {:?} ; c = {:?} ; is_text = {:?} ; portion_start = {:?} ; is_escaping = {:?}",
    //   i, c, is_text, portion_start, is_escaping
    // );
    if is_text == true && is_escaping == true {
      is_escaping = false;
      continue;
    } else if is_text == false && is_escaping == true {
      return Err(format!("bad escape at {}", i - 1));
    }
    match c {
      ' ' | '\t' | '\n' | '\r' if is_text == false => {
        if i > 0 && portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        let space_type = match c {
          ' ' => TokenSpace::Space,
          '\t' => TokenSpace::Tabulation,
          '\n' => TokenSpace::LineFeed,
          '\r' => TokenSpace::CarriageReturn,
          _ => panic!(
            "{}",
            format!(
              "Fatal error in program : token space not valid in engine's parser (found {:?})",
              c
            )
          ),
        };
        stack.push(Token::Space(space_type));
        portion_start = i + 1;
      }
      '(' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::ParenthesisOpening);
        portion_start = i + 1;
      }
      ')' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::ParenthesisEnding);
        portion_start = i + 1;
      }
      '+' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Plus);
        portion_start = i + 1;
      }
      '-' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Minus);
        portion_start = i + 1;
      }
      '=' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Equal);
        portion_start = i + 1;
      }
      '/' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Divide);
        portion_start = i + 1;
      }
      '*' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Multiply);
        portion_start = i + 1;
      }
      '|' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Pipe);
        portion_start = i + 1;
      }
      '&' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Ampersand);
        portion_start = i + 1;
      }
      '!' if is_text == false => {
        if portion_start < i - 1 {
          stack.push(Token::Symbol(portion_start, i));
        }
        stack.push(Token::Exclamation);
        portion_start = i + 1;
      }
      '\\' => {
        if is_escaping {
          is_escaping = false;
          stack.push(Token::Escape);
        } else {
          is_escaping = true;
        }
      }
      '"' => {
        if is_text {
          is_text = false;
          stack.push(Token::Text(portion_start, i));
          portion_start = i + 1;
        } else {
          is_text = true;
          portion_start = i + 1;
        }
      }
      _ => (),
    }
  }
  if is_text {
    return Err(format!(
      "text opened at position {} and not closed",
      portion_start
    ));
  }
  let max: usize = source.len();
  if portion_start < max {
    stack.push(Token::Symbol(portion_start, max));
  }
  Ok(stack)
}
