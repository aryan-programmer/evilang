use gc::{Finalize, Trace};

use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::functions::closure::Closure;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};

pub mod closure;
pub mod types;
pub mod ifunction;

#[derive(Debug, PartialEq, Trace, Finalize)]
pub enum Function {
	// NativeFunction,
	Closure(Closure)
}

impl IFunction for Function {
	fn call(&self, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
		return match self {
			Function::Closure(cl) => cl.call(params)
		};
	}
}
