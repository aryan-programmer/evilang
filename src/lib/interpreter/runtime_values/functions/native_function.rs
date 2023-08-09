use gc::{Finalize, Trace, unsafe_empty_trace};

use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};

#[derive(Debug, PartialEq)]
pub struct NativeFunction {
	pub f: fn(env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue>,
}

impl Finalize for NativeFunction {}

unsafe impl Trace for NativeFunction {
	unsafe_empty_trace!();
}

impl NativeFunction {
	pub fn new(f: fn(&mut Environment, FunctionParameters) -> ResultWithError<FunctionReturnValue>) -> Self {
		Self { f }
	}
}

impl IFunction for NativeFunction {
	#[inline(always)]
	fn execute(&self, env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
		return (self.f)(env, params);
	}
}
