use crate::engine::extensions::Context;
use crate::engine::extensions::Helper;
use crate::engine::extensions::HelperFunction;
use crate::engine::extensions::Value;

pub static MODULE_NAME: &'static str = "default";

fn execute_fct(context: &mut Context) -> Option<String> {
  Non
}

pub fn execute(context: &mut Context) -> Option<String> {
  match &context.fct_name[..] {
    "fct" => execute_fct(context),
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
        function_name: "fct",
        function_description: "no description",
        function_can_pipe: false,
        function_args: "none",
      },
    ],
  }
}
