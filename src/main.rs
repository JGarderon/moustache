use std::fs::File;
use std::io;
use std::io::Read;

mod engine;
mod utils;

use crate::utils::error::InternalError;

fn main() {
  let conf = match utils::args::parse() {
    Ok(conf) => conf,
    Err(err) => {
      create_internal_error!(
        "A fatal error occurred during configuration loading.",
        format!("Error details = {}", err)
      )
      .display(false);
      std::process::exit(1);
    }
  };
  if conf.is_helping {
    utils::args::display_helping(true);
    std::process::exit(0);
  }
  if conf.is_helping_extensions {
    utils::args::display_helping_extensions();
    std::process::exit(0);
  }
  if conf.display_version {
    utils::args::display_version();
    std::process::exit(0);
  }

  display_debug!(conf, "");
  display_debug_title!(conf, "Debugging is active");
  display_debug_block!(
    conf,
    "Configuration state at the beginning of the process",
    "{}",
    conf.display()
  );

  let mut env = engine::Environment::from_args(&conf);

  let mut buffer = String::new();
  match conf.input {
    Some(ref path) => match File::open(path) {
      Ok(mut fd) => match fd.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(err) => {
          create_internal_error!(
            "A fatal error occurred during reading input from file descriptor",
            format!("Error details = {}", err)
          )
          .display(conf.error_formatting);
          std::process::exit(1);
        }
      },
      Err(err) => {
        create_internal_error!(
          "A fatal error occurred during opening input",
          format!("Error details = {}", err.to_string())
        )
        .display(conf.error_formatting);
        std::process::exit(1);
      }
    },
    None => match io::stdin().read_to_string(&mut buffer) {
      Ok(_) => (),
      Err(err) => {
        create_internal_error!(
          "A fatal error occurred during reading from STDIN",
          format!("Error details = {}", err.to_string())
        )
        .display(conf.error_formatting);
        std::process::exit(1);
      }
    },
  };
  let mut doc = engine::Document::new(&conf, buffer);
  let mut reentrance: usize = 0;
  loop {
    display_debug_title!(conf, "Reentrant step n°{}", reentrance);
    match doc.parse_parts() {
      Ok(r) => {
        if r {
          let (n_parts, n_statictexts) = doc.stack_len();
          display_debug_block!(
            conf,
            "Parse parts",
            "parts found (total = {}, static text = {})",
            n_parts,
            n_statictexts
          );
        } else {
          display_debug_block!(conf, "Parse parts", "Nothing to do");
          break;
        }
      }
      Err(mut err) => {
        add_step_internal_error!(err, "A fatal error occurred during parsing document")
          .display(conf.error_formatting);
        std::process::exit(1);
      }
    }
    match doc.resolve(&mut env) {
      Ok(changed) => {
        if changed {
          let _ = display_debug_block!(conf, "Resolve parts", "Document is changed");
          doc.transform(&mut env);
        } else {
          if reentrance > 0 {
            let _ = display_debug_block!(conf, "Resolve parts", "Document is not changed");
          }
          break;
        }
      }
      Err(mut err) => {
        err = add_step_internal_error!(err, "Error during resolving");
        err.display(conf.error_formatting);
        std::process::exit(1);
      }
    }
    if conf.is_reentrant == false {
      break;
    }
    reentrance += 1;
  }

  if doc.conf.skip_first_line {
    doc.source = doc
      .source
      .split("\n")
      .skip(1)
      .collect::<Vec<_>>()
      .join("\n");
  }

  if let Some(err) = doc.write() {
    create_internal_error!(
      "Error during write output",
      format!("Error details = {}", err)
    )
    .display(conf.error_formatting);
    std::process::exit(1);
  }

  display_debug_title!(conf, "End of program, no errors");
  std::process::exit(0);
}
