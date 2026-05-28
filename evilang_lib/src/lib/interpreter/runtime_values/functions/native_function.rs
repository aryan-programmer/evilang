use gc::{unsafe_empty_trace, Finalize, Trace};
use uuid::Uuid;

use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{
	FunctionParameters, FunctionReturnValue,
};

pub type NativeFunctionFn =
	fn(env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue>;

#[derive(Debug)]
pub struct NativeFunction {
	pub f: NativeFunctionFn,
	pub comparision_id: Uuid,
}

impl PartialEq for NativeFunction {
	fn eq(&self, other: &Self) -> bool {
		self.comparision_id == other.comparision_id
	}
}

impl Finalize for NativeFunction {}

unsafe impl Trace for NativeFunction {
	unsafe_empty_trace!();
}

impl NativeFunction {
	pub fn new(f: NativeFunctionFn) -> Self {
		Self {
			f,
			comparision_id: Uuid::new_v4(),
		}
	}
}

impl IFunction for NativeFunction {
	#[inline(always)]
	fn execute(
		&self,
		env: &mut Environment,
		params: FunctionParameters,
	) -> ResultWithError<FunctionReturnValue> {
		return (self.f)(env, params);
	}
}
