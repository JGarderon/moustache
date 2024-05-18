use crate::engine::extensions::Context;
use crate::engine::extensions::Helper;
use crate::engine::extensions::HelperFunction;
use crate::engine::extensions::Value;

static MODULE_NAME: &'static str = "default";

fn execute_uppercase(context: &mut Context) -> Option<String> {
  match &context.result {
    Some(v) => match v {
      Value::Text(v) => {
        context.result = Some(Value::Text(v.to_uppercase()));
        None
      }
      v => Some(format!("can't format 'uppercase' on {:?} (via pipe)", v)),
    },
    None => {
      let mut r: Vec<Value> = vec![];
      let mut i: usize = 0;
      loop {
        match context.args.get(i) {
          Some(a) => match a {
            Value::Text(v) => r.push(Value::Text(v.to_uppercase())),
            v => return Some(format!("can't format 'uppercase' on {:?} (via args)", v)),
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
    fct_name => Some(format!(
      "module {} : unknow function name '{}'",
      MODULE_NAME, fct_name
    )),
  }
}

pub fn help() -> Helper {
  Helper {
    module_name: "Default",
    module_description: "Extension for generic functions",
    module_autor: "Julien Garderon <julien.garderon@gmail.com>",
    module_version: "v1",
    functions: vec![HelperFunction {
      function_name: "uppercase",
      function_description: "Uppercase for string",
      function_can_pipe: true,
      function_args: "as much as desired",
    }],
  }
}
