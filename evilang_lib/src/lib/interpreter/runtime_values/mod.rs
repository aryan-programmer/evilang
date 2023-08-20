use std::ops::Deref;

use gc::{Finalize, Trace};
use num_traits::Zero;

use evilang_traits::{Clone__SilentlyFail, TryClone};

use crate::errors::{Descriptor, EvilangError, ResultWithError, RuntimeError};
use crate::interpreter::runtime_values::functions::{Function, GcPtrToFunction};
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
// use crate::interpreter::runtime_values::native_struct::RcToGcCellOfNativeStruct;
use crate::interpreter::runtime_values::objects::runtime_object::GcPtrToObject;
use crate::interpreter::utils::cell_ref::{GcPtr, GcPtrCell};
use crate::types::number::NumberT;
use crate::types::string::StringT;

pub mod ref_to_value;
pub mod functions;
pub mod objects;
// pub mod native_struct;

pub type GcPtrVariable = GcPtr<GcPtrCell<PrimitiveValue>>;

pub trait GcPtrVariableExt {
	fn is_hoisted(&self) -> bool;
}

impl GcPtrVariableExt for GcPtrVariable {
	#[inline(always)]
	fn is_hoisted(&self) -> bool {
		return self.deref().borrow().deref().is_hoisted();
	}
}

#[derive(Debug, TryClone, Clone__SilentlyFail, Trace, Finalize)]
pub enum PrimitiveValue {
	_HoistedVariable,
	Null,
	Boolean(bool),
	Number(#[unsafe_ignore_trace] NumberT),
	String(StringT),
	Function(GcPtrToFunction),
	Object(GcPtrToObject),
	//NativeStruct(RcToGcCellOfNativeStruct),
}

impl From<GcPtrToObject> for PrimitiveValue {
	fn from(value: GcPtrToObject) -> Self {
		PrimitiveValue::Object(value)
	}
}

impl From<GcPtrToFunction> for PrimitiveValue {
	fn from(value: GcPtrToFunction) -> Self {
		PrimitiveValue::Function(value)
	}
}

impl PartialEq for PrimitiveValue {
	#[inline]
	fn eq(&self, other: &PrimitiveValue) -> bool {
		match (self, other) {
			(
				PrimitiveValue::Boolean(self_0),
				PrimitiveValue::Boolean(othr_0),
			) => *self_0 == *othr_0,
			(
				PrimitiveValue::Number(self_0),
				PrimitiveValue::Number(othr_0),
			) => *self_0 == *othr_0,
			(
				PrimitiveValue::String(self_0),
				PrimitiveValue::String(othr_0),
			) => *self_0 == *othr_0,
			(
				PrimitiveValue::Function(self_0),
				PrimitiveValue::Function(othr_0),
			) => GcPtr::ptr_eq(self_0, othr_0),
			(
				PrimitiveValue::Object(self_0),
				PrimitiveValue::Object(othr_0),
			) => GcPtr::ptr_eq(self_0, othr_0),
			// (
			// 	PrimitiveValue::NativeStruct(self_0),
			// 	PrimitiveValue::NativeStruct(othr_0),
			// ) => self_0 == othr_0,
			(PrimitiveValue::Null, PrimitiveValue::Null) => true,
			(PrimitiveValue::_HoistedVariable, PrimitiveValue::_HoistedVariable) => true,
			_ => false,
		}
	}
}

impl PrimitiveValue {
	pub fn float(v: f64) -> Self {
		PrimitiveValue::Number(NumberT::Float(v))
	}

	pub fn integer(v: i64) -> Self {
		PrimitiveValue::Number(NumberT::Integer(v as i128))
	}

	pub fn new_native_function(f: NativeFunction) -> Self {
		return PrimitiveValue::Function(GcPtr::new(Function::NativeFunction(f)));
	}

	pub fn is_truthy(&self) -> bool {
		return match self {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => false,
			PrimitiveValue::Boolean(v) => *v,
			PrimitiveValue::Number(i) => !i.is_zero(),
			PrimitiveValue::String(s) => s.len() != 0,
			PrimitiveValue::Function(..) | PrimitiveValue::Object(..) /*| PrimitiveValue::NativeStruct(..)*/ => true,
		};
	}

	#[inline(always)]
	pub fn is_hoisted(&self) -> bool {
		return self == &PrimitiveValue::_HoistedVariable;
	}

	#[inline(always)]
	pub fn try_clone_err(&self) -> ResultWithError<PrimitiveValue> {
		return Ok(self
			.try_clone()
			.map_err(|_e| EvilangError::new(RuntimeError::CantCloneSafely(Descriptor::Value(self.clone__silently_fail())).into()))?);
	}
}
