use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub};

use crate::ast::expression::{Expression, IdentifierT};
use crate::ast::operator::Operator;
use crate::ast::statement::{Statement, StatementList};
use crate::ast::structs::CallExpression;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_value::{DerefOfRefToValue, PrimitiveValue, rc_cell_from, RcCellValue, RefToValue};
use crate::parser::parse;

pub type EnvironmentExecutionResultType = ();

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
	variables: HashMap<IdentifierT, RcCellValue>,
	parent: Option<Box<Environment>>,
	pub res_stack: Vec<PrimitiveValue>,
}

macro_rules! auto_implement_binary_operators {
    ($val: expr, $typ:ident, $a:ident, $b:ident, $($op_t: path, $oper: ident);*;) => {
	    match $val {
	        $(($op_t, PrimitiveValue::$typ($a), PrimitiveValue::$typ($b)) => Some(PrimitiveValue::$typ($a.$oper($b)).into()),)*
		    _ => None,
	    }
    };
}

impl Environment {
	pub fn new_with_parent(parent: Option<Box<Environment>>) -> Self {
		Self { variables: HashMap::new(), parent, res_stack: Vec::new() }
	}

	pub fn new(variables: HashMap<IdentifierT, PrimitiveValue>, parent: Option<Box<Environment>>) -> Self {
		Self {
			variables: variables
				.into_iter()
				.map(|(iden, val)| (iden, rc_cell_from(val)))
				.collect(),
			parent,
			res_stack: Vec::new(),
		}
	}

	pub fn parse_and_run_program(&mut self, input: String) -> ResultWithError<EnvironmentExecutionResultType> {
		self.run_statements(&parse(input)?)
	}

	pub fn run_statements(&mut self, program: &StatementList) -> ResultWithError<EnvironmentExecutionResultType> {
		for statement in program.iter() {
			self.run_statement(statement)?;
		}
		return Ok(());
	}

	pub fn run_statement(&mut self, statement: &Statement) -> ResultWithError<EnvironmentExecutionResultType> {
		match statement {
			Statement::EmptyStatement => {
				// Empty intentionally
			}
			Statement::ExpressionStatement(expr) => {
				self.eval(expr)?;
			}
			Statement::VariableDeclarations(decls) => {
				for decl in decls.iter() {
					let value = if let Some(expr) = &decl.initializer {
						self.eval(expr)?
					} else {
						PrimitiveValue::Null.into()
					};
					self.assign_variable(decl.identifier.clone(), value);
				}
			}
			stmt => {
				return Err(ErrorT::UnimplementedStatementTypeForInterpreter(stmt.clone()).into());
			}
		}
		return Ok(());
	}

	pub fn eval(&mut self, expression: &Expression) -> ResultWithError<RefToValue> {
		return Ok(match expression {
			Expression::NullLiteral => PrimitiveValue::Null.into(),
			Expression::BooleanLiteral(a) => PrimitiveValue::Boolean(a.clone()).into(),
			Expression::IntegerLiteral(a) => PrimitiveValue::Integer(a.clone()).into(),
			Expression::StringLiteral(a) => PrimitiveValue::String(a.clone()).into(),
			// Expression::UnaryExpression { .. } => {}
			Expression::BinaryExpression { operator, left, right } => {
				let left_eval = self.eval(left.deref())?;
				let right_eval = self.eval(right.deref())?;
				self.eval_binary_expression(operator, left_eval, right_eval)?
			}
			Expression::AssignmentExpression { operator, left, right } => {
				let left_eval = self.eval(left.deref())?;
				let right_eval = self.eval(right.deref())?;
				self.eval_binary_expression(operator, left_eval, right_eval)?
			}
			Expression::Identifier(name) => self.get_variable_or_put_null(name),
			Expression::FunctionCall(call_expr) => self.eval_function_call(call_expr)?,
			expr => {
				return Err(ErrorT::UnimplementedExpressionTypeForInterpreter(expr.clone()).into());
			}
		});
	}

	pub fn eval_binary_expression(
		&mut self,
		operator: &Operator,
		mut left: RefToValue,
		right: RefToValue,
	) -> ResultWithError<RefToValue> {
		if operator.is_assignment() {
			if *operator == Operator::Assignment {
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
						let val_to_assign = self.eval_primitive_operation(
							&op.strip_assignment()?,
							left_mut_ref,
							right_value_ref,
						)?;
						*left_mut_ref = val_to_assign;
					}
				};
			}
			return Ok(left);
		}

		let lder = left.borrow();
		let rder = right.borrow();
		return Ok(self.eval_primitive_operation(operator, lder.deref(), rder.deref())?.into());
	}

	pub fn eval_primitive_operation(
		&mut self,
		operator: &Operator,
		left: &PrimitiveValue,
		right: &PrimitiveValue,
	) -> ResultWithError<PrimitiveValue> {
		let int_result: Option<PrimitiveValue> = auto_implement_binary_operators!(
			(operator, left, right),
			Integer, a, b,
			Operator::Plus, add;
			Operator::Minus, sub;
			Operator::Multiplication, mul;
			Operator::Division, div;
			Operator::Modulus, rem;
		);
		if let Some(int_r) = int_result {
			return Ok(int_r);
		}
		return match (operator, left, right) {
			(Operator::Plus, PrimitiveValue::String(a), PrimitiveValue::String(b)) =>
				Ok(PrimitiveValue::String(a.clone() + b)),
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
					rvec.push(self.eval(expr)?.consume_or_clone());
				}
				self.res_stack.extend(rvec.iter().map(|v| v.clone()));
				Ok(PrimitiveValue::Null.into())
			}
			expr => {
				Err(ErrorT::UnimplementedFunction(expr.clone()).into())
			}
		};
	}

	pub fn assign_variable(&mut self, name: IdentifierT, value: RefToValue) {
		self.variables.insert(name, rc_cell_from(value.consume_or_clone()));
	}

	pub fn get_variable(&self, name: &IdentifierT) -> Option<RefToValue> {
		return if let Some(v) = self.variables.get(name) {
			Some(RefToValue::from_rc(v))
		} else {
			None
		};
	}

	pub fn get_variable_or_put_null(&mut self, name: &IdentifierT) -> RefToValue {
		if let Some(v) = self.variables.get(name) {
			return RefToValue::from_rc(v);
		}
		let rv = RcCellValue::new(RefCell::new(PrimitiveValue::Null));
		let res = RefToValue::from_rc(&rv);
		self.variables.insert(name.clone(), rv);
		return res;
	}
}
