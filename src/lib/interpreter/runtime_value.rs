use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::errors::{ErrorT, ResultWithError};

pub type RcCellValue = Rc<RefCell<PrimitiveValue>>;

pub fn rc_cell_from(v: PrimitiveValue) -> RcCellValue {
	return RcCellValue::new(RefCell::new(v));
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PrimitiveValue {
	Null,
	Boolean(bool),
	Integer(i64),
	String(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum RefToValue {
	RValue(PrimitiveValue),
	LValue(RcCellValue),
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

	pub fn deref_clone(&self) -> PrimitiveValue {
		return match self {
			RefToValue::RValue(v) => v.clone(),
			RefToValue::LValue(v) => v.deref().borrow().deref().clone(),
		};
	}

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
}
