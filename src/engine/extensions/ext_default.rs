use crate::engine::extensions::Context;
use crate::engine::extensions::Helper;
use crate::engine::extensions::HelperFunction;
use crate::engine::extensions::Value;

pub static MODULE_NAME: &'static str = "default";

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
            Value::Text(v) => r.push(Value::Text(v.to_lowercase())),
            Value::Symbol(v) => r.push(Value::Symbol(v.to_lowercase())),
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

pub fn execute(context: &mut Context) -> Option<String> {
  match &context.fct_name[..] {
    "uppercase" => execute_uppercase(context),
    "lowercase" => execute_lowercase(context),
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
    ],
  }
}
