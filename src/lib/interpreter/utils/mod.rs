use std::ops::Deref;

use crate::ast::expression::{Expression, IdentifierT};
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::utils::cell_ref::{gc_cell_clone, GcBox};
use crate::interpreter::utils::consts::OBJECT;
use crate::interpreter::variables_containers::map::IVariablesMapConstMembers;
use crate::interpreter::variables_containers::VariablesMap;

pub mod consts;
pub mod consume_or_clone;
pub mod cell_ref;

pub fn get_object_superclass(env: &mut Environment) -> ResultWithError<GcBox<RuntimeObject>> {
	let object_class_name = OBJECT.to_string();
	let Some(result) = env
		.global_scope
		.borrow()
		.get_actual(&object_class_name) else {
		return Err(RuntimeError::ExpectedClassObject(object_class_name.into()).into());
	};
	Ok(expect_object(
		RefToValue::LValue(result),
		Some(&Expression::Identifier(object_class_name)))?)
}

#[inline(always)]
pub fn expect_object(object: RefToValue, expr: Option<&Expression>) -> ResultWithError<GcBox<RuntimeObject>> {
	return expect_object_fn(object, || expr.map(Expression::clone));
}

pub fn expect_object_fn<T>(object: RefToValue, expr_fn: T) -> ResultWithError<GcBox<RuntimeObject>> where T: Fn() -> Option<Expression> {
	let obj_eval_borr = object.borrow();
	return if let PrimitiveValue::Object(object_class_ref) = obj_eval_borr.deref() {
		Ok(gc_cell_clone(object_class_ref))
	} else {
		Err(RuntimeError::ExpectedClassObject(match expr_fn() {
			None => Descriptor::Value(obj_eval_borr.deref().clone()),
			Some(expr) => Descriptor::Both {
				value: obj_eval_borr.deref().clone(),
				expression: expr,
			}
		}).into())
	};
}

pub fn expect_object_or_set_object_if_null<T>(
	env: &mut Environment,
	mut object: RefToValue,
	object_name: &IdentifierT,
	expr_fn: T,
) -> ResultWithError<GcBox<RuntimeObject>> where T: Fn() -> Option<Expression> {
	return Ok(if object.borrow().deref() == &PrimitiveValue::Null {
		let obj = RuntimeObject::new_gc(
			VariablesMap::new(),
			Some(get_object_superclass(env)?),
			object_name.to_string(),
		);
		object.set(PrimitiveValue::Object(gc_cell_clone(&obj)))?;
		obj
	} else {
		expect_object_fn(object, expr_fn)?
	});
}
