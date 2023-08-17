use std::collections::HashMap;

use crate::ast::expression::IdentifierT;
use crate::errors::{ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::native_function::{NativeFunction, NativeFunctionFn};
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::{expect_object, get_object_superclass};
use crate::interpreter::utils::consts::INSTANCE_OF_;

pub fn push_res_stack(env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
	env.global_scope.borrow_mut().res_stack.extend(params.into_iter());
	Ok(PrimitiveValue::Null.into())
}

pub fn allocate_object(
	env: &mut Environment,
	mut params: FunctionParameters,
) -> ResultWithError<FunctionReturnValue> {
	if params.len() > 2 {
		return Err(RuntimeError::InvalidNumberArgumentsToFunction {
			expected: Some("0 to 2".to_string()),
			got: params.len(),
			func: "allocate_object".into(),
		}.into());
	}
	let name_opt = if params.len() == 2 { params.pop() } else { None };
	let class_val_opt = if params.len() == 1 { params.pop() } else { None };
	let object_class = if let Some(v) = class_val_opt {
		expect_object(v.into(), None)?
	} else {
		get_object_superclass(env)?
	};
	let name = if let Some(PrimitiveValue::String(ref s)) = name_opt { s.clone() } else {
		INSTANCE_OF_.to_string() + &object_class.name
	};
	return Ok(PrimitiveValue::Object(RuntimeObject::allocate(object_class, name)));
}

pub fn make_native_functions_list() -> HashMap<IdentifierT, NativeFunction> {
	return HashMap::from_iter([
		("push_res_stack", push_res_stack as NativeFunctionFn),
		("allocate_object", allocate_object as NativeFunctionFn)
	].into_iter().map(|(name, val)| (name.to_string(), NativeFunction::new(val))));
}
