use crate::engine::extensions::Context;
use crate::engine::extensions::Helper;
use crate::engine::extensions::HelperFunction;
use crate::engine::extensions::Value;

pub static MODULE_NAME: &'static str = "text";

// --------------------------

fn recursive_uppercase(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.to_uppercase()),
    Value::Text(v) => Value::Text(v.to_uppercase()),
    Value::Vector(vector) => Value::Vector(vector.into_iter().map(recursive_uppercase).collect()),
    v => v,
  }
}

fn execute_uppercase(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_uppercase(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.to_uppercase())),
            Value::Symbol(v) => r.push(Value::Symbol(v.to_uppercase())),
            _ => r.push(recursive_uppercase(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

fn recursive_ascii_uppercase(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.to_ascii_uppercase()),
    Value::Text(v) => Value::Text(v.to_ascii_uppercase()),
    Value::Vector(vector) => {
      Value::Vector(vector.into_iter().map(recursive_ascii_uppercase).collect())
    }
    v => v,
  }
}

fn execute_ascii_uppercase(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_ascii_uppercase(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.to_ascii_uppercase())),
            Value::Symbol(v) => r.push(Value::Symbol(v.to_ascii_uppercase())),
            _ => r.push(recursive_ascii_uppercase(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

fn recursive_lowercase(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.to_lowercase()),
    Value::Text(v) => Value::Text(v.to_lowercase()),
    Value::Vector(vector) => Value::Vector(vector.into_iter().map(recursive_lowercase).collect()),
    v => v,
  }
}

fn execute_lowercase(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_lowercase(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.to_lowercase())),
            Value::Symbol(v) => r.push(Value::Symbol(v.to_lowercase())),
            _ => r.push(recursive_lowercase(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

fn recursive_ascii_lowercase(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.to_ascii_lowercase()),
    Value::Text(v) => Value::Text(v.to_ascii_lowercase()),
    Value::Vector(vector) => {
      Value::Vector(vector.into_iter().map(recursive_ascii_lowercase).collect())
    }
    v => v,
  }
}

fn execute_ascii_lowercase(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_ascii_lowercase(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.to_ascii_lowercase())),
            Value::Symbol(v) => r.push(Value::Symbol(v.to_ascii_lowercase())),
            _ => r.push(recursive_ascii_lowercase(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

fn recursive_trim(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.trim().to_string()),
    Value::Text(v) => Value::Text(v.trim().to_string()),
    Value::Vector(vector) => Value::Vector(vector.into_iter().map(recursive_trim).collect()),
    v => v,
  }
}

fn execute_trim(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_trim(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.trim().to_string())),
            Value::Symbol(v) => r.push(Value::Symbol(v.trim().to_string())),
            _ => r.push(recursive_trim(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

fn recursive_trim_start(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.trim_start().to_string()),
    Value::Text(v) => Value::Text(v.trim_start().to_string()),
    Value::Vector(vector) => Value::Vector(vector.into_iter().map(recursive_trim_start).collect()),
    v => v,
  }
}

fn execute_trim_start(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_trim_start(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.trim_start().to_string())),
            Value::Symbol(v) => r.push(Value::Symbol(v.trim_start().to_string())),
            _ => r.push(recursive_trim_start(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

fn recursive_trim_end(value: Value) -> Value {
  match value {
    Value::Symbol(v) => Value::Symbol(v.trim_end().to_string()),
    Value::Text(v) => Value::Text(v.trim_end().to_string()),
    Value::Vector(vector) => Value::Vector(vector.into_iter().map(recursive_trim_end).collect()),
    v => v,
  }
}

fn execute_trim_end(context: &mut Context) -> Option<String> {
  match context.result.take() {
    Some(v) => {
      let r = recursive_trim_end(v);
      context.result = Some(r);
      None
    }
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.trim_end().to_string())),
            Value::Symbol(v) => r.push(Value::Symbol(v.trim_end().to_string())),
            _ => r.push(recursive_trim_end(context.args.remove(i))),
          },
          None => break,
        }
        i += 1;
      }
      context.result = Some(Value::Vector(r));
      None
    }
  }
}

// --------------------------

pub fn execute(context: &mut Context) -> Option<String> {
  match &context.fct_name[..] {
    "uppercase" => execute_uppercase(context),
    "lowercase" => execute_lowercase(context),
    "ascii_uppercase" => execute_ascii_uppercase(context),
    "ascii_lowercase" => execute_ascii_lowercase(context),
    "trim" => execute_trim(context),
    "trim_start" => execute_trim_start(context),
    "trim_end" => execute_trim_end(context),
    fct_name => Some(format!(
      "module {} : unknow function name '{}'",
      MODULE_NAME, fct_name
    )),
  }
}

pub fn help() -> Helper {
  Helper {
    module_name: MODULE_NAME,
    module_description: "Generic functions (UTF-8 compatibility)",
    functions: vec![
      HelperFunction {
        function_name: "uppercase",
        function_description: "uppercase for string",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
      HelperFunction {
        function_name: "lowercase",
        function_description: "lowercase for string",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
      HelperFunction {
        function_name: "ascii_uppercase",
        function_description: "uppercase for string (only for ascii char)",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
      HelperFunction {
        function_name: "ascii_lowercase",
        function_description: "lowercase for string (only for ascii char)",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
      HelperFunction {
        function_name: "trim",
        function_description: "trim string",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
      HelperFunction {
        function_name: "trim_start",
        function_description: "trim string (start)",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
      HelperFunction {
        function_name: "trim_end",
        function_description: "trim string (end)",
        function_can_pipe: true,
        function_args: "as much as desired (priority to pipe)",
      },
    ],
  }
}
