use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};

pub trait IFunction {
	fn execute(&self, env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue>;
}
