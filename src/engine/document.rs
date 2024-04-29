
use crate::engine::resolver;

#[derive(Debug)]
pub struct Document {
  source: String,
  stack: Vec<Part>
}

impl Document {
  pub fn new(source: String) -> Self {
    Document {
      source: source,
      stack: vec!()
    }
  }
  pub fn from_str<'a>(source: &'a str) -> Self {
    Document {
      source: source.to_string(),
      stack: vec!()
    }
  }
  pub fn stack_len(&self) -> usize {
    self.stack.len()
  }
  pub fn stack_get(&self, position: usize) -> Option<&Part> {
    self.stack.get(position)
  }
  pub fn parse_parts(&mut self) -> Result<bool,String> {
    let iter = self.source.chars().collect::<Vec<char>>(); 
    if iter.len() == 0 {
      return Ok(false);
    }
    let mut part_type: Part = Part::StaticText(0,0);
    let mut max = 0; 
    for (i, w) in iter.windows(2).enumerate() {
      max = i;
      match w {
        ['{','{'] => {
          match part_type {
            Part::StaticText(y,_) => self.stack.push(Part::StaticText(y, i)),
            p => return Err(format!("not authorized : start another part 'Expression' in {:?} part", p))
          }
          part_type = Part::Expression(i,0);
        }
        ['}','}'] => {
          match part_type {
            Part::Expression(y,_) => self.stack.push(Part::Expression(y, i+2)),
            p => return Err(format!("not authorized : end another part 'Expression' in {:?} part", p))
          }
          part_type = Part::StaticText(i+2,0);
        }
        ['{','%'] => {
          match part_type {
            Part::StaticText(y,_) => self.stack.push(Part::StaticText(y, i)),
            p => return Err(format!("not authorized : start another part 'Statement' in {:?} part", p))
          }
          part_type = Part::Statement(i,0);
        }
        ['%','}'] => {
          match part_type {
            Part::Statement(y,_) => self.stack.push(Part::Statement(y, i+2)),
            p => return Err(format!("not authorized : end another part 'Statement' in {:?} part", p))
          }
          part_type = Part::StaticText(i+2,0);
        }
        ['{','#'] => {
          match part_type {
            Part::StaticText(y,_) => self.stack.push(Part::StaticText(y, i)),
            p => return Err(format!("not authorized : start another part 'Comment' in {:?} part", p))
          }
          part_type = Part::Comment(i,0);
        }
        ['#','}'] => {
          match part_type {
            Part::Comment(y,_) => self.stack.push(Part::Comment(y, i+2)),
            p => return Err(format!("not authorized : end another part 'Comment' in {:?} part", p))
          }
          part_type = Part::StaticText(i+2,0);
        }
        _ => ()
      }
    }
    let l = self.source.len(); 
    match part_type {
      Part::StaticText(s,_) => if max < l && s < l {
        self.stack.push(Part::StaticText(s,l));
      }
      Part::Statement(s,_) => return Err(format!("no ending for expression (start at {:?})", s)),
      Part::Expression(s,_) => return Err(format!("no ending for expression (start at {:?})", s)),
      Part::Comment(s,_) => return Err(format!("no ending for comment (start at {:?})", s)),
      _ => ()
    }
    Ok(true)
  }
  pub fn debug_stack(&self) -> String {
    let mut string = "\n---\n[DEBUG STACK]".to_string(); 
    for p in &self.stack {
      match p {
        Part::StaticText(s,e) => string.push_str(&format!("\n>> static text... {}", &self.source[*s..*e])[..]),
        Part::GeneratedText(s) => string.push_str(&format!("\n>> generated text... {}", s)[..]),
        Part::Statement(s,e) => string.push_str(&format!("\n>> statement... {}", &self.source[*s..*e])[..]),
        Part::Expression(s,e) => string.push_str(&format!("\n>> expression... {}", &self.source[*s..*e])[..]),
        Part::Comment(s,e) => string.push_str(&format!("\n>> comment text... {}", &self.source[*s..*e])[..])
      }
    }
    string.push_str("\n---\n");
    return string; 
  }
  pub fn transform(self) -> Self {
    let mut destination: String = "".to_string();
    for p in &self.stack {
      match p {
        Part::StaticText(s,e) => destination.push_str(&self.source[*s..*e]),
        Part::GeneratedText(s) => destination.push_str(&s[..]),
        Part::Statement(s,e) => destination.push_str(&self.source[*s..*e]),
        Part::Expression(s,e) => destination.push_str(&self.source[*s..*e]),
        Part::Comment(_,_) => ()
      }
    }
    Document {
      source: destination,
      stack: vec!()
    }
  }
  pub fn resolve(&mut self) -> Result<bool,String> {
    match resolver::resolve(self) {
      Ok(v) if v.len() == 0 => Ok(false),
      Ok(v) => {
        self.stack = v;
        Ok(true)
      }
      Err(err) => Err(err)
    }
  }
}

#[derive(Debug)]
pub enum Part {
  StaticText(usize,usize),
  GeneratedText(String),
  Statement(usize,usize),
  Expression(usize,usize),
  Comment(usize,usize)
}
