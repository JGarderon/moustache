use crate::engine::extensions::Context;
use crate::engine::extensions::Helper;
use crate::engine::extensions::HelperFunction;
use crate::engine::extensions::Value;

pub static MODULE_NAME: &'static str = "macro";

fn transform_to_text(v: &Value) -> String {
  match v {
    Value::Text(t) => t.to_string(),
    Value::Symbol(s) => s.to_string(),
    Value::Number(n) => n.to_string(),
    Value::Vector(v) => {
      if v.len() == 0 {
        "".to_string()
      } else {
        v.iter()
          .map(|s| transform_to_text(s))
          .reduce(|acc: String, next: String| acc + &next)
          .unwrap()
      }
    }
    Value::True => "true".to_string(),
    Value::False | Value::Void => "".to_string(),
  }
}

fn execute_convert_to_text(context: &mut Context) -> Option<String> {
  let v = match context.result.take() {
    Some(v) => v,
    None => match context.args.len() {
      0 => return Some("void pipe and arg".to_string()),
      1 => match context.args.remove(0) {
        Value::Text(t) => Value::Text(t.to_string()),
        Value::Symbol(s) => Value::Text(s.to_string()),
        v => return Some(format!("invalid token {:?}", v)),
      },
      _ => return Some("too much args".to_string()),
    },
  };
  let r = transform_to_text(&v);
  context.result = Some(Value::Text(r));
  None
}

fn execute_convert_to_symbol(context: &mut Context) -> Option<String> {
  let v = match context.result.take() {
    Some(v) => v,
    None => match context.args.len() {
      0 => return Some("void pipe and arg".to_string()),
      1 => match context.args.remove(0) {
        Value::Text(t) => Value::Text(t.to_string()),
        Value::Symbol(s) => Value::Text(s.to_string()),
        v => return Some(format!("invalid token {:?}", v)),
      },
      _ => return Some("too much args".to_string()),
    },
  };
  let r = transform_to_text(&v);
  if r.is_empty() {
    return Some("the conversion produced an empty symbol (not allowed)".to_string());
  }
  context.result = Some(Value::Symbol(r));
  None
}

pub fn execute(context: &mut Context) -> Option<String> {
  match &context.fct_name[..] {
    "convert_to_text" => execute_convert_to_text(context),
    "convert_to_symbol" => execute_convert_to_symbol(context),
    fct_name => Some(format!(
      "module {} : unknow function name '{}'",
      MODULE_NAME, fct_name
    )),
  }
}

pub fn help() -> Helper {
  Helper {
    module_name: MODULE_NAME,
    module_description: "Macro functions",
    functions: vec![
      HelperFunction {
        function_name: "convert_to_text",
        function_description: "Convert any token or arg to a text",
        function_can_pipe: true,
        function_args: "Token or only one arg (priority to pipe)",
      },
      HelperFunction {
        function_name: "convert_to_symbol",
        function_description: "Convert any token or arg to a non-null symbol",
        function_can_pipe: true,
        function_args: "Token or only one arg (priority to pipe)",
      },
    ],
  }
}
