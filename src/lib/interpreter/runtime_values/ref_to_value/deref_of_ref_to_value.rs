use std::ops::Deref;

use gc::GcCellRef;

use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;

#[derive(Debug)]
pub enum DerefOfRefToValue<'a> {
	DerefRValue(&'a PrimitiveValue),
	DerefLValue(GcCellRef<'a, PrimitiveValue>),
	Value(PrimitiveValue),
}

impl<'a> ConsumeOrCloneOf for  DerefOfRefToValue<'a> {
	type Target = PrimitiveValue;
	fn consume_or_clone(self) -> PrimitiveValue {
		return match self {
			DerefOfRefToValue::DerefRValue(v) => v.clone(),
			DerefOfRefToValue::DerefLValue(r) => r.deref().clone(),
			DerefOfRefToValue::Value(cl) => cl,
		};
	}
}

impl<'a> Deref for DerefOfRefToValue<'a> {
	type Target = PrimitiveValue;

	fn deref(&self) -> &Self::Target {
		return match self {
			DerefOfRefToValue::DerefRValue(v) => *v,
			DerefOfRefToValue::DerefLValue(r) => r.deref(),
			DerefOfRefToValue::Value(cl) => cl,
		};
	}
}
