use std::ops::Deref;

use crate::ast::expression::Expression;
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::native_items::classes::object::ObjectSuperclass;
use crate::interpreter::runtime_values::i_native_struct::INativeClass_GetClassCached;
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::variables_containers::VariablesMap;
use crate::types::cell_ref::gc_clone;
use crate::types::string::CowStringT;

#[inline(always)]
pub fn expect_object(object: RefToValue, expr: Option<&Expression>) -> ResultWithError<GcPtrToObject> {
	return Ok(gc_clone(expect_object_fn(object.borrow().deref(), || expr.map(|v| Descriptor::Expression(v.clone())).unwrap_or(Descriptor::None))?));
}

pub fn expect_object_fn<T>(object: &PrimitiveValue, desc_fn: T) -> ResultWithError<&GcPtrToObject> where T: Fn() -> Descriptor {
	return if let PrimitiveValue::Object(object_class_ref) = object {
		Ok(object_class_ref)
	} else {
		Err(RuntimeError::ExpectedClassObject(desc_fn().with_value(object.deref().deref().into())).into())
	};
}

pub fn expect_object_or_set_object_if_null<T>(
	env: &mut Environment,
	mut object: RefToValue,
	object_name: CowStringT,
	expr_fn: T,
) -> ResultWithError<GcPtrToObject> where T: Fn() -> Descriptor {
	return Ok(if object.borrow().deref() == &PrimitiveValue::Null {
		let obj = RuntimeObject::new_gc(
			VariablesMap::new(),
			Some(ObjectSuperclass::get_class_cached(env)?),
			object_name.into(),
		);
		object.set(PrimitiveValue::Object(gc_clone(&obj)))?;
		obj
	} else {
		gc_clone(expect_object_fn(object.borrow().deref(), expr_fn)?)
	});
}
