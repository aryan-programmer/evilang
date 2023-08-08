use std::fmt::{Debug, Formatter};

use gc::{Finalize, Trace};

use crate::ast::structs::{FunctionDeclaration, FunctionParameterDeclaration};
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::statement_result::{StatementExecution, UnrollingReason};
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::variables_map::{GlobalScope, IVariablesMap, VariableScope};
use crate::utils::cell_ref::{gc_cell_clone, GcBox};

#[derive(PartialEq, Trace, Finalize)]
pub struct Closure {
	#[unsafe_ignore_trace]
	pub code: FunctionDeclaration,
	pub parent_scope: GcBox<VariableScope>,
	pub global_scope: GcBox<GlobalScope>,
}

impl Debug for Closure {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.code.fmt(f)
	}
}

impl IFunction for Closure {
	fn call(&self, params: FunctionParameters) -> ResultWithError<FunctionReturnValue> {
		let mut env = Environment::new_child_of(
			gc_cell_clone(&self.parent_scope),
			gc_cell_clone(&self.global_scope),
		);
		for (FunctionParameterDeclaration { identifier: param_name }, param_value) in self.code.parameters.iter().zip(params.into_iter()) {
			env.declare(param_name, param_value.into())?;
		}
		let stmt_res = env.setup_and_eval_statement(&self.code.body)?;
		let result = match stmt_res {
			StatementExecution::NormalFlow => PrimitiveValue::Null,
			StatementExecution::Unrolling(UnrollingReason::ReturningValue(ret_val)) => ret_val,
			stmt_res => {
				return Err(ErrorT::InvalidUnrollingOfFunction(self.code.name.clone(), stmt_res).into());
			}
		};
		return Ok(result);
	}
}

impl Closure {
	pub fn new(code: FunctionDeclaration, parent_scope: GcBox<VariableScope>, global_scope: GcBox<GlobalScope>) -> Self {
		Self { code, parent_scope, global_scope }
	}
}
