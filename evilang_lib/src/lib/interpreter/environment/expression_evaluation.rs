use std::ops::{Add, Div, Mul, Rem, Sub};
use std::ops::Deref;

use crate::ast::expression::{BoxExpression, DottedIdentifiers, Expression, MemberIndexer};
use crate::ast::operator::Operator;
use crate::ast::structs::CallExpression;
use crate::errors::{Descriptor, ErrorT, EvilangError, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::{GcPtrVariableExt, PrimitiveValue, ref_to_value::RefToValue};
use crate::interpreter::runtime_values::functions::Function;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::FunctionParameters;
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::utils::{expect_object, expect_object_or_set_object_if_null};
use crate::interpreter::utils::cell_ref::gc_clone;
use crate::interpreter::utils::consts::CONSTRUCTOR;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::types::traits::ConsumeOrCloneOf;
use crate::types::string::CowStringT;

macro_rules! by_ref {
    ($b:ident) => {($b)};
}

macro_rules! by_clone {
    ($b:ident) => {($b.clone())};
}

macro_rules! auto_implement_binary_operators {
    ($val: expr, $typ:ident, $a:ident, $b:ident, $($op_t: path, $oper: ident => $res_typ: ident ($by_what: tt));*;) => {
	    match $val {
	        $(($op_t, PrimitiveValue::$typ($a), PrimitiveValue::$typ($b)) =>
	            Some(PrimitiveValue::$res_typ($a.$oper($by_what!($b)))),)*
		    _ => None,
	    }
    };
	(by_ref $b:ident) => {
		($b)
	};
	(by_copy $b:ident) => {
		($b.clone())
	};
}

impl Environment {
	pub fn eval(&mut self, expression: &Expression) -> ResultWithError<RefToValue> {
		return Ok(match expression {
			Expression::NullLiteral => PrimitiveValue::Null.into(),
			Expression::BooleanLiteral(a) => PrimitiveValue::Boolean(a.clone()).into(),
			Expression::NumericLiteral(a) => PrimitiveValue::Number(a.clone()).into(),
			Expression::StringLiteral(a) => PrimitiveValue::String(a.clone()).into(),
			Expression::UnaryExpression { operator, argument } =>
				self.execute_unary_operator_expression(operator, argument)?,
			Expression::BinaryExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::AssignmentExpression { operator, left, right } =>
				self.eval_binary_operator_expression(operator, left, right)?,
			Expression::Identifier(name) => self.get_identifier(name.into())?,
			Expression::FunctionCall(call_expr) => self.eval_function_call(call_expr)?,
			Expression::FunctionExpression(fdecl) => {
				let function = Function::new_closure(self, fdecl.clone());
				self.assign_locally((&fdecl.name).into(), gc_clone(&function).into());
				RefToValue::Value(function.into())
			}
			Expression::ClassDeclarationExpression(cdecl) => {
				let class = RuntimeObject::new_class_decl(self, cdecl)?;
				self.assign_locally((&cdecl.name).into(), gc_clone(&class).into());
				RefToValue::Value(class.into())
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
								return Err(RuntimeError::ExpectedValidSubscript(Descriptor::new_both(val, expr)).into());
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
			Expression::DottedIdentifiers(idens) => self.get_dotted_identifiers(expression, idens)?,
			Expression::NewObjectExpression(call_expr) => self.eval_new_object_expression(call_expr)?,
			/*
			expr => {
				return Err(ErrorT::UnimplementedExpressionTypeForInterpreter(expr.clone()).into());
			}
			*/
		});
	}

	pub fn get_namespace_object(&mut self, idens: &DottedIdentifiers) -> ResultWithError<GcPtrToObject> {
		let mut iter = idens.iter();
		let Some(obj_expr) = iter.next() else {
			return Err(RuntimeError::ExpectedNamespaceObject(Descriptor::Expression(Expression::DottedIdentifiers(idens.clone()))).into());
		};
		let f = || Some(Expression::DottedIdentifiers(idens.clone()));
		let mut res_ref = self.get_identifier(obj_expr.into())?;
		let mut res_ref_name = obj_expr;
		while let Some(next_name) = iter.next() {
			let obj = expect_object_or_set_object_if_null(
				self,
				res_ref,
				res_ref_name.into(),
				f,
			)?;
			res_ref = RefToValue::new_object_property_ref(
				obj,
				next_name.clone(),
			);
			res_ref_name = next_name;
		};
		let ret_obj = expect_object_or_set_object_if_null(
			self,
			res_ref,
			res_ref_name.into(),
			f,
		)?;
		Ok(ret_obj)
	}

	fn get_dotted_identifiers(&mut self, expression: &Expression, idens: &DottedIdentifiers) -> ResultWithError<RefToValue> {
		let mut iter = idens.iter();
		let Some(obj_expr) = iter.next()else {
			return Ok(PrimitiveValue::Null.into());
		};
		let mut res = self.get_identifier(obj_expr.into())?;
		while let Some(next_name) = iter.next() {
			let obj = expect_object(res, Some(expression))?;
			res = RefToValue::new_object_property_ref(obj, next_name.clone());
		};
		Ok(res)
	}

	fn get_identifier(&mut self, name: CowStringT) -> ResultWithError<RefToValue> {
		let name_ref = name.deref();
		let var = self.get_actual(name_ref.into()).or_else(|| {
			self.assign_locally(name_ref.into(), PrimitiveValue::Null.into());
			self.get_actual(name_ref.into())
		}).ok_or_else(|| EvilangError::new(ErrorT::NeverError("When a variable is not found it is set to null and then it is immediately retrieved, but it was still not found".into())))?;
		if var.is_hoisted() {
			return Err(ErrorT::CantAccessHoistedVariable(name.into()).into());
		}
		Ok(RefToValue::Variable(var))
	}

	fn eval_expr_expect_object(&mut self, expr: &Expression) -> ResultWithError<GcPtrToObject> {
		return expect_object(self.eval(expr)?, Some(expr));
	}

	fn execute_unary_operator_expression(&mut self, operator: &Operator, argument: &Expression) -> ResultWithError<RefToValue> {
		let arg_eval = self.eval(argument)?;
		let prim_borrow = arg_eval.borrow();
		let prim_ref = prim_borrow.deref();
		return Ok(match (operator, prim_ref) {
			(Operator::LogicalNot, PrimitiveValue::Boolean(v)) => PrimitiveValue::Boolean(!*v).into(),
			(Operator::Plus, PrimitiveValue::Number(v)) => PrimitiveValue::Number(v.clone()).into(),
			(Operator::Minus, PrimitiveValue::Number(v)) => PrimitiveValue::Number(-v.clone()).into(),
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

	fn execute_binary_expression(
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
				left.set(right.consume_or_clone()?)?;
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

	fn execute_operation_on_primitive(
		&mut self,
		operator: &Operator,
		left: &PrimitiveValue,
		right: &PrimitiveValue,
		left_expr: &Expression,
		right_expr: &Expression,
	) -> ResultWithError<PrimitiveValue> {
		let int_result: Option<PrimitiveValue> = auto_implement_binary_operators!(
			(operator, left, right),
			Number, a, b,
			Operator::Plus, add => Number (by_clone);
			Operator::Minus, sub => Number (by_clone);
			Operator::Multiplication, mul => Number (by_clone);
			Operator::Division, div => Number (by_clone);
			Operator::Modulus, rem => Number (by_clone);
			Operator::LessThan, lt => Boolean (by_ref);
			Operator::GreaterThan, gt => Boolean (by_ref);
			Operator::LessThanOrEqualTo, le => Boolean (by_ref);
			Operator::GreaterThanOrEqualTo, ge => Boolean (by_ref);
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

	fn eval_new_object_expression(&mut self, call_expr: &CallExpression) -> ResultWithError<RefToValue> {
		let class = self.eval_expr_expect_object(&call_expr.callee)?;
		let obj = RuntimeObject::allocate_instance(class);
		let res = RuntimeObject::call_method_on_object_with_args(
			gc_clone(&obj),
			self,
			CONSTRUCTOR.into(),
			call_expr,
		)?;
		return Ok(match res {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => PrimitiveValue::Object(obj),
			rv => rv
		}.into());
	}

	//noinspection RsLift
	fn eval_function_call(&mut self, call_expr: &CallExpression) -> ResultWithError<RefToValue> {
		match call_expr.callee.deref() {
			Expression::MemberAccess {
				object,
				member: MemberIndexer::MethodNameArrow(method_name)
			} => {
				return Ok(RuntimeObject::call_method_on_object_with_args(
					self.eval_expr_expect_object(&object)?,
					self,
					method_name.into(),
					call_expr,
				)?.into());
			}
			expr => {
				let function = self.eval(expr)?.consume_or_clone()?;
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
						.and_then(RefToValue::consume_or_clone)
					)
					.collect::<ResultWithError<FunctionParameters>>()?;
				return Ok(gc_fn.execute(self, args)?.into());
			}
		};
	}
}
