use std::ops::Deref;
use either::Either;
use crate::ast::expression::IdentifierT;
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::utils::cell_ref::GcBox;

#[derive(Debug, PartialEq)]
pub struct RefToObjectProperty{
	pub object: GcBox<RuntimeObject>,
	pub property_name: IdentifierT,
	pub snapshot: Option<GcBox<PrimitiveValue>>
}

impl ConsumeOrCloneOf for RefToObjectProperty {
	type Target = PrimitiveValue;

	fn consume_or_clone(self) -> Self::Target {
		return match self.snapshot {
			None => PrimitiveValue::Null,
			Some(v) => v.borrow().deref().clone(),
		};
	}
}

impl RefToObjectProperty {
	pub fn new(object: GcBox<RuntimeObject>, property_name: IdentifierT) -> Self {
		let snapshot = object.borrow().get_actual(&property_name);
		Self {
			object,
			property_name,
			snapshot,
		}
	}

	pub fn set(&mut self, value: PrimitiveValue) -> Option<PrimitiveValue> {
		self.object.borrow().assign_locally(&self.property_name, value)
	}

	pub fn get_primitive(&self) -> PrimitiveValue {
		return match &self.snapshot {
			None => PrimitiveValue::Null,
			Some(v) => v.borrow().deref().clone(),
		};
	}
}
