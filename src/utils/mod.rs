pub mod args;
pub mod conf;
pub mod error;

pub static APP_NAME: &'static str = "Moustache";
pub static APP_VERSION: &'static str = "v1.0.0";
pub static APP_DATE: &'static str = "april 2024";
pub static APP_AUTHOR: &'static str = "Julien Garderon <julien.garderon@gmail.com>";

#[macro_export]
macro_rules! display_debug {
  ($conf:expr, $content:expr) => {
    if $conf.is_debugging {
      eprintln!($content);
    }
  };
  ($conf:expr, $content:expr $(, $arg:expr)*) => {
    if $conf.is_debugging {
      eprintln!(
        $content,
        $( $arg, )*
      );
    }
  };
}

#[macro_export]
macro_rules! display_debug_title {
  ($conf:expr, $content:expr) => {
    if $conf.is_debugging {
      let s: String;
      if $conf.error_formatting {
         s = format!("\x1b[1m\x1b[38;5;154m{}\x1b[0m", $content);
      } else {
        s = format!("{}", $content);
      }
      display_debug!($conf, "-- {}\n", s);
    }
  };
  ($conf:expr, $content:expr $(, $arg:expr)*) => {
    if $conf.is_debugging {
      let s: String;
      if $conf.error_formatting {
         s = format!("\x1b[1m\x1b[38;5;154m{}\x1b[0m", format!($content $( ,$arg)*));
      } else {
        s = format!("{}", format!($content $( ,$arg)*));
      }
      display_debug!($conf, "-- {}\n", s);
    }
  };
}

#[macro_export]
macro_rules! display_debug_block {
  ($conf:expr, $title:expr, $content:expr) => {
    if $conf.is_debugging {
      let t: String;
      let mut s: String;
      if $conf.error_formatting {
        t = format!("\x1b[38;5;148m{}\x1b[0m", $title);
        s = format!("  \x1b[38;5;230m{}\x1b[0m", $content);
      } else {
        t = format!("{}", $title);
        s = format!("  {}", $content);
      }
      s = s.replace("\n","\n  ");
      display_debug!($conf, "{}\n{}\n", t, s);
    }
  };
  ($conf:expr, $title:expr, $content:expr $(, $arg:expr)*) => {
    if $conf.is_debugging {
      let t: String;
      let mut s: String;
      if $conf.error_formatting {
        t = format!("\x1b[38;5;148m{}\x1b[0m", $title);
        s = format!("  \x1b[38;5;230m{}\x1b[0m", format!($content $( ,$arg)*));
      } else {
        t = format!("{}", $title);
        s = format!("  {}", format!($content $( ,$arg)*));
      }
      s = s.replace("\n","\n  ");
      display_debug!($conf, "{}\n{}\n", t, s);
    }
  };
}
