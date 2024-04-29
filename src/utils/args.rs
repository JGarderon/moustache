// use std::collections::HashMap;
use std::env;
// use utils_macro::modifier_item;

use crate::utils::conf::Configuration;

pub fn parse() -> Result<Configuration,String> {
  let mut c = Configuration::new();
  let mut iter = env::args().skip(1).peekable(); 
  loop {
    let arg = iter.next(); 
    if arg.is_none() {
      break;
    }
    let arg_next = iter.next(); 
    match arg.unwrap().as_ref() {
      "--help" | "-h" => c.is_helping(true),
      "--debug" | "-d" => c.is_debugging(true),
      "--input" | "-i" => match arg_next {
        Some(next_argument) => c.input = Some(next_argument.to_string()),
        None => return Err(format!(
          "the input has been declared but not defined in the command line parameters"
        ))
      }
      "--output" | "-o" => match arg_next {
        Some(next_argument) => c.output = Some(next_argument.to_string()),
        None => return Err(format!(
          "the output has been declared but not defined in the command line parameters"
        ))
      }
      "--var" | "-v" => match arg_next {
        Some(next_argument) => if let Some((k,v)) = next_argument.split_once('=') {
          c.variables.insert(
            k.to_string(),
            v.to_string()
          );
        } else {
          return Err(format!(
            "the variable in the command line parameters is invalid"
          ))
        }
        None => return Err(format!(
          "the variable has not been defined in the command line parameters"
        ))
      }
      _ => ()
    }
  }
  Ok(c)
}

// #[modifier_item("--help","help")]
pub fn display_helping() {
  println!(
"Moustache - v0.0.1 (april 2024)
by Julien Garderon <julien.garderon@gmail.com>

  --help    | -h    display this message and exit (0)
  --debug   | -d    display the debug
  --input   | -i    input of process (path ; else stdin)
  --output  | -o    output of process (path)
  --var     | -v    add var to env 
"
  );
}