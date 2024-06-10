use std::fs;

use crate::create_internal_error;
use crate::display_debug;
use crate::display_debug_block;
use crate::engine::environment;
use crate::engine::resolver;
use crate::engine::Environment;
use crate::utils::conf::Configuration;
use crate::utils::error::InternalError;

#[derive(Debug)]
pub struct Document<'c> {
  pub conf: &'c Configuration,
  pub source: String,
  pub stack: Vec<Part>,
}

#[allow(dead_code)]
impl<'c> Document<'c> {
  pub fn new(conf: &'c Configuration, source: String) -> Self {
    Document {
      conf,
      source,
      stack: vec![],
    }
  }
  pub fn from_str<'a>(conf: &'c Configuration, source: &'a str) -> Self {
    Document {
      conf,
      source: source.to_string(),
      stack: vec![],
    }
  }
  pub fn stack_len(&self) -> (usize, usize) {
    let mut i: usize = 0;
    let mut y: usize = 0;
    for item in self.stack.iter() {
      match item {
        Part::StaticText(_, _) => y += 1,
        _ => i += 1,
      }
    }
    (i + y, y)
  }
  pub fn stack_get(&self, position: usize) -> Option<&Part> {
    self.stack.get(position)
  }
  pub fn parse_parts(&mut self) -> Result<bool, InternalError> {
    let iter = self.source.char_indices().collect::<Vec<(usize, char)>>();
    if iter.len() == 0 {
      return Ok(false);
    }
    let mut part_type: Part = Part::StaticText(0, 0);
    let mut max = 0;
    for window in iter.windows(2) {
      if let [(ref_i, w1), (_, w2)] = window {
        let i: usize = *ref_i;
        max = i;
        match (w1, w2) {
          ('{', '{') => {
            match part_type {
              Part::StaticText(y, _) => {
                if y < i {
                  self.stack.push(Part::StaticText(y, i))
                }
              }
              p => {
                return Err(create_internal_error!(format!(
                  "not authorized : start another part 'Expression' in {:?} part",
                  p
                )));
              }
            }
            part_type = Part::Expression(i, 0);
          }
          ('}', '}') => {
            match part_type {
              Part::Expression(y, _) => self.stack.push(Part::Expression(y, i + 2)),
              p => {
                return Err(create_internal_error!(format!(
                  "not authorized : end another part 'Expression' in {:?} part",
                  p
                )));
              }
            }
            part_type = Part::StaticText(i + 2, 0);
          }
          ('{', '%') => {
            match part_type {
              Part::StaticText(y, _) => {
                if y < i {
                  self.stack.push(Part::StaticText(y, i))
                }
              }
              p => {
                return Err(create_internal_error!(format!(
                  "not authorized : start another part 'Statement' in {:?} part",
                  p
                )));
              }
            }
            part_type = Part::Statement(i, 0);
          }
          ('%', '}') => {
            match part_type {
              Part::Statement(y, _) => self.stack.push(Part::Statement(y, i + 2)),
              p => {
                return Err(create_internal_error!(format!(
                  "not authorized : end another part 'Statement' in {:?} part",
                  p
                )));
              }
            }
            part_type = Part::StaticText(i + 2, 0);
          }
          ('{', '#') => {
            match part_type {
              Part::StaticText(y, _) => {
                if y < i {
                  self.stack.push(Part::StaticText(y, i))
                }
              }
              p => {
                return Err(create_internal_error!(format!(
                  "not authorized : start another part 'Comment' in {:?} part",
                  p
                )));
              }
            }
            part_type = Part::Comment(i, 0);
          }
          ('#', '}') => {
            match part_type {
              Part::Comment(y, _) => self.stack.push(Part::Comment(y, i + 2)),
              p => {
                return Err(create_internal_error!(format!(
                  "not authorized : end another part 'Comment' in {:?} part",
                  p
                )));
              }
            }
            part_type = Part::StaticText(i + 2, 0);
          }
          _ => (),
        }
      }
    }
    let l = self.source.len();
    match part_type {
      Part::StaticText(s, _) => {
        if max < l && s < l {
          self.stack.push(Part::StaticText(s, l));
        }
      }
      Part::Statement(s, _) => {
        return Err(create_internal_error!(format!(
          "no ending for expression (start at {:?})",
          s
        )))
      }
      Part::Expression(s, _) => {
        return Err(create_internal_error!(format!(
          "no ending for expression (start at {:?})",
          s
        )))
      }
      Part::Comment(s, _) => {
        return Err(create_internal_error!(format!(
          "no ending for comment (start at {:?})",
          s
        )))
      }
      _ => (),
    }
    Ok(true)
  }
  pub fn transform(&mut self, env: &mut Environment) {
    env.transform(self);
    let mut destination: String = "".to_string();
    for p in &self.stack {
      match p {
        &Part::StaticText(s, e) => destination.push_str(&self.source[s..e]),
        Part::GeneratedText(s) => destination.push_str(&s[..]),
        &Part::Statement(s, e) => destination.push_str(&self.source[s..e]),
        &Part::Expression(s, e) => destination.push_str(&self.source[s..e]),
        Part::Comment(_, _) => (),
      }
    }
    self.stack = vec![];
    self.source = destination;
  }
  pub fn resolve(&mut self, env: &mut environment::Environment) -> Result<bool, InternalError> {
    match resolver::resolve(self, env) {
      Ok(r) => {
        if r.changed {
          self.stack = r.stack;
          Ok(true)
        } else {
          Ok(false)
        }
      }
      Err(err) => Err(err),
    }
  }
  pub fn write<'a>(&self) -> Option<String> {
    match &self.conf.output {
      Some(path) => {
        display_debug_block!(self.conf, "Try to write output", "Path = {:?}", path);
        match fs::write(path, &self.source) {
          Ok(_) => (),
          Err(err) => return Some(err.to_string()),
        }
      }
      None => print!("{}", self.source),
    }
    None
  }
}

#[derive(Debug, Clone)]
pub enum Part {
  StaticText(usize, usize),
  GeneratedText(String),
  Statement(usize, usize),
  Expression(usize, usize),
  Comment(usize, usize),
}
