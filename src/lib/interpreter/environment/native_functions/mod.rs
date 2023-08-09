use std::collections::HashMap;

use crate::ast::expression::IdentifierT;
use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::PrimitiveValue;

pub fn push_res_stack(env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
	env.global_scope.borrow_mut().res_stack.extend(params.into_iter());
	Ok(PrimitiveValue::Null.into())
}

pub fn get_native_functions_list() -> HashMap<IdentifierT, NativeFunction> {
	return HashMap::from_iter([
		("push_res_stack", push_res_stack),
	].into_iter().map(|(name, val)| (name.to_string(), NativeFunction::new(val))));
}
