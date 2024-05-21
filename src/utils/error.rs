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
    eprint!("\n-- ");
    if error_formatting {
      eprint!("\x1b[5m\x1b[1m\x1b[31m");
    }
    eprint!("ERROR FOUND");
    if error_formatting {
      eprint!("\x1b[0m");
    }
    eprintln!("\n");
    let mut deep = 0;
    while let Some(InternalErrorStep {
      message,
      file,
      line,
      mut infos,
    }) = self.stack.pop()
    {
      eprint!("[{}] >> ", deep);
      if error_formatting {
        eprint!("\x1b[93m");
      }
      eprint!("{}", message);
      if error_formatting {
        eprint!("\x1b[0m");
      }
      eprint!("\n       ");
      if error_formatting {
        eprint!("\x1b[38;5;244m(");
      }
      eprint!("{}/{}#{})", APP_VERSION, file, line);
      if error_formatting {
        eprint!("\x1b[0m");
      }
      eprintln!();
      if infos.len() > 0 {
        while let Some(line) = infos.pop() {
          eprint!("       ");
          if error_formatting {
            eprint!("\x1b[38;5;253m");
          }
          eprint!("{}", line);
          if error_formatting {
            eprint!("\x1b[0m");
          }
          eprintln!();
        }
      }
      eprintln!();
      deep += 1;
    }
    eprintln!("--\n");
  }
}

#[macro_export]
macro_rules! add_step_internal_error {
    ($instance:expr, $content:expr) => {
        {
          $instance.add_step($content, (file!()).to_string(), line!(), vec!());
          $instance
        }
    };
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
    ($content:expr) => {
        {
          let mut e: InternalError = InternalError::new();
          e.add_step($content, (file!()).to_string(), line!(), vec!());
          e
        }
    };
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
