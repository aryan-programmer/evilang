use std::mem::replace;
use std::ops::{Deref, DerefMut};

use delegate::delegate;

use crate::ast::expression::IdentifierT;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::runtime_values::PrimitiveValue;
pub use crate::interpreter::runtime_values::ref_to_value::deref_of_ref_to_value::DerefOfRefToValue;
use crate::interpreter::utils::cell_ref::{gc_box_from, GcBox};
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator};

pub mod deref_of_ref_to_value;

#[derive(Debug, PartialEq)]
pub enum RefToValue {
	RValue(PrimitiveValue),
	LValue(GcBox<PrimitiveValue>),
	ObjectProperty {
		object: GcBox<RuntimeObject>,
		property_name: IdentifierT,
		snapshot: Option<GcBox<PrimitiveValue>>,
	},
}

impl From<PrimitiveValue> for RefToValue {
	#[inline(always)]
	fn from(value: PrimitiveValue) -> Self {
		return RefToValue::RValue(value);
	}
}

impl ConsumeOrCloneOf for RefToValue {
	type Target = PrimitiveValue;

	#[inline(always)]
	fn consume_or_clone(self) -> Self::Target {
		return match self {
			RefToValue::RValue(v) => v,
			RefToValue::LValue(v) |
			RefToValue::ObjectProperty { snapshot: Some(v), .. } => v.deref().borrow().deref().clone(),
			RefToValue::ObjectProperty { snapshot: None, .. } => PrimitiveValue::Null,
		};
	}
}

impl RefToValue {
	#[inline(always)]
	pub fn new_variable(val: PrimitiveValue) -> RefToValue {
		return RefToValue::LValue(gc_box_from(val));
	}

	pub fn new_object_property_ref(object: GcBox<RuntimeObject>, property_name: IdentifierT) -> Self {
		let snapshot = object.borrow().get_actual(&property_name);
		RefToValue::ObjectProperty {
			object,
			property_name,
			snapshot,
		}
	}

	pub fn set(&mut self, value: PrimitiveValue) -> ResultWithError<Option<PrimitiveValue>> {
		return match self {
			RefToValue::RValue(_v) => Err(ErrorT::ExpectedLhsExpression.into()),
			RefToValue::LValue(v) => {
				Ok(Some(replace(v.borrow_mut().deref_mut(), value)))
			}
			RefToValue::ObjectProperty {
				object,
				property_name,
				snapshot: _
			} => {
				Ok(object.borrow().assign_locally(property_name, value))
			}
		};
	}

	#[inline(always)]
	pub fn borrow(&self) -> DerefOfRefToValue {
		return match self {
			RefToValue::RValue(v) => DerefOfRefToValue::DerefRValue(v),
			RefToValue::LValue(v) |
			RefToValue::ObjectProperty { snapshot: Some(v), .. } => {
				DerefOfRefToValue::DerefLValue(v.deref().borrow())
			}
			RefToValue::ObjectProperty { snapshot: None, .. } => DerefOfRefToValue::Value(PrimitiveValue::Null),
		};
	}

	delegate! {
		to match self {
			RefToValue::RValue(v) => v,
			RefToValue::LValue(v) |
			RefToValue::ObjectProperty{ snapshot: Some(v), .. } => {
				v.deref().borrow().deref()
			}
			RefToValue::ObjectProperty{ snapshot: None, .. } => DerefOfRefToValue::Value(PrimitiveValue::Null),
		} {
			pub fn is_truthy(&self) -> bool;
			pub fn is_hoisted(&self) -> bool;
			#[call(clone)]
			pub fn deref_clone(&self) -> PrimitiveValue;
		}
	}
}
