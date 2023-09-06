use std::collections::HashMap;

use crate::ast::expression::IdentifierT;
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::native_items::classes::object::ObjectSuperclass;
use crate::interpreter::runtime_values::functions::native_function::NativeFunctionFn;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::i_native_struct::INativeClass_GetClassCached;
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::expect_object;

pub mod classes;

pub fn push_res_stack(env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
	env.global_scope.borrow_mut().res_stack.extend(params.into_iter());
	Ok(PrimitiveValue::Null.into())
}

pub fn to_string(_env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
	Ok(PrimitiveValue::String(match params.first().unwrap() {
		PrimitiveValue::Null => { "null".to_string() }
		PrimitiveValue::Boolean(v) => { if *v { "true" } else { "false" }.to_string() }
		PrimitiveValue::Number(num) => { num.to_string() }
		PrimitiveValue::String(v) => { v.clone() }
		PrimitiveValue::Function(function) => { function.to_string() }
		pv => {
			return Err(RuntimeError::InvalidArgumentsToFunction(
				"Can't convert to string".to_string(),
				Descriptor::Value(pv.clone__silently_fail()),
			).into());
		}
	}))
}

pub fn print(_env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
	for x in params.into_iter() {
		print!("{}", x);
	}
	Ok(PrimitiveValue::Null.into())
}

pub fn debug(_env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
	println!("{0:#?}", params);
	Ok(PrimitiveValue::Null.into())
}

pub fn allocate_object(
	env: &mut Environment,
	mut params: FunctionParameters,
) -> ResultWithError<FunctionReturnValue> {
	if params.len() > 2 {
		return Err(RuntimeError::InvalidNumberArgumentsToFunction {
			expected: Some("0 to 2".into()),
			got: params.len(),
			func: "allocate_object".into(),
		}.into());
	}
	let name_opt = if params.len() == 2 { params.pop() } else { None };
	let class_val_opt = if params.len() == 1 { params.pop() } else { None };
	let object_class = if let Some(v) = class_val_opt {
		expect_object(v.into(), None)?
	} else {
		ObjectSuperclass::get_class_cached(env)?
	};
	let name = name_opt.and_then(|v| v.consume_as_string().left());
	return Ok(PrimitiveValue::Object(RuntimeObject::allocate_instance(object_class, name)));
}

pub fn make_native_functions_list() -> HashMap<IdentifierT, NativeFunctionFn> {
	return HashMap::from_iter([
		("push_res_stack", push_res_stack as NativeFunctionFn),
		("debug", debug as NativeFunctionFn),
		("print", print as NativeFunctionFn),
		("allocate_object", allocate_object as NativeFunctionFn),
		("to_string", to_string as NativeFunctionFn),
	].into_iter().map(|(name, val)| (name.into(), val)));
}
