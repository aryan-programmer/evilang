use gc::{Finalize, Trace};

use crate::ast::structs::FunctionDeclaration;
use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::closure::Closure;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::utils::cell_ref::{gc_clone, GcPtr};

pub mod closure;
pub mod types;
pub mod ifunction;
pub mod native_function;

pub type GcPtrToFunction = GcPtr<Function>;

#[derive(Debug, PartialEq, Trace, Finalize)]
pub enum Function {
	NativeFunction(NativeFunction),
	Closure(Closure),
}

impl Function {
	pub fn new_closure(env: &Environment, decl: FunctionDeclaration) -> GcPtrToFunction {
		let closure = Closure::new(
			decl,
			gc_clone(&env.scope),
		);
		let function_closure = Function::Closure(closure);
		return GcPtr::new(function_closure);
	}
}

impl IFunction for Function {
	#[inline(always)]
	fn execute(&self, env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
		return match self {
			Function::Closure(cl) => cl.execute(env, params),
			Function::NativeFunction(f) => f.execute(env, params),
		};
	}
}
