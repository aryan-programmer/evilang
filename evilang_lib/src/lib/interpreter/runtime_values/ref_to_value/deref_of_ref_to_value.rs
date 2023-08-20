use std::ops::Deref;

use gc::GcCellRef;

use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::types::traits::ConsumeOrCloneOf;

#[derive(Debug)]
pub enum DerefOfRefToValue<'a> {
	DerefRValue(&'a PrimitiveValue),
	DerefLValue(GcCellRef<'a, PrimitiveValue>),
	Value(PrimitiveValue),
}

impl<'a> ConsumeOrCloneOf for DerefOfRefToValue<'a> {
	type Target = PrimitiveValue;
	fn consume_or_clone(self) -> ResultWithError<PrimitiveValue> {
		return match self {
			DerefOfRefToValue::DerefRValue(v) => v.try_clone_err(),
			DerefOfRefToValue::DerefLValue(r) => r.deref().deref().try_clone_err(),
			DerefOfRefToValue::Value(cl) => Ok(cl),
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
