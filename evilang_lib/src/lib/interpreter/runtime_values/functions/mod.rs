use std::fmt::{ Display, Formatter };
use std::ops::Deref;

use gc::{ Finalize, Trace };

use crate::ast::structs::FunctionDeclaration;
use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::closure::Closure;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
use crate::interpreter::runtime_values::functions::types::{
	FunctionParameters,
	FunctionReturnValue,
};
use crate::interpreter::variables_containers::{ VariableScope, VariablesMap };
use crate::interpreter::variables_containers::map::IVariablesMapConstMembers;
use crate::interpreter::variables_containers::scope::IGenericVariablesScope;
use crate::semantic::captured_variables::analyze_captured_variables;
use crate::types::cell_ref::{ gc_clone, GcPtr };

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
			Function::Closure(cl) => f.write_str(cl.code.name.as_str()),
		}
	}
}

impl Function {
	pub fn new_closure(env: &Environment, decl: FunctionDeclaration) -> GcPtrToFunction {
		let captured_vars = analyze_captured_variables(&decl);
		let global_scope_ptr = gc_clone(&env.global_scope.borrow().scope);
		let mut captured_map = VariablesMap::new();

		for var in captured_vars {
			if let Some(val) = env.scope.get_actual(var.as_str().into()) {
				// Check if this variable comes from the global scope
				let var_scope = env.scope.resolve_variable_scope(var.as_str().into());
				if !GcPtr::ptr_eq(&var_scope, &global_scope_ptr.variables) {
					// It's not global, so we capture it
					captured_map.variables.insert(var.into(), val.into_owned());
				}
			}
		}

		let closure_scope = VariableScope::new_gc_from_map(captured_map, Some(global_scope_ptr));
		let closure = Closure::new(decl, closure_scope);
		let function_closure = Function::Closure(closure);
		return GcPtr::new(function_closure);
	}
}

impl IFunction for Function {
	#[inline(always)]
	fn execute(
		&self,
		env: &mut Environment,
		params: FunctionParameters
	) -> ResultWithError<FunctionReturnValue> {
		return match self {
			Function::Closure(cl) => cl.execute(env, params),
			Function::NativeFunction(f) => f.execute(env, params),
		};
	}
}
