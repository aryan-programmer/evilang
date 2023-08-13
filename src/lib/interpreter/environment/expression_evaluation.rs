use std::ops::{Add, Div, Mul, Rem, Sub};
use std::ops::Deref;

use gc::GcCellRefMut;

use crate::ast::expression::{BoxExpression, Expression, MemberIndexer};
use crate::ast::operator::Operator;
use crate::ast::structs::CallExpression;
use crate::errors::{Descriptor, ErrorT, EvilangError, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::{GcBoxOfPrimitiveValueExt, PrimitiveValue, ref_to_value::{DerefOfRefToValue, RefToValue}};
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::FunctionParameters;
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::utils::consts::CONSTRUCTOR;
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::utils::cell_ref::{gc_cell_clone, GcBox};

macro_rules! auto_implement_binary_operators {
    ($val: expr, $typ:ident, $a:ident, $b:ident, $($op_t: path, $oper: ident => $res_typ: ident);*;) => {
	    match $val {
	        $(($op_t, PrimitiveValue::$typ($a), PrimitiveValue::$typ($b)) => Some(PrimitiveValue::$res_typ($a.$oper($b))),)*
		    _ => None,
	    }
    };
}

impl Environment {
	pub fn eval(&mut self, expression: &Expression) -> ResultWithError<RefToValue> {
		return Ok(match expression {
			Expression::NullLiteral => PrimitiveValue::Null.into(),
			Expression::BooleanLiteral(a) => PrimitiveValue::Boolean(a.clone()).into(),
			Expression::IntegerLiteral(a) => PrimitiveValue::Integer(a.clone()).into(),
			Expression::StringLiteral(a) => PrimitiveValue::String(a.clone()).into(),
			Expression::UnaryExpression { operator, argument } =>
				self.execute_unary_operator_expression(operator, argument)?,
			Expression::BinaryExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::AssignmentExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::Identifier(name) => {
				let var = self.get_actual(name).or_else(|| {
					self.assign_locally(name, PrimitiveValue::Null.into());
					self.get_actual(name)
				}).ok_or_else(|| EvilangError::new(ErrorT::NeverError("When a variable is not found it is set to null and then it is immediately retrieved, but it was still not found".into())))?;
				if var.is_hoisted() {
					return Err(ErrorT::CantAccessHoistedVariable(name.clone()).into());
				}
				RefToValue::LValue(var)
			}
			Expression::FunctionCall(call_expr) => self.eval_function_call(call_expr)?,
			Expression::FunctionExpression(fdecl) => {
				let function = PrimitiveValue::new_closure(self, fdecl.clone());
				self.assign_locally(&fdecl.name, function.clone());
				function.into()
			}
			Expression::ClassDeclarationExpression(cdecl) => {
				let class = PrimitiveValue::new_class_by_eval(self, cdecl)?;
				self.assign_locally(&cdecl.name, class.clone());
				class.into()
			}
			Expression::ParenthesizedExpression(expr) => self.eval(expr)?,
			Expression::MemberAccess { object, member } => {
				let name = match member {
					MemberIndexer::PropertyName(name) => name.clone(),
					MemberIndexer::SubscriptExpression(expr) => {
						let subscript = self.eval(expr)?;
						let subs_borr = subscript.borrow();
						match subs_borr.deref() {
							PrimitiveValue::String(str) => str.clone(),
							val => {
								return Err(RuntimeError::ExpectedValidSubscript(Descriptor::Both {
									value: val.clone(),
									expression: (**expr).clone(),
								}).into());
							}
						}
					}
					MemberIndexer::MethodNameArrow(_) => {
						return Err(ErrorT::InvalidMethodArrowAccess(expression.clone()).into());
					}
				};
				let object_val = self.eval_expr_expect_object(object)?;
				RefToValue::new_object_property_ref(object_val, name)
			}
			Expression::NewObjectExpression(call_expr) => self.eval_new_object_expression(call_expr)?,
			expr => {
				return Err(ErrorT::UnimplementedExpressionTypeForInterpreter(expr.clone()).into());
			}
		});
	}

	fn eval_expr_expect_object(&mut self, object: &Expression) -> ResultWithError<GcBox<RuntimeObject>> {
		let object_eval = self.eval(object)?;
		let obj_eval_borr = object_eval.borrow();
		return if let PrimitiveValue::Object(object_class_ref) = obj_eval_borr.deref() {
			Ok(gc_cell_clone(object_class_ref))
		} else {
			Err(RuntimeError::ExpectedClassObject(Descriptor::Both {
				value: obj_eval_borr.deref().clone(),
				expression: object.clone(),
			}).into())
		};
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
				left.set(right.consume_or_clone())?;
			} else {
				let val_to_assign = self.execute_operation_on_primitive(
					&operator.strip_assignment()?,
					left.borrow().deref(),
					right.borrow().deref(),
					left_expr,
					right_expr,
				)?;
				if val_to_assign.is_hoisted() {
					return Err(ErrorT::CantSetToHoistedValue.into());
				}
				left.set(val_to_assign)?;
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

	pub fn eval_new_object_expression(&mut self, call_expr: &CallExpression) -> ResultWithError<RefToValue> {
		let class = self.eval_expr_expect_object(&call_expr.callee)?;
		let obj = RuntimeObject::allocate_instance(class);
		let res = RuntimeObject::call_method_on_object_with_args(
			gc_cell_clone(&obj),
			self,
			&CONSTRUCTOR.to_string(),
			call_expr,
		)?;
		return Ok(match res {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => PrimitiveValue::Object(obj),
			rv => rv
		}.into());
	}

	//noinspection RsLift
	pub fn eval_function_call(&mut self, call_expr: &CallExpression) -> ResultWithError<RefToValue> {
		match call_expr.callee.deref() {
			Expression::MemberAccess {
				object,
				member: MemberIndexer::MethodNameArrow(method_name)
			} => {
				return Ok(RuntimeObject::call_method_on_object_with_args(
					self.eval_expr_expect_object(&object)?,
					self,
					method_name,
					call_expr,
				)?.into());
			}
			expr => {
				let function = self.eval(expr)?.consume_or_clone();
				let PrimitiveValue::Function(ref gc_fn) = function else {
					return Err(RuntimeError::ExpectedFunction(Descriptor::Both {
						value: function,
						expression: expr.clone(),
					}).into());
				};
				let args = call_expr
					.arguments
					.iter()
					.map(|v| self
						.eval(v)
						.map(RefToValue::consume_or_clone)
					)
					.collect::<ResultWithError<FunctionParameters>>()?;
				return Ok(gc_fn.borrow().execute(self, args)?.into());
			}
		};
	}
}
