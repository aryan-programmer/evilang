use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use delegate::delegate;

use crate::errors::{ErrorT, ResultWithError};
use crate::utils::cell_ref::rc_cell_from;

pub type RcCellValue = Rc<RefCell<PrimitiveValue>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PrimitiveValue {
	Null,
	Boolean(bool),
	Integer(i64),
	String(String),
}

impl PrimitiveValue {
	pub fn is_truthy(&self) -> bool {
		return match self {
			PrimitiveValue::Null => false,
			PrimitiveValue::Boolean(v) => *v,
			PrimitiveValue::Integer(i) => *i != 0,
			PrimitiveValue::String(s) => s.len() != 0,
		};
	}
}

#[derive(Debug)]
pub enum DerefOfRefToValue<'a> {
	DerefRValue(&'a PrimitiveValue),
	DerefLValue(Ref<'a, PrimitiveValue>),
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
	LValue(RcCellValue),
}

impl From<PrimitiveValue> for RefToValue {
	fn from(value: PrimitiveValue) -> Self {
		return RefToValue::RValue(value);
	}
}

impl Clone for RefToValue {
	fn clone(&self) -> Self {
		return match self {
			RefToValue::RValue(v) => RefToValue::RValue(v.clone()),
			RefToValue::LValue(v) => RefToValue::LValue(RcCellValue::clone(v)),
		};
	}
}

impl RefToValue {
	pub fn from_rc(val: &RcCellValue) -> RefToValue {
		return RefToValue::LValue(RcCellValue::clone(val));
	}

	pub fn new_lvalue(val: PrimitiveValue) -> RefToValue {
		return RefToValue::LValue(rc_cell_from(val));
	}

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

	#[inline]
	pub fn consume_or_clone(self) -> PrimitiveValue {
		return match self {
			RefToValue::RValue(v) => v,
			RefToValue::LValue(v) => v.deref().borrow().deref().clone(),
		};
	}

	pub fn try_borrow_mut(&self) -> ResultWithError<RefMut<PrimitiveValue>> {
		return match self {
			RefToValue::RValue(_) => Err(ErrorT::InvalidMutableBorrowForRValue.into()),
			RefToValue::LValue(v) => Ok(v.deref().borrow_mut()),
		};
	}

	delegate! {
		to match self {
			RefToValue::RValue(v) => v,
			RefToValue::LValue(v) => v.deref().borrow().deref(),
		} {
			pub fn is_truthy(&self) -> bool;
			#[call(clone)]
			pub fn deref_clone(&self) -> PrimitiveValue;
		}
	}
}
