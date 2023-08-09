use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use gc::{Finalize, Trace};

use crate::ast::expression::{Expression, IdentifierT};
use crate::ast::statement::{BoxStatement, Statement, StatementList};
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::environment::default_global_scope::get_default_global_scope;
use crate::interpreter::environment::statement_result::{handle_unrolling, handle_unrolling_in_loop, StatementExecution, StatementMetaGeneration, UnrollingReason};
use crate::interpreter::runtime_values::{PrimitiveValue, ref_to_value::RefToValue};
use crate::interpreter::variables_map::{delegate_ivariables_map, GlobalScope, IVariablesMap, IVariablesMapConstMembers, IVariablesMapDelegator, VariableScope, VariablesMap};
use crate::parser::parse;
use crate::utils::cell_ref::{gc_cell_clone, GcBox};

pub mod statement_result;
pub mod expression_evaluation;
pub mod native_functions;
pub mod default_global_scope;

#[derive(Trace, Finalize)]
pub struct Environment {
	pub scope: GcBox<VariableScope>,
	pub global_scope: GcBox<GlobalScope>,
}

delegate_ivariables_map!(for Environment =>
	&self: self.scope.borrow(),
	&self: (mut) self.scope.borrow_mut()
);

impl Environment {
	#[inline(always)]
	pub fn new_full(scope: GcBox<VariableScope>, global_scope: GcBox<GlobalScope>) -> Environment {
		return Self {
			scope,
			global_scope,
		};
	}

	#[inline(always)]
	pub fn new_child_of(scope: GcBox<VariableScope>, global_scope: GcBox<GlobalScope>) -> Environment {
		return Self {
			scope: VariableScope::new_gc(VariablesMap::new(), Some(scope)),
			global_scope,
		};
	}

	#[inline(always)]
	pub fn new() -> Environment {
		let global_scope = get_default_global_scope();
		let scope = gc_cell_clone(&global_scope.borrow().scope);
		return Self::new_full(scope, global_scope);
	}

	#[inline(always)]
	pub fn new_from_primitives(variables: HashMap<IdentifierT, PrimitiveValue>) -> Environment {
		let global_scope = get_default_global_scope();
		let scope = gc_cell_clone(&global_scope.borrow().scope);
		{
			let scope_borr = scope.borrow();
			let mut scope_vars_borr = scope_borr.variables.borrow_mut();
			for (name, value) in variables.into_iter() {
				scope_vars_borr.deref_mut().assign(&name, value.into());
			}
		}
		return Self::new_full(scope, global_scope);
	}

	pub fn new_with_parent(env: &Environment) -> Environment {
		let global_scope = gc_cell_clone(&env.global_scope);
		return Self::new_full(
			VariableScope::new_gc(
				VariablesMap::new(),
				Some(gc_cell_clone(&env.scope)),
			),
			global_scope,
		);
	}


	pub fn eval_program_string(&mut self, input: String) -> ResultWithError<StatementExecution> {
		self.setup_and_eval_statements(&parse(input)?)
	}

	pub fn setup_scope_for_statement(&mut self, statement: &Statement) -> ResultWithError<StatementMetaGeneration> {
		match statement {
			Statement::VariableDeclarations(decls) => {
				for decl in decls.iter() {
					self.hoist_identifier(&decl.identifier)?;
				}
			}
			Statement::FunctionDeclarationStatement(fdecl) => {
				self.declare(&fdecl.name, PrimitiveValue::new_closure(self, fdecl.clone()).into())?;
			}
			_ => {}
		}
		return Ok(StatementMetaGeneration::NormalGeneration);
	}

	pub fn setup_scope(&mut self, statements: &StatementList) -> ResultWithError<StatementMetaGeneration> {
		for statement in statements.iter() {
			self.setup_scope_for_statement(statement)?;
		}
		return Ok(StatementMetaGeneration::NormalGeneration);
	}

	pub fn setup_and_eval_statements(&mut self, statements: &StatementList) -> ResultWithError<StatementExecution> {
		self.setup_scope(statements)?;
		for statement in statements.iter() {
			handle_unrolling!(self.eval_statement(statement)?);
		}
		return Ok(StatementExecution::NormalFlow);
	}

	pub fn setup_and_eval_statement(&mut self, statement: &Statement) -> ResultWithError<StatementExecution> {
		self.setup_scope_for_statement(statement)?;
		return self.eval_statement(statement);
	}

	#[allow(non_snake_case)]
	pub fn eval_statement__creates_scope(&mut self, statement: &Statement) -> ResultWithError<StatementExecution> {
		return match statement {
			Statement::EmptyStatement => {
				Ok(StatementExecution::NormalFlow)
			}
			Statement::BlockStatement(statements) => {
				self.eval_block__creates_scope(statements)
			}
			Statement::ForLoop { initialization, condition, increment, body } => {
				self.eval_for_loop__creates_scope(initialization, condition, increment, body)
			}
			v => {
				let mut env = Environment::new_with_parent(self);
				env.setup_and_eval_statement(v)
			}
		};
	}

	pub fn eval_statement(&mut self, statement: &Statement) -> ResultWithError<StatementExecution> {
		return match statement {
			Statement::EmptyStatement => {
				Ok(StatementExecution::NormalFlow)
			}
			Statement::BlockStatement(statements) => {
				self.eval_block__creates_scope(statements)
			}
			Statement::IfStatement { condition, if_branch, else_branch } => {
				self.eval_if_statement(condition, if_branch, else_branch)
			}
			Statement::WhileLoop { condition, body } => {
				while self.eval(condition)?.is_truthy() {
					let v = self.eval_statement__creates_scope(body)?;
					handle_unrolling_in_loop!(v);
				}
				Ok(StatementExecution::NormalFlow)
			}
			Statement::DoWhileLoop { condition, body } => {
				loop {
					let v = self.eval_statement__creates_scope(body)?;
					handle_unrolling_in_loop!(v);
					if !self.eval(condition)?.is_truthy() {
						break;
					}
				}
				Ok(StatementExecution::NormalFlow)
			}
			Statement::ForLoop { initialization, condition, increment, body } => {
				self.eval_for_loop__creates_scope(initialization, condition, increment, body)
			}
			Statement::ExpressionStatement(expr) => {
				self.eval(expr)?;
				Ok(StatementExecution::NormalFlow)
			}
			Statement::VariableDeclarations(decls) => {
				for decl in decls.iter() {
					let value = if let Some(expr) = &decl.initializer {
						self.eval(expr)?
					} else {
						PrimitiveValue::Null.into()
					};
					self.declare(&decl.identifier, value)?;
				}
				Ok(StatementExecution::NormalFlow)
			}
			Statement::BreakStatement(v) => {
				Ok(StatementExecution::Unrolling(UnrollingReason::EncounteredBreak(*v)))
			}
			Statement::ContinueStatement(v) => {
				Ok(StatementExecution::Unrolling(UnrollingReason::EncounteredContinue(*v)))
			}
			Statement::FunctionDeclarationStatement(..) => {
				// Function declaration has already been hoisted
				Ok(StatementExecution::NormalFlow)
			}
			Statement::ReturnStatement(expr_opt) => {
				let res = if let Some(expr) = expr_opt.as_ref() {
					self.eval(expr)?.consume_or_clone()
				} else {
					PrimitiveValue::Null
				};
				Ok(StatementExecution::Unrolling(UnrollingReason::ReturningValue(res)))
			}
			stmt => {
				Err(ErrorT::UnimplementedStatementTypeForInterpreter(stmt.clone()).into())
			}
		};
	}

	#[allow(non_snake_case)]
	fn eval_block__creates_scope(&mut self, statements: &StatementList) -> ResultWithError<StatementExecution> {
		Environment::new_with_parent(self).setup_and_eval_statements(statements)
	}

	#[allow(non_snake_case)]
	fn eval_for_loop__creates_scope(&mut self, initialization: &BoxStatement, condition: &Expression, increment: &BoxStatement, body: &BoxStatement) -> ResultWithError<StatementExecution> {
		let mut env = Environment::new_with_parent(self);
		let init_eval_res = match &**initialization {
			Statement::BlockStatement(stmts) => {
				env.setup_and_eval_statements(stmts)?
			}
			init => env.setup_and_eval_statement(init)?
		};
		handle_unrolling!(init_eval_res);
		'for_simulator: while env.eval(condition)?.is_truthy() {
			'for_simulator_innermost: loop {
				let v = env.eval_statement__creates_scope(body)?;
				handle_unrolling_in_loop!(v =>
					break: break 'for_simulator;
					continue: break 'for_simulator_innermost;
				);
				break;
			}
			handle_unrolling_in_loop!(env.eval_statement__creates_scope(increment)? =>
				break: break 'for_simulator;
				continue: continue 'for_simulator;
			);
		}
		Ok(StatementExecution::NormalFlow)
	}

	fn eval_if_statement(&mut self, condition: &Expression, if_branch: &BoxStatement, else_branch: &Option<BoxStatement>) -> ResultWithError<StatementExecution> {
		return if self.eval(condition)?.is_truthy() {
			self.eval_statement__creates_scope(if_branch)
		} else {
			if let Some(else_branch_v) = else_branch {
				self.eval_statement__creates_scope(else_branch_v)
			} else {
				Ok(StatementExecution::NormalFlow)
			}
		};
	}

	#[inline]
	pub fn hoist_identifier(&mut self, iden: &IdentifierT) -> ResultWithError<StatementMetaGeneration> {
		self.scope.deref().borrow().hoist(iden)?;
		return Ok(StatementMetaGeneration::NormalGeneration);
	}

	#[inline]
	pub fn assign_locally(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue> {
		return self.scope.borrow().variables.borrow_mut().assign(name, value);
	}
}
