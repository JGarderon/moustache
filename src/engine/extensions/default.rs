use crate::engine::extensions::Context;
use crate::engine::extensions::Helper;
use crate::engine::extensions::HelperFunction;
use crate::engine::extensions::Value;

static MODULE_NAME: &'static str = "default";

fn execute_uppercase(context: &mut Context) -> Result<Value, String> {
  Ok(Value::Void)
}

pub fn execute(mut context: Context) -> Result<Value, String> {
  match context.fct_name {
    "uppercase" => execute_uppercase(&mut context),
    fct_name => Err(format!(
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
      function_description: "Upper for string",
      function_can_pipe: true,
      function_args: "as much as desired",
    }],
  }
}
