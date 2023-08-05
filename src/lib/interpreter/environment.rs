use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub};
use std::rc::Rc;

use delegate::delegate;

use crate::ast::expression::{BoxExpression, Expression, IdentifierT};
use crate::ast::operator::Operator;
use crate::ast::statement::{BoxStatement, Statement, StatementList};
use crate::ast::structs::CallExpression;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_value::{DerefOfRefToValue, PrimitiveValue, RefToValue};
use crate::interpreter::variables_map::{GlobalScope, IVariablesMap, VariableScope, VariablesMap};
use crate::parser::parse;
use crate::utils::cell_ref::RcCell;

macro_rules! auto_implement_binary_operators {
    ($val: expr, $typ:ident, $a:ident, $b:ident, $($op_t: path, $oper: ident => $res_typ: ident);*;) => {
	    match $val {
	        $(($op_t, PrimitiveValue::$typ($a), PrimitiveValue::$typ($b)) => Some(PrimitiveValue::$res_typ($a.$oper($b))),)*
		    _ => None,
	    }
    };
}

macro_rules! handle_unrolling {
	($val: expr) => {
		if let StatementExecution::Unrolling(imm_exit) = $val {
			return Ok(StatementExecution::Unrolling(imm_exit));
		}
	}
}

macro_rules! handle_unrolling_in_loop {
	($val: expr) => {
		handle_unrolling_in_loop!($val => break: break; continue: continue;)
	};
	($val: expr => break: $break_stmt: stmt; continue: $continue_stmt: stmt;) => {
		if let StatementExecution::Unrolling(imm_exit) = $val {
			match imm_exit {
				UnrollingReason::EncounteredBreak(v) => {
					if v <= 1 {
						$break_stmt
					}
					return Ok(StatementExecution::Unrolling(UnrollingReason::EncounteredBreak(v-1)));
				},
				UnrollingReason::EncounteredContinue(v) => {
					if v <= 1 {
						$continue_stmt
					}
					return Ok(StatementExecution::Unrolling(UnrollingReason::EncounteredContinue(v-1)));
				}
			};
		}
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum UnrollingReason {
	EncounteredBreak(i64),
	EncounteredContinue(i64),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StatementExecution {
	NormalFlow,
	Unrolling(UnrollingReason),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StatementMetaGeneration {
	NormalGeneration,
}

fn try_left_borrow_mut<'a>(left: &'a mut RefToValue, right: &'a RefToValue) -> ResultWithError<(RefMut<'a, PrimitiveValue>, DerefOfRefToValue<'a>)> {
	let RefToValue::LValue(right_lval) = right else {
		if let RefToValue::RValue(v) = right {
			// dbg!(("RValue", &left, &v));
			return Ok((left.try_borrow_mut()?, DerefOfRefToValue::DerefRValue(v)));
		}
		return Err(ErrorT::NeverError.into());
	};
	{
		let left_mut = left.try_borrow_mut()?;
		if let Ok(borrow) = right_lval.deref().try_borrow() {
			// dbg!(("Left right unrelated", &left_mut, &borrow));
			return Ok((left_mut, DerefOfRefToValue::DerefLValue(borrow)));
		}
	}
	let right_val = DerefOfRefToValue::Value(
		if let Ok(borrow) = right_lval.deref().try_borrow() {
			borrow.clone()
		} else {
			return Err(ErrorT::InvalidBorrow.into());
		}
	);
	let left_mut = left.try_borrow_mut()?;
	// dbg!(("Cloned right", &left_mut, &right_val));
	return Ok((left_mut, right_val));
}

pub struct Environment {
	scope: RcCell<VariableScope>,
	pub global_scope: Rc<RefCell<GlobalScope>>,
}

impl IVariablesMap for Environment {
	delegate! {
		to self.scope.borrow_mut() {
			fn set_actual(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue>;
			fn declare(&mut self, name: &IdentifierT, value: RefToValue) -> ResultWithError<()>;
			fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()>;
		}
		to self.scope.borrow() {
			fn get_actual(&self, name: &IdentifierT) -> Option<RefToValue>;
			fn contains_key(&self, name: &IdentifierT) -> bool;
		}
	}
}

impl Environment {
	pub fn new() -> Environment {
		return Self::new_from_variables(VariablesMap::new());
	}

	pub fn new_from_primitives(variables: HashMap<IdentifierT, PrimitiveValue>) -> Environment {
		return Self::new_from_variables(VariablesMap::new_from_primitives(variables));
	}

	pub fn new_from_variables(variables: VariablesMap) -> Environment {
		let global_scope = GlobalScope::new_rc_from_variables(variables);
		let rc = Rc::clone(&(global_scope.borrow_mut().scope));
		return Environment {
			scope: rc,
			global_scope,
		};
	}

	pub fn new_with_parent(env: &Environment) -> Environment {
		let global_scope = Rc::clone(&env.global_scope);
		return Environment {
			scope: VariableScope::new_rc(
				VariablesMap::new(),
				Some(Rc::clone(&env.scope)),
			),
			global_scope,
		};
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

	pub fn eval(&mut self, expression: &Expression) -> ResultWithError<RefToValue> {
		return Ok(match expression {
			Expression::NullLiteral => PrimitiveValue::Null.into(),
			Expression::BooleanLiteral(a) => PrimitiveValue::Boolean(a.clone()).into(),
			Expression::IntegerLiteral(a) => PrimitiveValue::Integer(a.clone()).into(),
			Expression::StringLiteral(a) => PrimitiveValue::String(a.clone()).into(),
			Expression::UnaryExpression { operator, argument } => {
				let arg_eval = self.eval(argument)?;
				self.execute_unary_operator_expression(operator, arg_eval)?
			}
			Expression::BinaryExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::AssignmentExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::Identifier(name) => self.get_variable_or_null(name)?,
			Expression::FunctionCall(call_expr) => self.eval_function_call(call_expr)?,
			expr => {
				return Err(ErrorT::UnimplementedExpressionTypeForInterpreter(expr.clone()).into());
			}
		});
	}

	fn execute_unary_operator_expression(&mut self, operator: &Operator, argument: RefToValue) -> ResultWithError<RefToValue> {
		let prim_borrow = argument.borrow();
		let prim_ref = prim_borrow.deref();
		return Ok(match (operator, prim_ref) {
			(Operator::LogicalNot, PrimitiveValue::Boolean(v)) => PrimitiveValue::Boolean(!*v).into(),
			(Operator::Plus, PrimitiveValue::Integer(v)) => PrimitiveValue::Integer(*v).into(),
			(Operator::Minus, PrimitiveValue::Integer(v)) => PrimitiveValue::Integer(-*v).into(),
			(op, prim) => {
				return Err(ErrorT::UnimplementedUnaryOperatorForValues(op.clone(), prim.clone()).into());
			}
		});
	}

	fn eval_binary_operator_expression(&mut self, operator: &Operator, left: &BoxExpression, right: &BoxExpression) -> ResultWithError<RefToValue> {
		let left_eval = self.eval(left.deref())?;
		match operator {
			Operator::LogicalOr => {
				return if left_eval.is_truthy() {
					Ok(left_eval)
				} else {
					let right_eval = self.eval(right.deref())?;
					Ok(right_eval)
				};
			}
			Operator::LogicalAnd => {
				return if !left_eval.is_truthy() {
					Ok(left_eval)
				} else {
					let right_eval = self.eval(right.deref())?;
					Ok(right_eval)
				};
			}
			_ => {}
		};

		let right_eval = self.eval(right.deref())?;
		self.execute_binary_expression(operator, left_eval, right_eval)
	}

	pub fn execute_binary_expression(&mut self, operator: &Operator, mut left: RefToValue, right: RefToValue) -> ResultWithError<RefToValue> {
		if operator.is_assignment() {
			if *operator == Operator::Assignment {
				if right.is_hoisted() {
					return Err(ErrorT::CantSetToHoistedValue.into());
				}
				*left.try_borrow_mut()? = right.consume_or_clone();
			} else {
				let (mut left_borrow, right_value) = try_left_borrow_mut(&mut left, &right)?;
				match (operator, left_borrow.deref_mut(), right_value.deref()) {
					// region ...Integer Assignment Operators
					(
						Operator::PlusAssignment,
						PrimitiveValue::Integer(a),
						PrimitiveValue::Integer(b),
					) => *a += b,
					(
						Operator::MinusAssignment,
						PrimitiveValue::Integer(a),
						PrimitiveValue::Integer(b),
					) => *a -= b,
					(
						Operator::MultiplicationAssignment,
						PrimitiveValue::Integer(a),
						PrimitiveValue::Integer(b),
					) => *a *= b,
					(
						Operator::DivisionAssignment,
						PrimitiveValue::Integer(a),
						PrimitiveValue::Integer(b),
					) => *a /= b,
					(
						Operator::ModulusAssignment,
						PrimitiveValue::Integer(a),
						PrimitiveValue::Integer(b),
					) => *a %= b,
					// endregion Integer Assignment Operators
					(
						Operator::PlusAssignment,
						PrimitiveValue::String(a),
						PrimitiveValue::String(b)
					) => *a += b,
					(op, left_mut_ref, right_value_ref) => {
						let val_to_assign = self.execute_operation_on_primitive(
							&op.strip_assignment()?,
							left_mut_ref,
							right_value_ref,
						)?;
						if val_to_assign.is_hoisted() {
							return Err(ErrorT::CantSetToHoistedValue.into());
						}
						*left_mut_ref = val_to_assign;
					}
				};
			}
			return Ok(left);
		}

		let lder = left.borrow();
		let rder = right.borrow();
		return Ok(self.execute_operation_on_primitive(operator, lder.deref(), rder.deref())?.into());
	}

	pub fn execute_operation_on_primitive(&mut self, operator: &Operator, left: &PrimitiveValue, right: &PrimitiveValue) -> ResultWithError<PrimitiveValue> {
		let int_result: Option<PrimitiveValue> = auto_implement_binary_operators!(
			(operator, left, right),
			Integer, a, b,
			Operator::Plus, add => Integer;
			Operator::Minus, sub => Integer;
			Operator::Multiplication, mul => Integer;
			Operator::Division, div => Integer;
			Operator::Modulus, rem => Integer;
			Operator::LessThan, lt => Boolean;
			Operator::GreaterThan, gt => Boolean;
			Operator::LessThanOrEqualTo, le => Boolean;
			Operator::GreaterThanOrEqualTo, ge => Boolean;
			// Operator::Equals, eq => Boolean;
			// Operator::NotEquals, ne => Boolean;
		);
		if let Some(int_r) = int_result {
			return Ok(int_r);
		}
		return match (operator, left, right) {
			(Operator::Plus, PrimitiveValue::String(a), PrimitiveValue::String(b)) =>
				Ok(PrimitiveValue::String(a.clone() + b)),
			(Operator::Equals, a, b) =>
				Ok(PrimitiveValue::Boolean(a == b)),
			(Operator::NotEquals, a, b) =>
				Ok(PrimitiveValue::Boolean(a != b)),
			(op, l, r) => {
				return Err(ErrorT::UnimplementedBinaryOperatorForValues(op.clone(), l.clone(), r.clone()).into());
			}
		};
	}

	pub fn eval_function_call(&mut self, call_expr: &CallExpression) -> ResultWithError<RefToValue> {
		return match call_expr.callee.deref() {
			Expression::Identifier(method_name) if method_name == "push_res_stack" => {
				let mut rvec = Vec::<PrimitiveValue>::new();
				for expr in call_expr.arguments.iter() {
					let expr_eval = self.eval(expr)?;
					// dbg!(&expr_eval);
					rvec.push(expr_eval.consume_or_clone());
				}
				self.global_scope.borrow_mut().res_stack.extend(rvec.iter().map(|v| v.clone()));
				Ok(PrimitiveValue::Null.into())
			}
			expr => {
				Err(ErrorT::UnimplementedFunction(expr.clone()).into())
			}
		};
	}

	pub fn hoist_identifier(&mut self, iden: &IdentifierT) -> ResultWithError<StatementMetaGeneration> {
		self.scope.deref().borrow_mut().deref_mut().hoist(iden)?;
		return Ok(StatementMetaGeneration::NormalGeneration);
	}
}
