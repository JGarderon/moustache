use std::env;

// use utils_macro::modifier_item;
use crate::engine::extensions::default;
use crate::utils::APP_AUTHOR;
use crate::utils::APP_DATE;
use crate::utils::APP_NAME;
use crate::utils::APP_VERSION;

use crate::utils::conf::Configuration;

pub fn parse() -> Result<Configuration, String> {
  let mut c = Configuration::new();
  let mut iter = env::args().skip(1).peekable();
  loop {
    let arg = iter.next();
    if arg.is_none() {
      break;
    }
    match arg.unwrap().as_ref() {
      "--help" | "-h" => c.is_helping(true),
      "--help-extensions" => c.is_helping_extensions(true),
      "--debug" | "-d" => c.is_debugging(true),
      "--version" => c.display_version(true),
      "--input" | "-i" => match iter.peek() {
        Some(next_argument) => c.input = Some(next_argument.to_string()),
        None => {
          return Err(format!(
            "the input has been declared but not defined in the command line parameters"
          ))
        }
      },
      "--output" | "-o" => match iter.peek() {
        Some(next_argument) => c.output = Some(next_argument.to_string()),
        None => {
          return Err(format!(
            "the output has been declared but not defined in the command line parameters"
          ))
        }
      },
      "--var" | "-v" => match iter.peek() {
        Some(next_argument) => {
          if let Some((k, v)) = next_argument.split_once('=') {
            c.variables.insert(k.to_string(), v.to_string());
          } else {
            return Err(format!(
              "the value of variable in the command line parameters is invalid"
            ));
          }
        }
        None => {
          return Err(format!(
            "the variable has not been defined in the command line parameters"
          ))
        }
      },
      "--reentrant" | "-r" => c.is_reentrant(true),
      _ => (),
    }
  }
  Ok(c)
}

// #[modifier_item("--help","help")]
pub fn display_helping() {
  println!(
    "{} - {} ({})
by {}

  --help      | -h    display this message and exit (0)
  --debug     | -d    display the debug
  --input     | -i    input of process (path ; else stdin)
  --output    | -o    output of process (path)
  --var       | -v    add var to env 
  --reentrant | -r    document is reentrant 
",
    APP_NAME, APP_VERSION, APP_DATE, APP_AUTHOR,
  );
}

// #[modifier_item("--help","help")]
pub fn display_version() {
  println!("{}", APP_VERSION);
}

pub fn display_helping_extensions() {
  println!(
    "{} - {} ({})
by {}

  Extensions documentation",
    APP_NAME, APP_VERSION, APP_DATE, APP_AUTHOR,
  );
  println!("helping extensions = {:?}", default::help());


}
