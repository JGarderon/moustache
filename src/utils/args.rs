use std::env;

// use utils_macro::modifier_item;
use crate::engine::extensions;
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
      "--reentrant" | "-r" => c.is_reentrant(true),
      "--no-extensions" => c.no_extensions(true),
      "--error-formatting" => c.error_formatting(true),
      "--skip-first-line" => c.skip_first_line(true),
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
      _ => (),
    }
  }
  Ok(c)
}

// #[modifier_item("--help","help")]
pub fn display_helping(resume: bool) {
  print!(
    "
{} - {} ({})
by {}
",
    APP_NAME, APP_VERSION, APP_DATE, APP_AUTHOR,
  );
  if resume {
    println!(
      "
  --help      | -h    display this message and exit (0)
  --debug     | -d    display the debug

  --input +   | -i +  input of process (path ; else stdin) - with arg
  --output +  | -o +  output of process (path) - with arg
  --var +     | -v +  add var to env - with arg 
  --reentrant | -r    document is reentrant 
  --error-formatting  support of ANSI color and style codes 
  --skip-first-line   removes the first line of the output, 
                      for example in the case where the call is made 
                      via a shebang of the source file

  --help-extensions   display extensions documentation and exit (0)
  --no-extensions     disable extensions (with error)
"
    );
  }
}

// #[modifier_item("--help","help")]
pub fn display_version() {
  println!("{}", APP_VERSION);
}

pub fn display_helping_extensions() {
  display_helping(false);
  print!(
    "
Extensions documentation
------------------------
"
  );
  extensions::help();
}
