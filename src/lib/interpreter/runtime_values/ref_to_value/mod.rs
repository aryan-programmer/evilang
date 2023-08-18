use std::mem::replace;
use std::ops::{Deref, DerefMut};

use delegate::delegate;

use crate::ast::expression::IdentifierT;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_values::{GcPtrVariable, PrimitiveValue};
use crate::interpreter::runtime_values::objects::runtime_object::GcPtrToObject;
pub use crate::interpreter::runtime_values::ref_to_value::deref_of_ref_to_value::DerefOfRefToValue;
use crate::interpreter::utils::cell_ref::gc_ptr_cell_from;
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator};

pub mod deref_of_ref_to_value;

#[derive(Debug, PartialEq)]
pub enum RefToValue {
	Value(PrimitiveValue),
	Variable(GcPtrVariable),
	ObjectProperty {
		object: GcPtrToObject,
		property_name: IdentifierT,
		snapshot: Option<GcPtrVariable>,
	},
}

impl From<PrimitiveValue> for RefToValue {
	#[inline(always)]
	fn from(value: PrimitiveValue) -> Self {
		return RefToValue::Value(value);
	}
}

impl ConsumeOrCloneOf for RefToValue {
	type Target = PrimitiveValue;

	#[inline(always)]
	fn consume_or_clone(self) -> Self::Target {
		/*
		return match self {
			RefToValue::Value(v) |
			RefToValue::ObjectProperty { snapshot: v, .. } => v,
			RefToValue::Variable(v) => v.borrow().clone(),
		};
		*/
		return match self {
			RefToValue::Value(v) => v,
			RefToValue::ObjectProperty { snapshot: None, .. } => PrimitiveValue::Null,
			RefToValue::ObjectProperty { snapshot: Some(v), .. } |
			RefToValue::Variable(v) => v.borrow().clone(),
		};
	}
}

impl RefToValue {
	#[inline(always)]
	pub fn new_variable(val: PrimitiveValue) -> RefToValue {
		return RefToValue::Variable(gc_ptr_cell_from(val));
	}

	pub fn new_object_property_ref(object: GcPtrToObject, property_name: IdentifierT) -> Self {
		let snapshot = object.get_actual(property_name.deref().into());
		RefToValue::ObjectProperty {
			object,
			property_name,
			snapshot,
		}
	}

	pub fn set(&mut self, value: PrimitiveValue) -> ResultWithError<Option<PrimitiveValue>> {
		return match self {
			RefToValue::Value(_v) => Err(ErrorT::ExpectedLhsExpression.into()),
			RefToValue::Variable(v) => {
				Ok(Some(replace(v.borrow_mut().deref_mut(), value)))
			}
			RefToValue::ObjectProperty {
				object,
				property_name,
				snapshot: _
			} => {
				Ok(object.assign_locally(property_name.as_str().into(), value))
			}
		};
	}

	#[inline(always)]
	pub fn borrow(&self) -> DerefOfRefToValue {
		/*
		return match self {
			RefToValue::Value(v) |
			RefToValue::ObjectProperty { snapshot: v, .. } => DerefOfRefToValue::DerefRValue(v),
			RefToValue::Variable(v) => {
				DerefOfRefToValue::DerefLValue(v.borrow())
			}
		};
		*/
		return match self {
			RefToValue::Value(v) => DerefOfRefToValue::DerefRValue(v),
			RefToValue::ObjectProperty { snapshot: None, .. } => DerefOfRefToValue::Value(PrimitiveValue::Null),
			RefToValue::ObjectProperty { snapshot: Some(v), .. } |
			RefToValue::Variable(v) => {
				DerefOfRefToValue::DerefLValue(v.borrow())
			}
		};
	}

	delegate! {
		/*
		to match self {
			RefToValue::Value(v) |
			RefToValue::ObjectProperty{ snapshot: v, .. } => v,
			RefToValue::Variable(v) => {
				v.deref().borrow().deref()
			}
		}
		*/

		to match self {
			RefToValue::Value(v) => v,
			RefToValue::ObjectProperty { snapshot: None, .. } => PrimitiveValue::Null,
			RefToValue::ObjectProperty { snapshot: Some(v), .. } |
			RefToValue::Variable(v) => v.borrow(),
		} {
			pub fn is_truthy(&self) -> bool;
			pub fn is_hoisted(&self) -> bool;
		}
	}
}
