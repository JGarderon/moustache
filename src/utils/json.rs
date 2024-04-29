
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
  Bool(bool),
  Number(f64),
  Text(String),
  Array(Vec<Value>),
  Dict(HashMap<String,Value>),
  Null,
  None
}

#[derive(Debug)]
pub enum Token {
  Expression(usize, usize),
  Text(usize, usize),
  Array_Start(usize),
  Array_Stop(usize),
  Dict_Start(usize),
  Dict_Stop(usize),
  Separator_List(usize),
  Separator_Pair(usize),
  Blank(usize) 
}

pub fn tokenize<'a>(source: &'a str) -> Result<Vec<Token>,String> {
  let mut tokens = vec!();
  let mut text = false;
  let mut text_start = 0; 
  let mut expression = false; 
  let mut expression_start = 0; 
  let mut escape = false;
  for (i, c) in source.chars().enumerate() {
    match c {
      ' ' | '\t' | '\n' | '\r' if text == false => {
        if expression {
          expression = true; 
          tokens.push(Token::Expression(expression_start, i));
        } else {
          tokens.push(Token::Blank(i));
        }
      }
      '{' if text == false => {
        if expression {
          expression = false; 
          tokens.push(Token::Expression(expression_start, i));          
        } 
        tokens.push(Token::Dict_Start(i));
      }
      '}' if text == false => {
        if expression {
          expression = false; 
          tokens.push(Token::Expression(expression_start, i));          
        } 
        tokens.push(Token::Dict_Stop(i));
      }
      '(' if text == false => {
        if expression {
          expression = false; 
          tokens.push(Token::Expression(expression_start, i));          
        } 
        tokens.push(Token::Array_Start(i));
      }
      ')' if text == false => {
        if expression {
          expression = false; 
          tokens.push(Token::Expression(expression_start, i));          
        } 
        tokens.push(Token::Array_Stop(i));
      }
      ',' if text == false => {
        if expression {
          expression = false; 
          tokens.push(Token::Expression(expression_start, i));          
        } 
        tokens.push(Token::Separator_List(i));
      }
      ':' if text == false => {
        if expression {
          expression = false; 
          tokens.push(Token::Expression(expression_start, i));          
        } 
        tokens.push(Token::Separator_Pair(i));
      }
      '"' => if text {
        if escape == false {
          text = false; 
          tokens.push(Token::Text(text_start+1, i));            
        } 
      } else {
        text = true;
        text_start = i; 
      }
      '\\' if text == true => escape = true,
      _ if text == false => {
        if expression == false {
          expression = true;
          expression_start = i;
        }
      }
      _ => () 
    }
    if escape {
      escape = false;
    }
  }
  if text {
    return Err("a text is opened".to_string())
  }
  Ok(tokens)
}


// pub fn load_text<'a>(real: usize, source: &'a str) -> Result<Value,String> {
//   let mut opened = false;
//   for (i, c) in source.chars().enumerate() {
//     ' ' | '\t' | '\n' | '\r' if opened == false => (),



//   }
// }

// pub fn load_dict<'a>(real: usize, source: &'a str) -> Result<Value,String> {
//   let mut key = true;
//   let mut d: HashMap<String,Value> = HashMap::new();
//   let mut tmp_key: String = String::new(); 
//   for (i, c) in source.chars().enumerate() {
//     match c {
//       ' ' | '\t' | '\n' | '\r' if key => (),
//       '{' if i == 0 => (),
//       '"' if key => match load_text(real+i, &source[i..]) {
//         Ok(v) => {
//           let Value::Text(s) = v else { return panic!("invalid return in json::load_dict"); };
//           tmp_key = s;
//           key = false;
//         }
//         Err(e) => return Err(e)
//       }
//       _ if key == false => match load_expression(real+i, &source[i..]) {
//         Ok(v) => {
//           d.insert(
//             tmp_key.clone(),
//             v
//           );
//         }
//         Err(e) => return Err(e)
//       }
//       _ => return Err(format!("invalid key in dict (position = {})", real+i)),
//     }
//   }
//   Ok(Value::Dict(d))
// }

// pub fn load_expression<'a>(real: usize, source: &'a str) -> Result<Value,String> {
//   let mut start = true;
//   for (i, c) in source.chars().enumerate() {
//     match c {
//       ' ' | '\t' | '\n' | '\r' if start => (),
//       '{' if start => return load_dict(real+i, &source[i..]),
//       _ => ()
//     }
//   }
//   Ok(Value::None)
// }

pub fn load<'a>(source: &str) -> Result<Value,String> {
  // load_expression(0, source)
  let source_string = source.to_string();
  if let Ok(tokens) = tokenize(source) {
    for token in tokens {
      let cs = match token {
        Token::Blank(s) => source_string.chars().nth(s).unwrap().to_string(),
        Token::Expression(s,e) => (&source[s..e]).to_string(),
        Token::Text(s,e) => (&source[s..e]).to_string(),
        _ => "".to_string()
      };
      println!("{:?} = {:?}", token, cs);
    }    
  }

  Ok(Value::None)
}

