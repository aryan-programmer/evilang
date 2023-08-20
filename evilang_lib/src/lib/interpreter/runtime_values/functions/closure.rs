use std::fmt::{Debug, Formatter};

use gc::{Finalize, Trace};

use crate::ast::structs::{FunctionDeclaration, FunctionParameterDeclaration};
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::statement_result::{StatementExecution, UnrollingReason};
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::cell_ref::gc_clone;
use crate::interpreter::variables_containers::map::IVariablesMapDelegator;
use crate::interpreter::variables_containers::scope::GcPtrToVariableScope;

#[derive(Trace, Finalize)]
pub struct Closure {
	#[unsafe_ignore_trace]
	pub code: FunctionDeclaration,
	pub parent_scope: GcPtrToVariableScope,
}

impl PartialEq for Closure {
	fn eq(&self, _other: &Self) -> bool {
		false
	}
}

impl Debug for Closure {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.code.fmt(f)
	}
}

impl IFunction for Closure {
	fn execute(&self, this_env: &mut Environment, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
		let parent_env = Environment::new_raw(
			gc_clone(&self.parent_scope),
			gc_clone(&this_env.global_scope),
		);
		let mut env = Environment::new_with_parent(&parent_env);
		for (FunctionParameterDeclaration { identifier: param_name }, param_value) in self.code.parameters.iter().zip(params.into_iter()) {
			env.declare(param_name.into(), param_value.into())?;
		}
		let stmt_res = env.setup_and_eval_statement(&self.code.body)?;
		let result = match stmt_res {
			StatementExecution::NormalFlow => PrimitiveValue::Null,
			StatementExecution::Unrolling(UnrollingReason::ReturningValue(ret_val)) => ret_val,
			stmt_res => {
				return Err(ErrorT::InvalidUnrollingOfFunction(self.code.name.clone(), format!("{0:#?}", stmt_res)).into());
			}
		};
		return Ok(result);
	}
}

impl Closure {
	pub fn new(code: FunctionDeclaration, parent_scope: GcPtrToVariableScope) -> Self {
		Self { code, parent_scope }
	}
}
