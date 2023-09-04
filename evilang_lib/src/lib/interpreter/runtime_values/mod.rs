use std::fmt::{Display, Formatter, Write};
use std::mem::swap;
use std::ops::Deref;

use gc::{Finalize, Trace};
use itertools::{Either, Either::Left, Either::Right};
use num_traits::Zero;

use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::functions::{Function, GcPtrToFunction};
use crate::interpreter::runtime_values::functions::native_function::{NativeFunction, NativeFunctionFn};
use crate::interpreter::runtime_values::i_native_struct::GcPtrToNativeStruct;
use crate::interpreter::runtime_values::objects::runtime_object::GcPtrToObject;
use crate::types::cell_ref::{GcPtr, GcPtrCell};
use crate::types::number::NumberT;
use crate::types::string::StringT;

pub mod ref_to_value;
pub mod functions;
pub mod objects;
pub mod i_native_struct;

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

#[derive(Debug, Clone, Trace, Finalize)]
pub enum PrimitiveValue {
	_HoistedVariable,
	Null,
	Boolean(bool),
	Number(#[unsafe_ignore_trace] NumberT),
	String(StringT),
	Function(GcPtrToFunction),
	Object(GcPtrToObject),
	NativeStruct(GcPtrToNativeStruct),
}

impl Display for PrimitiveValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			PrimitiveValue::_HoistedVariable => {f.write_str("<hoisted_variable>")}
			PrimitiveValue::Null => {f.write_str("null")}
			PrimitiveValue::Boolean(b) => {std::fmt::Display::fmt(b, f)}
			PrimitiveValue::Number(n) => {std::fmt::Display::fmt(n, f)}
			PrimitiveValue::String(s) => {std::fmt::Display::fmt(s, f)}
			PrimitiveValue::Function(fnc) => {std::fmt::Display::fmt(fnc.deref().deref(), f)}
			PrimitiveValue::Object(o) => {std::fmt::Display::fmt(&o.name, f)}
			PrimitiveValue::NativeStruct(ns) => {f.write_str("<native_struct>")}
		}
	}
}

impl<T: Into<PrimitiveValue>> From<Option<T>> for PrimitiveValue {
	fn from(value: Option<T>) -> Self {
		match value {
			None => PrimitiveValue::Null,
			Some(v) => v.into(),
		}
	}
}

impl From<bool> for PrimitiveValue {
	fn from(value: bool) -> Self {
		PrimitiveValue::Boolean(value)
	}
}

impl From<NumberT> for PrimitiveValue {
	fn from(value: NumberT) -> Self {
		PrimitiveValue::Number(value)
	}
}

impl From<StringT> for PrimitiveValue {
	fn from(value: StringT) -> Self {
		PrimitiveValue::String(value)
	}
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
			(
				PrimitiveValue::NativeStruct(self_0),
				PrimitiveValue::NativeStruct(othr_0),
			) => GcPtr::ptr_eq(self_0, othr_0),
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

	pub fn new_native_function(f: NativeFunctionFn) -> Self {
		return PrimitiveValue::Function(GcPtr::new(Function::NativeFunction(NativeFunction::new(f))));
	}

	pub fn is_truthy(&self) -> bool {
		return match self {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => false,
			PrimitiveValue::Boolean(v) => *v,
			PrimitiveValue::Number(i) => !i.is_zero(),
			PrimitiveValue::String(s) => s.len() != 0,
			PrimitiveValue::Function(..) | PrimitiveValue::Object(..) | PrimitiveValue::NativeStruct(..) => true,
		};
	}

	#[inline(always)]
	pub fn is_hoisted(&self) -> bool {
		return self == &PrimitiveValue::_HoistedVariable;
	}

	#[inline(always)]
	pub fn try_clone_err(&self) -> ResultWithError<PrimitiveValue> {
		return Ok(self.clone());
		// return Ok(self
		// 	.try_clone()
		// 	.map_err(|_e| EvilangError::new(RuntimeError::CantCloneSafely(Descriptor::Value(self.clone__silently_fail())).into()))?);
	}

	#[inline(always)]
	pub fn consume_as_string(mut self) -> Either<StringT, PrimitiveValue> {
		match self {
			PrimitiveValue::String(ref mut v) => {
				let mut str = String::new();
				swap(v, &mut str);
				Left(str)
			}
			v => Right(v),
		}
	}

	#[allow(non_snake_case)]
	#[inline(always)]
	pub fn clone__silently_fail(&self) -> Self {
		return self.clone();
	}
}
