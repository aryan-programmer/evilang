use std::collections::HashMap;
use std::ops::DerefMut;

use gc::{Finalize, Trace};
use itertools::{Either::Left, Either::Right};

use crate::ast::expression::{Expression, IdentifierT};
use crate::ast::statement::{BoxStatement, Statement, StatementList};
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::default_global_scope::{get_default_global_scope, setup_environment};
use crate::interpreter::environment::resolver::{BoxIResolver, DefaultResolver};
use crate::interpreter::environment::statement_result::{handle_unrolling, handle_unrolling_in_loop, StatementExecution, StatementMetaGeneration, UnrollingReason};
use crate::interpreter::runtime_values::{GcPtrVariable, PrimitiveValue};
use crate::interpreter::runtime_values::functions::Function;
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::variables_containers::{GcPtrMutCellToGlobalScope, map::{delegate_ivariables_map, IVariablesMap, IVariablesMapConstMembers, IVariablesMapDelegator}, VariableScope, VariablesMap};
use crate::interpreter::variables_containers::scope::GcPtrToVariableScope;
use crate::parser::parse;
use crate::types::cell_ref::gc_clone;
use crate::types::consts::CURRENT_FILE;
use crate::types::string::{CowStringT, StringT};
use crate::types::traits::ConsumeOrCloneOf;

pub mod statement_result;
pub mod expression_evaluation;
pub mod native_items;
pub mod default_global_scope;
pub mod resolver;

#[derive(Clone, Trace, Finalize)]
pub struct Environment {
	pub scope: GcPtrToVariableScope,
	pub global_scope: GcPtrMutCellToGlobalScope,
}

delegate_ivariables_map!(for Environment =>
	&self: self.scope,
	&self: (mut) self.scope
);

impl Environment {
	#[inline(always)]
	pub(crate) fn new_raw(scope: GcPtrToVariableScope, global_scope: GcPtrMutCellToGlobalScope) -> Self {
		Self { scope, global_scope }
	}

	#[inline(always)]
	pub fn new() -> ResultWithError<Environment> {
		return Self::new_with_resolver(DefaultResolver::new_box());
	}

	pub fn new_with_resolver(resolver: BoxIResolver) -> ResultWithError<Environment> {
		let global_scope = get_default_global_scope(resolver);
		let scope = gc_clone(&global_scope.borrow().scope);
		let mut v = Self { scope, global_scope };
		setup_environment(&mut v)?;
		return Ok(v);
	}

	pub fn new_from_primitives(
		variables: HashMap<IdentifierT, PrimitiveValue>,
		resolver: BoxIResolver,
	) -> ResultWithError<Environment> {
		let global_scope = get_default_global_scope(resolver);
		let scope = gc_clone(&global_scope.borrow().scope);
		{
			let mut scope_vars_borr = scope.variables.borrow_mut();
			for (name, value) in variables.into_iter() {
				scope_vars_borr.deref_mut().assign(name.into(), value);
			}
		}
		return Ok(Self { scope, global_scope });
	}

	pub fn new_with_parent(env: &Environment) -> ResultWithError<Environment> {
		let global_scope = gc_clone(&env.global_scope);
		return Ok(Self {
			scope: VariableScope::new_gc_from_map(
				VariablesMap::new(),
				Some(gc_clone(&env.scope)),
			),
			global_scope,
		});
	}

	pub fn new_with_object_scope(env: &Environment, obj: &GcPtrToObject) -> ResultWithError<Environment> {
		let global_scope = gc_clone(&env.global_scope);
		return Ok(Self {
			scope: VariableScope::new_gc(
				gc_clone(&obj.properties),
				Some(gc_clone(&env.scope)),
			),
			global_scope,
		});
	}

	pub fn execute_file(file: StringT, resolver: BoxIResolver) -> ResultWithError<Environment> {
		let resolved_res = resolver.resolve(None, file)?;
		let mut env = Self::new_with_resolver(resolver)?;
		env.scope.assign_locally(CURRENT_FILE.into(), PrimitiveValue::String(resolved_res.absolute_file_path));
		env.setup_and_eval_statements(&resolved_res.statements)?;
		return Ok(env);
	}

	fn import_file(
		&self,
		namespace_object: GcPtrToObject,
		file_path: StringT,
	) -> ResultWithError<StatementExecution> {
		let resolved_res = self.global_scope.borrow().resolver.resolve(Some(self), file_path)?;
		let mut env = Environment::new_with_object_scope(self, &namespace_object)?;
		env.scope.assign_locally(CURRENT_FILE.into(), PrimitiveValue::String(resolved_res.absolute_file_path));
		env.setup_and_eval_statements(&resolved_res.statements)
	}

	pub fn eval_program_string(&mut self, input: StringT) -> ResultWithError<StatementExecution> {
		self.setup_and_eval_statements(&parse(input)?)
	}

	pub fn setup_scope_for_statement(&mut self, statement: &Statement) -> ResultWithError<StatementMetaGeneration> {
		match statement {
			Statement::VariableDeclarations(decls) => {
				for decl in decls.iter() {
					self.hoist_identifier((&decl.identifier).into())?;
				}
			}
			Statement::FunctionDeclarationStatement(fdecl) => {
				self.declare(
					(&fdecl.name).into(),
					Function::new_closure(self, fdecl.clone()).into(),
				)?;
			}
			Statement::ClassDeclarationStatement(cdecl) => {
				let class = RuntimeObject::new_class_decl(self, cdecl)?;
				self.declare(
					(&cdecl.name).into(),
					class.into(),
				)?;
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
				let mut env = Environment::new_with_parent(self)?;
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
						self.eval(expr)?.consume_or_clone()?
					} else {
						PrimitiveValue::Null
					};
					self.declare((&decl.identifier).into(), value)?;
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
					self.eval(expr)?.consume_or_clone()?
				} else {
					PrimitiveValue::Null
				};
				Ok(StatementExecution::Unrolling(UnrollingReason::ReturningValue(res)))
			}
			Statement::ClassDeclarationStatement(..) => {
				// Class declaration has already been hoisted
				Ok(StatementExecution::NormalFlow)
			}
			Statement::NamespaceStatement { namespace, body } => {
				let obj = self.get_namespace_object(namespace)?;
				let mut env = Environment::new_with_object_scope(self, &obj)?;
				env.setup_and_eval_statements(body)
			}
			Statement::ImportStatement {
				as_object,
				file_name,
			} => {
				let obj = self.get_namespace_object(as_object)?;
				let file = match self.eval(file_name)?.consume_or_clone()?.consume_as_string() {
					Left(str) => str,
					Right(val) => {
						return Err(RuntimeError::ExpectedValidFileName(Descriptor::new_both(val.into(), file_name.into())).into());
					}
				};
				self.import_file(obj, file)
			}
			/*stmt => {
				Err(ErrorT::UnimplementedStatementTypeForInterpreter(stmt.clone()).into())
			}*/
		};
	}

	#[allow(non_snake_case)]
	fn eval_block__creates_scope(&mut self, statements: &StatementList) -> ResultWithError<StatementExecution> {
		Environment::new_with_parent(self)?.setup_and_eval_statements(statements)
	}

	#[allow(non_snake_case)]
	fn eval_for_loop__creates_scope(&mut self, initialization: &BoxStatement, condition: &Expression, increment: &BoxStatement, body: &BoxStatement) -> ResultWithError<StatementExecution> {
		let mut env = Environment::new_with_parent(self)?;
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
	pub fn hoist_identifier(&mut self, iden: CowStringT) -> ResultWithError<StatementMetaGeneration> {
		self.scope.hoist(iden)?;
		return Ok(StatementMetaGeneration::NormalGeneration);
	}
}
