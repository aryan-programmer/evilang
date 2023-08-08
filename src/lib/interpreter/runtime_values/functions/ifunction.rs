use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};

pub trait IFunction {
	fn call(&self, params: FunctionParameters) -> ResultWithError<FunctionReturnValue>;
}
