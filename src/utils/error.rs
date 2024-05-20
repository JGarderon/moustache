use crate::utils::APP_VERSION;

#[derive(Debug)]
struct InternalErrorStep {
  message: String,
  file: String,
  line: u32,
  infos: Vec<String>,
}

#[derive(Debug)]
pub struct InternalError {
  stack: Vec<InternalErrorStep>,
}

impl InternalError {
  pub fn new() -> Self {
    InternalError { stack: vec![] }
  }
  pub fn add_step<T: Into<String>>(
    &mut self,
    step: T,
    file: String,
    line: u32,
    infos: Vec<String>,
  ) {
    self.stack.push(InternalErrorStep {
      message: step.into(),
      file,
      line,
      infos,
    });
  }
  pub fn display(&mut self, error_formatting: bool) {
    print!("\n-- ");
    if error_formatting {
      print!("\x1b[5m\x1b[1m\x1b[31m");
    }
    print!("ERROR FOUND");
    if error_formatting {
      print!("\x1b[0m");
    }
    println!("\n");
    let mut deep = 0;
    while let Some(InternalErrorStep {
      message,
      file,
      line,
      mut infos,
    }) = self.stack.pop()
    {
      print!("[{}] >> ", deep);
      if error_formatting {
        print!("\x1b[93m");
      }
      print!("{}", message);
      if error_formatting {
        print!("\x1b[0m");
      }
      print!("\n       ");
      if error_formatting {
        print!("\x1b[38;5;244m(");
      }
      print!("{}/{}#{})", APP_VERSION, file, line);
      if error_formatting {
        print!("\x1b[0m");
      }
      println!();
      if infos.len() > 0 {
        while let Some(line) = infos.pop() {
          print!("       ");
          if error_formatting {
            print!("\x1b[38;5;253m");
          }
          print!("{}", line);
          if error_formatting {
            print!("\x1b[0m");
          }
          println!();
        }
      }
      println!();
      deep += 1;
    }
    println!("--\n");
  }
}

#[macro_export]
macro_rules! add_step_internal_error {
    ($instance:expr, $content:expr $(, $content_sup:expr)*) => {
        {
          let mut infos: Vec<String> = vec!();
          $(
              infos.push($content_sup.into());
          )*
          $instance.add_step($content, (file!()).to_string(), line!(), infos);
          $instance
        }
    };
}

#[macro_export]
macro_rules! create_internal_error {
    ($content:expr $(, $content_sup:expr)*) => {
        {
          let mut e: InternalError = InternalError::new();
          let mut infos: Vec<String> = vec!();
          $(
              infos.push($content_sup.into());
          )*
          e.add_step($content, (file!()).to_string(), line!(), infos);
          e
        }
    };
}
