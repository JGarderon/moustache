use std::fs::File;
use std::io;
use std::io::Read;

mod engine;
mod utils;

fn main() {
  let conf = match utils::args::parse() {
    Ok(conf) => conf,
    Err(err) => {
      println!("Fatal error : {}", err);
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
  if conf.is_debugging {
    println!("conf = {:?}", conf);
  }

  let mut env = engine::Environment::from_args(&conf);

  let mut buffer = String::new();
  match conf.input {
    Some(ref path) => match File::open(path) {
      Ok(mut fd) => match fd.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(err) => {
          println!(
            "Error during read input from file descriptor = {:?}",
            err.to_string()
          );
          std::process::exit(1);
        }
      },
      Err(err) => {
        println!("Error during open input = {:?}", err.to_string());
        std::process::exit(1);
      }
    },
    None => match io::stdin().read_to_string(&mut buffer) {
      Ok(_) => (),
      Err(err) => {
        println!("Error during read input from stdin = {:?}", err.to_string());
        std::process::exit(1);
      }
    },
  };
  let mut doc = engine::Document::new(&conf, buffer);

  let mut reentrance: usize = 0;
  loop {
    if conf.is_debugging {
      println!("--- Reentrance n°{:?}", reentrance);
    }
    match doc.parse_parts() {
      Ok(r) => {
        if conf.is_debugging {
          if r {
            println!("Parse parts = part(s) found (n = {})", doc.stack_len());
          } else {
            println!("Parse parts = nothing to do (blank source)");
            break;
          }
        }
      }
      Err(err) => {
        println!("Fatal error during parsing : {:?}", err);
        std::process::exit(1);
      }
    }
    if conf.is_debugging {
      println!("{}", doc.debug_stack())
    }

    match doc.resolve(&mut env) {
      Ok(changed) => {
        if conf.is_debugging {
          println!("Resolved, source changed = {:?}", changed);
        }
        if changed {
          doc.transform();
        } else {
          if conf.is_debugging && reentrance > 0 {
            println!("Resolve parts = nothing to do (no change)");
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

  match &doc.conf.output {
    Some(path) => match doc.write(&path[..]) {
      Some(err) => {
        println!("Error during write output = {:?}", err);
        std::process::exit(1);
      }
      None => {
        if conf.is_debugging {
          println!("Write output to {:?}", path);
        }
      }
    },
    None => println!("{}", doc.source),
  }
  std::process::exit(0);
}
