use std::fmt::{Display, Formatter};

use gc::{Finalize, Trace};

use crate::ast::structs::FunctionDeclaration;
use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::closure::Closure;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::types::cell_ref::{gc_clone, GcPtr};

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

impl Display for Function {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Function::NativeFunction(nf) => {
				let mut res = Ok(());
				backtrace::resolve(nf.f as *mut std::os::raw::c_void, |v| {
					res = if let Some(name) = v.name() {
						std::fmt::Display::fmt(&name, f)
					} else {
						f.write_str("native function")
					};
				});
				res
			}
			Function::Closure(cl) => f.write_str(cl.code.name.as_str())
		}
	}
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
