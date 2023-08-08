use std::ops::{Deref, DerefMut};
use std::ops::{Add, Div, Mul, Rem, Sub};

use gc::GcCellRefMut;

use crate::ast::expression::{BoxExpression, Expression};
use crate::ast::operator::Operator;
use crate::ast::structs::CallExpression;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::{PrimitiveValue, ref_to_value::{DerefOfRefToValue, RefToValue}};
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::FunctionParameters;
use crate::interpreter::variables_map::IVariablesMap;

macro_rules! auto_implement_binary_operators {
    ($val: expr, $typ:ident, $a:ident, $b:ident, $($op_t: path, $oper: ident => $res_typ: ident);*;) => {
	    match $val {
	        $(($op_t, PrimitiveValue::$typ($a), PrimitiveValue::$typ($b)) => Some(PrimitiveValue::$res_typ($a.$oper($b))),)*
		    _ => None,
	    }
    };
}

fn try_left_borrow_mut<'a>(left: &'a mut RefToValue, right: &'a RefToValue) -> ResultWithError<(GcCellRefMut<'a, PrimitiveValue>, DerefOfRefToValue<'a>)> {
	let RefToValue::LValue(right_lval) = right else {
		if let RefToValue::RValue(v) = right {
			return Ok((left.try_borrow_mut()?, DerefOfRefToValue::DerefRValue(v)));
		}
		return Err(ErrorT::NeverError.into());
	};
	{
		let left_mut = left.try_borrow_mut()?;
		if let Ok(borrow) = right_lval.deref().try_borrow() {
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
	return Ok((left_mut, right_val));
}

impl Environment {
	pub fn eval(&mut self, expression: &Expression) -> ResultWithError<RefToValue> {
		return Ok(match expression {
			Expression::NullLiteral => PrimitiveValue::Null.into(),
			Expression::BooleanLiteral(a) => PrimitiveValue::Boolean(a.clone()).into(),
			Expression::IntegerLiteral(a) => PrimitiveValue::Integer(a.clone()).into(),
			Expression::StringLiteral(a) => PrimitiveValue::String(a.clone()).into(),
			Expression::UnaryExpression { operator, argument } => {
				self.execute_unary_operator_expression(operator, argument)?
			}
			Expression::BinaryExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::AssignmentExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::Identifier(name) => self.get_variable_or_null(name)?,
			Expression::FunctionCall(call_expr) => self.eval_function_call(call_expr)?,
			Expression::FunctionExpression(fdecl) => {
				let function: RefToValue = PrimitiveValue::new_closure(self, fdecl.clone()).into();
				self.assign_locally(&fdecl.name, function.clone());
				function
			}
			expr => {
				return Err(ErrorT::UnimplementedExpressionTypeForInterpreter(expr.clone()).into());
			}
		});
	}

	fn execute_unary_operator_expression(&mut self, operator: &Operator, argument: &Expression) -> ResultWithError<RefToValue> {
		let arg_eval = self.eval(argument)?;
		let prim_borrow = arg_eval.borrow();
		let prim_ref = prim_borrow.deref();
		return Ok(match (operator, prim_ref) {
			(Operator::LogicalNot, PrimitiveValue::Boolean(v)) => PrimitiveValue::Boolean(!*v).into(),
			(Operator::Plus, PrimitiveValue::Integer(v)) => PrimitiveValue::Integer(*v).into(),
			(Operator::Minus, PrimitiveValue::Integer(v)) => PrimitiveValue::Integer(-*v).into(),
			(op, _) => {
				return Err(ErrorT::UnimplementedUnaryOperatorForValues(op.clone(), argument.clone()).into());
			}
		});
	}

	fn eval_binary_operator_expression(
		&mut self,
		operator: &Operator,
		left: &BoxExpression,
		right: &BoxExpression,
	) -> ResultWithError<RefToValue> {
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
		return self.execute_binary_expression(operator, left_eval, right_eval, left, right);
	}

	pub fn execute_binary_expression(
		&mut self,
		operator: &Operator,
		mut left: RefToValue,
		right: RefToValue,
		left_expr: &Expression,
		right_expr: &Expression,
	) -> ResultWithError<RefToValue> {
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
							left_expr,
							right_expr,
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
		return Ok(self.execute_operation_on_primitive(
			operator,
			lder.deref(),
			rder.deref(),
			left_expr,
			right_expr,
		)?.into());
	}

	pub fn execute_operation_on_primitive(
		&mut self,
		operator: &Operator,
		left: &PrimitiveValue,
		right: &PrimitiveValue,
		left_expr: &Expression,
		right_expr: &Expression,
	) -> ResultWithError<PrimitiveValue> {
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
			(op, _l, _r) => {
				return Err(ErrorT::UnimplementedBinaryOperatorForValues(op.clone(), left_expr.clone(), right_expr.clone()).into());
			}
		};
	}

	pub fn eval_function_call(&mut self, call_expr: &CallExpression) -> ResultWithError<RefToValue> {
		return match call_expr.callee.deref() {
			Expression::Identifier(method_name) if method_name == "push_res_stack" => {
				let mut rvec = Vec::<PrimitiveValue>::new();
				for expr in call_expr.arguments.iter() {
					let expr_eval = self.eval(expr)?;
					rvec.push(expr_eval.consume_or_clone());
				}
				self.global_scope.borrow_mut().res_stack.extend(rvec.iter().map(|v| v.clone()));
				Ok(PrimitiveValue::Null.into())
			}
			expr => {
				let function = self.eval(expr)?.consume_or_clone();
				match function {
					PrimitiveValue::Function(ref gc_fn) => {
						let args = call_expr.arguments.iter().map(|v| self.eval(v).map(RefToValue::consume_or_clone)).collect::<ResultWithError<FunctionParameters>>()?;
						Ok(gc_fn.borrow().call(args)?.into())
					}
					_ => {
						Err(ErrorT::NotAFunction(expr.clone()).into())
					}
				}
			}
		};
	}
}
