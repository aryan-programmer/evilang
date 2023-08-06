use std::ops::Deref;

use delegate::delegate;
use gc::{Finalize, GcCellRef, GcCellRefMut, Trace};

use crate::errors::{ErrorT, ResultWithError};
use crate::utils::cell_ref::{gc_box_from, gc_cell_clone, GcBox};

pub trait GcBoxOfPrimitiveValueExt {
	fn is_hoisted(&self) -> bool;
}

impl GcBoxOfPrimitiveValueExt for GcBox<PrimitiveValue> {
	#[inline(always)]
	fn is_hoisted(&self) -> bool {
		return self.deref().borrow().deref().is_hoisted();
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Trace, Finalize)]
pub enum PrimitiveValue {
	_HoistedVariable,
	Null,
	Boolean(bool),
	Integer(i64),
	String(String),
}

impl PrimitiveValue {
	pub fn is_truthy(&self) -> bool {
		return match self {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => false,
			PrimitiveValue::Boolean(v) => *v,
			PrimitiveValue::Integer(i) => *i != 0,
			PrimitiveValue::String(s) => s.len() != 0,
		};
	}

	#[inline(always)]
	pub fn is_hoisted(&self) -> bool {
		return self == &PrimitiveValue::_HoistedVariable;
	}
}

#[derive(Debug)]
pub enum DerefOfRefToValue<'a> {
	DerefRValue(&'a PrimitiveValue),
	DerefLValue(GcCellRef<'a, PrimitiveValue>),
	Value(PrimitiveValue),
}

impl<'a> DerefOfRefToValue<'a> {
	pub fn consume_or_clone(self) -> PrimitiveValue {
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

#[derive(Debug, Eq, PartialEq)]
pub enum RefToValue {
	RValue(PrimitiveValue),
	LValue(GcBox<PrimitiveValue>),
}

impl From<PrimitiveValue> for RefToValue {
	#[inline(always)]
	fn from(value: PrimitiveValue) -> Self {
		return RefToValue::RValue(value);
	}
}

impl Clone for RefToValue {
	#[inline(always)]
	fn clone(&self) -> Self {
		return match self {
			RefToValue::RValue(v) => RefToValue::RValue(v.clone()),
			RefToValue::LValue(v) => RefToValue::LValue(gc_cell_clone(v)),
		};
	}
}

impl RefToValue {
	#[inline(always)]
	pub fn new_variable(val: PrimitiveValue) -> RefToValue {
		return RefToValue::LValue(gc_box_from(val));
	}

	#[inline(always)]
	pub fn borrow(&self) -> DerefOfRefToValue {
		return match self {
			RefToValue::RValue(v) => DerefOfRefToValue::DerefRValue(v),
			RefToValue::LValue(v) => DerefOfRefToValue::DerefLValue(v.deref().borrow()),
		};
	}

	pub fn try_borrow(&self) -> ResultWithError<DerefOfRefToValue> {
		return match self {
			RefToValue::RValue(v) => Ok(DerefOfRefToValue::DerefRValue(v)),
			RefToValue::LValue(v) => {
				if let Ok(borrow) = v.deref().try_borrow() {
					Ok(DerefOfRefToValue::DerefLValue(borrow))
				} else {
					Err(ErrorT::InvalidBorrow.into())
				}
			}
		};
	}

	#[inline(always)]
	pub fn consume_or_clone(self) -> PrimitiveValue {
		return match self {
			RefToValue::RValue(v) => v,
			RefToValue::LValue(v) => v.deref().borrow().deref().clone(),
		};
	}

	#[inline(always)]
	pub fn try_borrow_mut(&self) -> ResultWithError<GcCellRefMut<PrimitiveValue>> {
		return match self {
			RefToValue::RValue(_) => Err(ErrorT::ExpectedLhsExpression.into()),
			RefToValue::LValue(v) => Ok(v.deref().borrow_mut()),
		};
	}

	delegate! {
		to match self {
			RefToValue::RValue(v) => v,
			RefToValue::LValue(v) => v.deref().borrow().deref(),
		} {
			pub fn is_truthy(&self) -> bool;
			pub fn is_hoisted(&self) -> bool;
			#[call(clone)]
			pub fn deref_clone(&self) -> PrimitiveValue;
		}
	}
}
